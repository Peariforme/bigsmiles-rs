//! Tests des branches (ramifications)
//!
//! Ces tests vérifient le parsing des branches dans les molécules:
//! - Branches simples `CC(C)C`
//! - Branches multiples `CC(C)(C)C`
//! - Branches imbriquées `CC(C(C)C)C`
//! - Branches avec différents types de liaisons

use smiles_core::{parse, BondType, ParserError};

#[test]
fn parse_simple_branch() {
    // CC(C)C = isobutane (2-méthylpropane)
    let molecule = parse("CC(C)C").expect("Failed to parse isobutane");

    assert_eq!(molecule.nodes().len(), 4);
    assert_eq!(molecule.bonds().len(), 3);

    // Vérifier la connectivité : atome 1 est connecté à 0, 2, et 3
    let bonds: Vec<_> = molecule.bonds().iter().collect();

    // Bond 0: C(0) - C(1)
    assert_eq!(bonds[0].source(), 0);
    assert_eq!(bonds[0].target(), 1);

    // Bond 1: C(1) - C(2) (branche)
    assert_eq!(bonds[1].source(), 1);
    assert_eq!(bonds[1].target(), 2);

    // Bond 2: C(1) - C(3)
    assert_eq!(bonds[2].source(), 1);
    assert_eq!(bonds[2].target(), 3);
}

#[test]
fn parse_multiple_branches() {
    // CC(C)(C)C = néopentane (2,2-diméthylpropane)
    let molecule = parse("CC(C)(C)C").expect("Failed to parse neopentane");

    assert_eq!(molecule.nodes().len(), 5);
    assert_eq!(molecule.bonds().len(), 4);

    // L'atome central (index 1) doit avoir 4 liaisons
    let central_bonds: Vec<_> = molecule
        .bonds()
        .iter()
        .filter(|b| b.source() == 1 || b.target() == 1)
        .collect();
    assert_eq!(central_bonds.len(), 4);
}

#[test]
fn parse_nested_branches() {
    // CC(C(C)C)C = 2,3-diméthylbutane
    let molecule = parse("CC(C(C)C)C").expect("Failed to parse 2,3-dimethylbutane");

    assert_eq!(molecule.nodes().len(), 6);
    assert_eq!(molecule.bonds().len(), 5);
}

#[test]
fn parse_branch_with_double_bond() {
    // CC(=O)O = acide acétique
    let molecule = parse("CC(=O)O").expect("Failed to parse acetic acid");

    assert_eq!(molecule.nodes().len(), 4);
    assert_eq!(molecule.bonds().len(), 3);

    // Trouver la liaison double C=O
    let double_bond = molecule
        .bonds()
        .iter()
        .find(|b| *b.kind() == BondType::Double);
    assert!(double_bond.is_some(), "Should have a double bond");
}

#[test]
fn parse_branch_with_triple_bond() {
    // CC(C#N)C = isobutyronitrile
    let molecule =
        parse("CC(C#N)C").expect("Failed to parse molecule with triple bond in branch");

    // Vérifier qu'il y a une liaison triple
    let triple_bond = molecule
        .bonds()
        .iter()
        .find(|b| *b.kind() == BondType::Triple);
    assert!(triple_bond.is_some(), "Should have a triple bond");
}

#[test]
fn parse_long_branch() {
    // C(CCCC)C = hexane avec branche longue
    let molecule = parse("C(CCCC)C").expect("Failed to parse molecule with long branch");

    assert_eq!(molecule.nodes().len(), 6);
    assert_eq!(molecule.bonds().len(), 5);
}

#[test]
fn parse_branch_at_start() {
    // (C)CC = propane (branche au début, équivalent à CCC)
    let molecule = parse("(C)CC").expect("Failed to parse branch at start");

    assert_eq!(molecule.nodes().len(), 3);
    assert_eq!(molecule.bonds().len(), 2);
}

#[test]
fn parse_empty_branch() {
    // C()C = devrait être équivalent à CC ou erreur selon l'implémentation
    // Selon OpenSMILES, les branches vides ne sont pas valides
    let result = parse("C()C");
    assert!(result.is_err(), "Empty branches should not be allowed");
}

#[test]
fn error_position_in_branch() {
    // C([C+X])C - le X est un caractère invalide dans le bracket atom
    // Position: C=1, (=2, [=3, C=4, +=5, X=6
    // L'erreur devrait indiquer la position 6 (position absolue, pas relative à la branche)
    match parse("C([C+X])C") {
        Err(ParserError::UnexpectedCharacter(c, pos)) => {
            assert_eq!(c, 'X', "Expected unexpected character 'X'");
            assert_eq!(pos, 6, "Position should be absolute (6), not relative to branch");
        }
        Ok(_) => panic!("Expected error, but parsing succeeded"),
        Err(other) => panic!("Expected UnexpectedCharacter, got {:?}", other),
    }
}
