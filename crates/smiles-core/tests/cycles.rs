//! Ring closure tests
//!
//! These tests verify the parsing of cyclic structures:
//! - Simple rings (cyclopropane to cyclohexane)
//! - Rings with different bond types
//! - Fused and spiro rings
//! - Two-digit notation (%nn)

use smiles_core::{parse, BondType};

#[test]
fn parse_cyclopropane() {
    // C1CC1 = cyclopropane (3-carbon ring)
    let molecule = parse("C1CC1").expect("Failed to parse cyclopropane");

    assert_eq!(molecule.nodes().len(), 3);
    assert_eq!(molecule.bonds().len(), 3); // Closed ring

    // Check that there is a bond that closes the ring (0-2)
    let closing_bond = molecule
        .bonds()
        .iter()
        .find(|b| (b.source() == 0 && b.target() == 2) || (b.source() == 2 && b.target() == 0));
    assert!(closing_bond.is_some(), "Should have a closing bond");
}

#[test]
fn parse_cyclobutane() {
    // C1CCC1 = cyclobutane (4-carbon ring)
    let molecule = parse("C1CCC1").expect("Failed to parse cyclobutane");

    assert_eq!(molecule.nodes().len(), 4);
    assert_eq!(molecule.bonds().len(), 4);
}

#[test]
fn parse_cyclopentane() {
    // C1CCCC1 = cyclopentane (5-carbon ring)
    let molecule = parse("C1CCCC1").expect("Failed to parse cyclopentane");

    assert_eq!(molecule.nodes().len(), 5);
    assert_eq!(molecule.bonds().len(), 5);
}

#[test]
fn parse_cyclohexane() {
    // C1CCCCC1 = cyclohexane (6-carbon ring)
    let molecule = parse("C1CCCCC1").expect("Failed to parse cyclohexane");

    assert_eq!(molecule.nodes().len(), 6);
    assert_eq!(molecule.bonds().len(), 6);

    // Each carbon in cyclohexane has 2 hydrogens
    for node in molecule.nodes() {
        assert_eq!(node.hydrogens(), 2);
    }
}

#[test]
fn parse_cyclohexene() {
    // C1=CCCCC1 = cyclohexene (ring with a double bond)
    let molecule = parse("C1=CCCCC1").expect("Failed to parse cyclohexene");

    assert_eq!(molecule.nodes().len(), 6);
    assert_eq!(molecule.bonds().len(), 6);

    // Check that there is exactly one double bond
    let double_bonds: Vec<_> = molecule
        .bonds()
        .iter()
        .filter(|b| *b.kind() == BondType::Double)
        .collect();
    assert_eq!(double_bonds.len(), 1);
}

#[test]
fn parse_benzene() {
    // c1ccccc1 = benzene (aromatic ring)
    let molecule = parse("c1ccccc1").expect("Failed to parse benzene");

    assert_eq!(molecule.nodes().len(), 6);
    assert_eq!(molecule.bonds().len(), 6);

    // All atoms must be aromatic
    for node in molecule.nodes() {
        assert!(node.aromatic());
    }

    // All bonds must be aromatic
    for bond in molecule.bonds() {
        assert_eq!(*bond.kind(), BondType::Aromatic);
    }
}

#[test]
fn parse_multiple_ring_closures() {
    // C12CC1CC2 = spiro[2.2]pentane (two rings sharing one atom)
    let molecule = parse("C12CC1CC2").expect("Failed to parse spiro compound");

    // Structure: atom 0 is shared by two 3-membered rings
    assert_eq!(molecule.nodes().len(), 5);
    // 4 linear bonds + 2 ring closures = 6 bonds
    assert_eq!(molecule.bonds().len(), 6);
}

#[test]
fn parse_fused_rings() {
    // C1CCC2CCCCC2C1 = decalin (two fused cyclohexanes)
    let molecule = parse("C1CCC2CCCCC2C1").expect("Failed to parse decalin");

    assert_eq!(molecule.nodes().len(), 10);
    // 10 atoms, 11 bonds for two fused rings
    assert_eq!(molecule.bonds().len(), 11);
}

#[test]
fn parse_ring_with_branch() {
    // C1CC(C)CC1 = methylcyclopentane
    let molecule = parse("C1CC(C)CC1").expect("Failed to parse methylcyclopentane");

    assert_eq!(molecule.nodes().len(), 6);
    assert_eq!(molecule.bonds().len(), 6); // 5 for the ring + 1 for the branch
}

#[test]
fn parse_two_digit_ring() {
    // For large rings, use %10, %11, etc.
    // C%10CCCCCCCCC%10 = cyclodecane
    let molecule = parse("C%10CCCCCCCCC%10").expect("Failed to parse cyclodecane");

    assert_eq!(molecule.nodes().len(), 10);
    assert_eq!(molecule.bonds().len(), 10);
}

#[test]
fn parse_multiple_two_digit_rings() {
    // Use of multiple two-digit identifiers
    let molecule =
        parse("C%10%11CC%10CC%11").expect("Failed to parse molecule with multiple ring closures");

    // Check that rings are correctly closed
    assert!(molecule.bonds().len() >= 4);
}

#[test]
fn parse_naphthalene() {
    // c1ccc2ccccc2c1 = naphthalene (two fused benzenes)
    let molecule = parse("c1ccc2ccccc2c1").expect("Failed to parse naphthalene");

    assert_eq!(molecule.nodes().len(), 10);
    assert_eq!(molecule.bonds().len(), 11);

    // All atoms must be aromatic
    for node in molecule.nodes() {
        assert!(node.aromatic());
    }
}

#[test]
fn parse_cyclopropene() {
    // C1=CC1 = cyclopropene (3-membered ring with double bond)
    let molecule = parse("C1=CC1").expect("Failed to parse cyclopropene");

    assert_eq!(molecule.nodes().len(), 3);
    assert_eq!(molecule.bonds().len(), 3);

    // Check that there is a double bond
    let double_bonds: Vec<_> = molecule
        .bonds()
        .iter()
        .filter(|b| *b.kind() == BondType::Double)
        .collect();
    assert_eq!(double_bonds.len(), 1);
}

#[test]
fn parse_cubane() {
    // C12C3C4C1C5C4C3C25 = cubane (cubic structure)
    let molecule = parse("C12C3C4C1C5C4C3C25").expect("Failed to parse cubane");

    assert_eq!(molecule.nodes().len(), 8); // 8 vertices of the cube
    assert_eq!(molecule.bonds().len(), 12); // 12 edges of the cube
}
