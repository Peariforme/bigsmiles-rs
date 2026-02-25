//! Integration tests — structural parsing assertions.
//!
//! Each test verifies not just the parse succeeds, but also the exact shape of the
//! resulting AST: number of segments, BD kinds/indices, atom counts, bond counts, etc.

use bigsmiles::{
    opensmiles::{AtomSymbol, BondType, OrganicAtom},
    parse, BigSmilesSegment, BondDescriptor, BondDescriptorKind, StochasticObject,
};
use pretty_assertions::assert_eq;

// ── Helper constructors ───────────────────────────────────────────────────────

fn bd(kind: BondDescriptorKind) -> BondDescriptor {
    BondDescriptor { kind, index: None }
}
fn bd_indexed(kind: BondDescriptorKind, index: u32) -> BondDescriptor {
    BondDescriptor {
        kind,
        index: Some(index),
    }
}

/// Assert there is exactly one segment, that it is `Stochastic`, and return it.
fn only_stochastic(input: &str) -> StochasticObject {
    let result = parse(input).unwrap_or_else(|e| panic!("parse({input:?}) failed: {e}"));
    assert_eq!(
        result.segments.len(),
        1,
        "expected a single segment for {input:?}"
    );
    match result.segments.into_iter().next().unwrap() {
        BigSmilesSegment::Stochastic(obj) => obj,
        other => panic!("expected Stochastic, got {:?}", other),
    }
}

// ── NoBond descriptors ────────────────────────────────────────────────────────

#[test]
fn polyethylene_no_bond_descriptors() {
    // {[]CC[]} — polyethylene with terminal (no-bond) descriptors
    let obj = only_stochastic("{[]CC[]}");

    assert_eq!(obj.left_end, None, "no outer terminal on the left");
    assert_eq!(obj.right_end, None, "no outer terminal on the right");
    assert_eq!(obj.repeat_units.len(), 1);
    assert_eq!(obj.end_groups.len(), 0);

    let ru = &obj.repeat_units[0];
    assert_eq!(ru.left, bd(BondDescriptorKind::NoBond));
    assert_eq!(ru.right, bd(BondDescriptorKind::NoBond));

    // Molecule: C–C (ethylene unit, 2 atoms, 1 single bond)
    assert_eq!(ru.molecule.nodes().len(), 2);
    assert_eq!(ru.molecule.bonds().len(), 1);
    assert_eq!(
        ru.molecule.nodes()[0].atom().element(),
        &AtomSymbol::Organic(OrganicAtom::C)
    );
    assert_eq!(ru.molecule.bonds()[0].kind(), BondType::Simple);

    // Connection atoms: left BD → atom 0 (first C), right BD → atom 1 (last C)
    assert_eq!(ru.left_atom, 0);
    assert_eq!(ru.right_atom, 1);
}

#[test]
fn polypropylene_no_bond_descriptors() {
    // {[]CC(C)[]} — polypropylene: main chain through C0–C1, methyl branch at C1
    let obj = only_stochastic("{[]CC(C)[]}");

    assert_eq!(obj.repeat_units.len(), 1);
    let ru = &obj.repeat_units[0];
    assert_eq!(ru.left, bd(BondDescriptorKind::NoBond));
    assert_eq!(ru.right, bd(BondDescriptorKind::NoBond));

    // CC(C): 3 carbons, 2 single bonds
    assert_eq!(ru.molecule.nodes().len(), 3);
    assert_eq!(ru.molecule.bonds().len(), 2);
    assert!(ru
        .molecule
        .nodes()
        .iter()
        .all(|n| n.atom().element() == &AtomSymbol::Organic(OrganicAtom::C)));

    // left BD → C0 (first written), right BD → C1 (last main-chain atom, not the methyl C2)
    assert_eq!(ru.left_atom, 0);
    assert_eq!(
        ru.right_atom, 1,
        "right BD connects to C1 (backbone), not C2 (branch methyl)"
    );
}

