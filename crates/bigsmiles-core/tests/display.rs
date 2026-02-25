//! Integration tests — Display (faithful round-trip).
//!
//! SMILES inside stochastic objects are displayed verbatim (raw string),
//! because the atom ordering carries semantic meaning: the left bond descriptor
//! connects to the *first written atom* and the right bond descriptor to the
//! *last written atom*.  Reordering (e.g. `CC(C)` → `CCC`) would silently
//! change the polymer topology.
//!
//! Outer SMILES segments (no stochastic object) are still canonicalised by
//! the underlying `opensmiles` parser:
//!   `CCO` → `OCC`  (O first as terminal non-C)

use bigsmiles::parse;
use pretty_assertions::assert_eq;

/// Parse then display — verifies the canonical string.
fn roundtrip(input: &str) -> String {
    parse(input)
        .unwrap_or_else(|e| panic!("parse({input:?}) failed: {e}"))
        .to_string()
}

/// Parse → display → re-parse → display again: the second display must equal the first.
/// This confirms the output is itself valid BigSMILES and stable under a second pass.
fn stable_roundtrip(input: &str) {
    let first = roundtrip(input);
    let second = roundtrip(&first);
    assert_eq!(first, second, "display is not stable for {input:?}");
}

// ── NoBond descriptors ────────────────────────────────────────────────────────

#[test]
fn display_polyethylene_no_bond() {
    // CC is already canonical
    assert_eq!(roundtrip("{[]CC[]}"), "{[]CC[]}");
    stable_roundtrip("{[]CC[]}");
}

#[test]
fn display_polypropylene_no_bond() {
    // CC(C) must be preserved: the polymer chain grows through C1-C2 with a
    // methyl branch — topologically different from linear CCC (polypropane).
    assert_eq!(roundtrip("{[]CC(C)[]}"), "{[]CC(C)[]}");
    stable_roundtrip("{[]CC(C)[]}");
}

// ── NonDirectional descriptors ────────────────────────────────────────────────

#[test]
fn display_polyethylene_non_directional() {
    assert_eq!(roundtrip("{[$]CC[$]}"), "{[$]CC[$]}");
    stable_roundtrip("{[$]CC[$]}");
}

// ── Indexed descriptors ───────────────────────────────────────────────────────

#[test]
fn display_indexed_non_directional() {
    assert_eq!(roundtrip("{[$1]CC[$1]}"), "{[$1]CC[$1]}");
    stable_roundtrip("{[$1]CC[$1]}");
}

#[test]
fn display_indexed_directional() {
    assert_eq!(roundtrip("{[<2]CC[>2]}"), "{[<2]CC[>2]}");
    stable_roundtrip("{[<2]CC[>2]}");
}

// ── Directed with outer terminals ─────────────────────────────────────────────

#[test]
fn display_isotactic_polypropylene() {
    // CC(C) preserved: polypropylene, not polypropane.
    assert_eq!(roundtrip("{[>][<]CC(C)[>][<]}"), "{[>][<]CC(C)[>][<]}");
    stable_roundtrip("{[>][<]CC(C)[>][<]}");
}

// ── Copolymers ────────────────────────────────────────────────────────────────

#[test]
fn display_ethylene_propylene_copolymer() {
    // Both units preserved verbatim: CC(C) ≠ CCC in polymer context.
    assert_eq!(
        roundtrip("{[$]CC[$],[$]CC(C)[$]}"),
        "{[$]CC[$],[$]CC(C)[$]}"
    );
    stable_roundtrip("{[$]CC[$],[$]CC(C)[$]}");
}

#[test]
fn display_three_unit_copolymer() {
    assert_eq!(
        roundtrip("{[$]CC[$],[$]CCC[$],[$]CCCC[$]}"),
        "{[$]CC[$],[$]CCC[$],[$]CCCC[$]}"
    );
    stable_roundtrip("{[$]CC[$],[$]CCC[$],[$]CCCC[$]}");
}

// ── End groups ────────────────────────────────────────────────────────────────

#[test]
fn display_with_end_group() {
    // CCO preserved: the hydroxyl is the right connection point of the end group.
    assert_eq!(roundtrip("{[$]CC[$];[$]CCO[$]}"), "{[$]CC[$];[$]CCO[$]}");
    stable_roundtrip("{[$]CC[$];[$]CCO[$]}");
}

// ── Surrounding SMILES ────────────────────────────────────────────────────────

#[test]
fn display_alpha_omega_dimethyl_polyethylene() {
    assert_eq!(roundtrip("CC{[$]CC[$]}CC"), "CC{[$]CC[$]}CC");
    stable_roundtrip("CC{[$]CC[$]}CC");
}

#[test]
fn display_plain_smiles() {
    // Pure SMILES, no stochastic object — canonical ethanol
    assert_eq!(roundtrip("CCO"), "OCC");
    stable_roundtrip("CCO");
}

// ── Aromatic SMILES ───────────────────────────────────────────────────────────

#[test]
fn display_polystyrene() {
    // Raw SMILES preserved: phenyl branch stays attached to C2, not reordered.
    assert_eq!(roundtrip("{[]CC(c1ccccc1)[]}"), "{[]CC(c1ccccc1)[]}");
    stable_roundtrip("{[]CC(c1ccccc1)[]}");
}

// ── Double bond ───────────────────────────────────────────────────────────────

#[test]
fn display_double_bond_repeat_unit() {
    assert_eq!(roundtrip("{[$]C=C[$]}"), "{[$]C=C[$]}");
    stable_roundtrip("{[$]C=C[$]}");
}
