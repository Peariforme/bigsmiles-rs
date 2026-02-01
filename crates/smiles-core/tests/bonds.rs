//! Tests des liaisons explicites
//!
//! Ces tests vérifient le parsing des différents types de liaisons:
//! - Simple (`-`)
//! - Double (`=`)
//! - Triple (`#`)
//! - Quadruple (`$`)
//! - Aromatique (`:`)

use smiles_core::{parse, BondType};

#[test]
#[ignore] // Pas encore implémenté
fn parse_explicit_single_bond() {
    // C-C = éthane avec liaison simple explicite
    let molecule = parse("C-C").expect("Failed to parse C-C");

    assert_eq!(molecule.nodes().len(), 2);
    assert_eq!(molecule.bonds().len(), 1);

    let bond = &molecule.bonds()[0];
    assert_eq!(*bond.kind(), BondType::Simple);
    assert_eq!(bond.source(), 0);
    assert_eq!(bond.target(), 1);

    // Vérifier les hydrogènes implicites (CH3-CH3)
    assert_eq!(molecule.nodes()[0].hydrogens(), 3);
    assert_eq!(molecule.nodes()[1].hydrogens(), 3);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_double_bond() {
    // C=C = éthène (éthylène) : liaison double
    let molecule = parse("C=C").expect("Failed to parse ethene");

    assert_eq!(molecule.nodes().len(), 2);
    assert_eq!(molecule.bonds().len(), 1);

    let bond = &molecule.bonds()[0];
    assert_eq!(*bond.kind(), BondType::Double);
    assert_eq!(bond.source(), 0);
    assert_eq!(bond.target(), 1);

    // CH2=CH2 : chaque carbone a 2 hydrogènes
    assert_eq!(molecule.nodes()[0].hydrogens(), 2);
    assert_eq!(molecule.nodes()[1].hydrogens(), 2);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_triple_bond() {
    // C#C = éthyne (acétylène) : liaison triple
    let molecule = parse("C#C").expect("Failed to parse ethyne");

    assert_eq!(molecule.nodes().len(), 2);
    assert_eq!(molecule.bonds().len(), 1);

    let bond = &molecule.bonds()[0];
    assert_eq!(*bond.kind(), BondType::Triple);
    assert_eq!(bond.source(), 0);
    assert_eq!(bond.target(), 1);

    // HC≡CH : chaque carbone a 1 hydrogène
    assert_eq!(molecule.nodes()[0].hydrogens(), 1);
    assert_eq!(molecule.nodes()[1].hydrogens(), 1);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_quadruple_bond() {
    // C$C = liaison quadruple (rare, utilisé pour certains complexes métalliques)
    let molecule = parse("C$C").expect("Failed to parse quadruple bond");

    assert_eq!(molecule.nodes().len(), 2);
    assert_eq!(molecule.bonds().len(), 1);

    let bond = &molecule.bonds()[0];
    assert_eq!(*bond.kind(), BondType::Quadruple);
    assert_eq!(bond.source(), 0);
    assert_eq!(bond.target(), 1);

    // Pas d'hydrogènes avec une liaison quadruple
    assert_eq!(molecule.nodes()[0].hydrogens(), 0);
    assert_eq!(molecule.nodes()[1].hydrogens(), 0);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_mixed_bonds() {
    // C=C-C#N = acrylonitrile : mélange de liaisons
    let molecule = parse("C=C-C#N").expect("Failed to parse acrylonitrile");

    assert_eq!(molecule.nodes().len(), 4);
    assert_eq!(molecule.bonds().len(), 3);

    // Liaison double C=C
    assert_eq!(*molecule.bonds()[0].kind(), BondType::Double);
    assert_eq!(molecule.bonds()[0].source(), 0);
    assert_eq!(molecule.bonds()[0].target(), 1);

    // Liaison simple C-C
    assert_eq!(*molecule.bonds()[1].kind(), BondType::Simple);
    assert_eq!(molecule.bonds()[1].source(), 1);
    assert_eq!(molecule.bonds()[1].target(), 2);

    // Liaison triple C#N
    assert_eq!(*molecule.bonds()[2].kind(), BondType::Triple);
    assert_eq!(molecule.bonds()[2].source(), 2);
    assert_eq!(molecule.bonds()[2].target(), 3);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_aromatic_bond() {
    // c:c = liaison aromatique explicite
    let molecule = parse("c:c").expect("Failed to parse aromatic bond");

    assert_eq!(molecule.nodes().len(), 2);
    assert_eq!(molecule.bonds().len(), 1);

    let bond = &molecule.bonds()[0];
    assert_eq!(*bond.kind(), BondType::Aromatic);

    // Les deux atomes doivent être aromatiques
    assert_eq!(molecule.nodes()[0].aromatic(), true);
    assert_eq!(molecule.nodes()[1].aromatic(), true);
}