#[test]
fn polystyrene_no_bond_descriptors() {
    // {[]CC(c1ccccc1)[]} — styrene repeat unit: 2 aliphatic + 6 aromatic C
    let obj = only_stochastic("{[]CC(c1ccccc1)[]}");

    assert_eq!(obj.repeat_units.len(), 1);
    let ru = &obj.repeat_units[0];

    // 8 heavy atoms (2 aliphatic C + 6 aromatic c)
    assert_eq!(ru.molecule.nodes().len(), 8);
    // 1 (C–C) + 1 (C–ring) + 6 (ring: 5 bonds + 1 closure) = 8 bonds
    assert_eq!(ru.molecule.bonds().len(), 8);

    // Backbone: C0 (left) — C1 (right), phenyl ring is a branch at C1
    assert_eq!(ru.left_atom, 0);
    assert_eq!(
        ru.right_atom, 1,
        "right BD connects to C1 (backbone), not the phenyl ring atoms"
    );
}

// ── NonDirectional descriptors ────────────────────────────────────────────────

#[test]
fn polyethylene_non_directional() {
    let obj = only_stochastic("{[$]CC[$]}");

    assert_eq!(obj.left_end, None);
    assert_eq!(obj.right_end, None);
    assert_eq!(obj.repeat_units.len(), 1);

    let ru = &obj.repeat_units[0];
    assert_eq!(ru.left, bd(BondDescriptorKind::NonDirectional));
    assert_eq!(ru.right, bd(BondDescriptorKind::NonDirectional));

    assert_eq!(ru.molecule.nodes().len(), 2);
    assert_eq!(ru.molecule.bonds().len(), 1);
    assert_eq!(ru.molecule.bonds()[0].kind(), BondType::Simple);
}

// ── Indexed descriptors ───────────────────────────────────────────────────────

#[test]
fn non_directional_indexed() {
    let obj = only_stochastic("{[$1]CC[$1]}");
    let ru = &obj.repeat_units[0];
    assert_eq!(ru.left, bd_indexed(BondDescriptorKind::NonDirectional, 1));
    assert_eq!(ru.right, bd_indexed(BondDescriptorKind::NonDirectional, 1));
}

#[test]
fn directional_indexed() {
    let obj = only_stochastic("{[<2]CC[>2]}");
    let ru = &obj.repeat_units[0];
    assert_eq!(ru.left, bd_indexed(BondDescriptorKind::Head, 2));
    assert_eq!(ru.right, bd_indexed(BondDescriptorKind::Tail, 2));
}

// ── Directed descriptors with outer terminals ─────────────────────────────────

#[test]
fn isotactic_polypropylene_with_terminals() {
    // {[>][<]CC(C)[>][<]}
    // left_end=[>]=Tail, ru=[<]CC(C)[>]=Head/Tail, right_end=[<]=Head
    let obj = only_stochastic("{[>][<]CC(C)[>][<]}");

    assert_eq!(
        obj.left_end,
        Some(bd(BondDescriptorKind::Tail)),
        "outer left is Tail [>]"
    );
    assert_eq!(
        obj.right_end,
        Some(bd(BondDescriptorKind::Head)),
        "outer right is Head [<]"
    );
    assert_eq!(obj.repeat_units.len(), 1);
    assert_eq!(obj.end_groups.len(), 0);

    let ru = &obj.repeat_units[0];
    assert_eq!(ru.left, bd(BondDescriptorKind::Head), "ru left is Head [<]");
    assert_eq!(
        ru.right,
        bd(BondDescriptorKind::Tail),
        "ru right is Tail [>]"
    );

    // CC(C) = propane: 3 carbons, 2 single bonds
    assert_eq!(ru.molecule.nodes().len(), 3);
    assert_eq!(ru.molecule.bonds().len(), 2);
    assert!(ru
        .molecule
        .bonds()
        .iter()
        .all(|b| b.kind() == BondType::Simple));
}

// ── Copolymers (multiple repeat units) ───────────────────────────────────────

#[test]
fn ethylene_propylene_copolymer() {
    let obj = only_stochastic("{[$]CC[$],[$]CC(C)[$]}");

    assert_eq!(obj.left_end, None);
    assert_eq!(obj.right_end, None);
    assert_eq!(obj.repeat_units.len(), 2);
    assert_eq!(obj.end_groups.len(), 0);

    // First unit: CC (ethylene) — 2 atoms, 1 bond
    let ru0 = &obj.repeat_units[0];
    assert_eq!(ru0.left, bd(BondDescriptorKind::NonDirectional));
    assert_eq!(ru0.right, bd(BondDescriptorKind::NonDirectional));
    assert_eq!(ru0.molecule.nodes().len(), 2);
    assert_eq!(ru0.molecule.bonds().len(), 1);

    // Second unit: CC(C) = propane — 3 atoms, 2 bonds
    let ru1 = &obj.repeat_units[1];
    assert_eq!(ru1.left, bd(BondDescriptorKind::NonDirectional));
    assert_eq!(ru1.right, bd(BondDescriptorKind::NonDirectional));
    assert_eq!(ru1.molecule.nodes().len(), 3);
    assert_eq!(ru1.molecule.bonds().len(), 2);
}

#[test]
fn three_unit_copolymer() {
    let obj = only_stochastic("{[$]CC[$],[$]CCC[$],[$]CCCC[$]}");

    assert_eq!(obj.repeat_units.len(), 3);
    assert_eq!(obj.repeat_units[0].molecule.nodes().len(), 2); // ethylene
    assert_eq!(obj.repeat_units[1].molecule.nodes().len(), 3); // propane
    assert_eq!(obj.repeat_units[2].molecule.nodes().len(), 4); // butane
}

// ── End groups (after `;`) ────────────────────────────────────────────────────

#[test]
fn stochastic_with_end_groups() {
    // {[$]CC[$];[$]CCO[$]} — one repeat unit, one end group
    let obj = only_stochastic("{[$]CC[$];[$]CCO[$]}");

    assert_eq!(obj.repeat_units.len(), 1);
    assert_eq!(obj.end_groups.len(), 1);

    let ru = &obj.repeat_units[0];
    assert_eq!(ru.left, bd(BondDescriptorKind::NonDirectional));
    assert_eq!(ru.molecule.nodes().len(), 2); // CC
    assert_eq!(ru.left_atom, 0);
    assert_eq!(ru.right_atom, 1);

    let eg = &obj.end_groups[0];
    assert_eq!(eg.left, bd(BondDescriptorKind::NonDirectional));
    // CCO: 3 atoms (C, C, O), 2 bonds
    assert_eq!(eg.molecule.nodes().len(), 3);
    assert_eq!(eg.molecule.bonds().len(), 2);
    // One node must be oxygen
    assert!(eg
        .molecule
        .nodes()
        .iter()
        .any(|n| n.atom().element() == &AtomSymbol::Organic(OrganicAtom::O)));
    // right BD connects to the hydroxyl O (last main-chain atom, index 2)
    assert_eq!(eg.left_atom, 0);
    assert_eq!(eg.right_atom, 2, "right BD bonds to the O in CCO");
}

#[test]
fn stochastic_multiple_end_groups() {
    // {[$]CC[$];[$]CO[$],[$]CCO[$]}
    let obj = only_stochastic("{[$]CC[$];[$]CO[$],[$]CCO[$]}");
    assert_eq!(obj.repeat_units.len(), 1);
    assert_eq!(obj.end_groups.len(), 2);
}

// ── Surrounding SMILES fragments ──────────────────────────────────────────────

#[test]
fn alpha_omega_dimethyl_polyethylene() {
    let result = parse("CC{[$]CC[$]}CC").unwrap();

    assert_eq!(result.segments.len(), 3);

    // Left SMILES: CC — 2 carbons
    match &result.segments[0] {
        BigSmilesSegment::Smiles(mol) => {
            assert_eq!(mol.nodes().len(), 2);
            assert_eq!(mol.bonds().len(), 1);
        }
        other => panic!("expected Smiles, got {:?}", other),
    }

    // Stochastic object
    match &result.segments[1] {
        BigSmilesSegment::Stochastic(obj) => {
            assert_eq!(obj.repeat_units.len(), 1);
            assert_eq!(obj.repeat_units[0].molecule.nodes().len(), 2);
        }
        other => panic!("expected Stochastic, got {:?}", other),
    }

    // Right SMILES: CC — 2 carbons
    match &result.segments[2] {
        BigSmilesSegment::Smiles(mol) => {
            assert_eq!(mol.nodes().len(), 2);
        }
        other => panic!("expected Smiles, got {:?}", other),
    }
}

#[test]
fn plain_smiles_no_stochastic() {
    let result = parse("CCO").unwrap();

    assert_eq!(result.segments.len(), 1);
    match &result.segments[0] {
        BigSmilesSegment::Smiles(mol) => {
            assert_eq!(mol.nodes().len(), 3); // C, C, O
            assert_eq!(mol.bonds().len(), 2);
            assert!(mol
                .nodes()
                .iter()
                .any(|n| n.atom().element() == &AtomSymbol::Organic(OrganicAtom::O)));
        }
        other => panic!("expected Smiles, got {:?}", other),
    }
}

#[test]
fn smiles_before_and_after_stochastic() {
    let result = parse("c1ccccc1{[$]CC[$]}c1ccccc1").unwrap();
    assert_eq!(result.segments.len(), 3);
    assert!(matches!(result.segments[0], BigSmilesSegment::Smiles(_)));
    assert!(matches!(
        result.segments[1],
        BigSmilesSegment::Stochastic(_)
    ));
    assert!(matches!(result.segments[2], BigSmilesSegment::Smiles(_)));
}

#[test]
fn multiple_stochastic_objects() {
    // Two stochastic objects in sequence
    let result = parse("{[$]CC[$]}{[$]CCC[$]}").unwrap();
    assert_eq!(result.segments.len(), 2);
    assert!(matches!(
        result.segments[0],
        BigSmilesSegment::Stochastic(_)
    ));
    assert!(matches!(
        result.segments[1],
        BigSmilesSegment::Stochastic(_)
    ));
}

// ── Bracket atoms inside SMILES (must not be confused with BDs) ───────────────

#[test]
fn bracket_atom_charged_in_repeat_unit() {
    // [N+] must not be parsed as a bond descriptor
    let obj = only_stochastic("{[$]CC[N+][$]}");

    assert_eq!(obj.repeat_units.len(), 1);
    let ru = &obj.repeat_units[0];

    // C, C, N — three atoms
    assert_eq!(ru.molecule.nodes().len(), 3);
    assert_eq!(ru.molecule.bonds().len(), 2);

    // The nitrogen has charge +1
    let n_node = ru
        .molecule
        .nodes()
        .iter()
        .find(|n| n.atom().element() == &AtomSymbol::Organic(OrganicAtom::N))
        .expect("no nitrogen found");
    assert_eq!(n_node.atom().charge(), 1);
}

#[test]
fn bracket_atom_isotope_in_repeat_unit() {
    // [13C] should be preserved as a bracket atom
    let obj = only_stochastic("{[$][13C]C[$]}");

    let ru = &obj.repeat_units[0];
    assert_eq!(ru.molecule.nodes().len(), 2);

    let c13 = ru
        .molecule
        .nodes()
        .iter()
        .find(|n| n.atom().isotope() == Some(13))
        .expect("no 13C found");
    assert_eq!(c13.atom().isotope(), Some(13));
}

#[test]
fn aromatic_bracket_atom_in_repeat_unit() {
    // [se] and [as] are aromatic bracket atoms per OpenSMILES
    let obj = only_stochastic("{[$]c1cc[se]c1[$]}");
    let ru = &obj.repeat_units[0];
    assert_eq!(ru.molecule.nodes().len(), 5); // c, c, c, [se], c
}

// ── Double-bond repeat unit ───────────────────────────────────────────────────

#[test]
fn repeat_unit_with_double_bond() {
    let obj = only_stochastic("{[$]C=C[$]}");
    let ru = &obj.repeat_units[0];
    assert_eq!(ru.molecule.nodes().len(), 2);
    assert_eq!(ru.molecule.bonds().len(), 1);
    assert_eq!(ru.molecule.bonds()[0].kind(), BondType::Double);
    assert_eq!(ru.left_atom, 0);
    assert_eq!(ru.right_atom, 1);
}
