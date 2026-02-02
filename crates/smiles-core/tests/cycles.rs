//! Tests des cycles (ring closures)
//!
//! Ces tests vérifient le parsing des structures cycliques:
//! - Cycles simples (cyclopropane à cyclohexane)
//! - Cycles avec différentes liaisons
//! - Cycles fusionnés et spiro
//! - Notation à deux chiffres (%nn)

use smiles_core::{parse, BondType};

#[test]
#[ignore] // Pas encore implémenté
fn parse_cyclopropane() {
    // C1CC1 = cyclopropane (cycle à 3 carbones)
    let molecule = parse("C1CC1").expect("Failed to parse cyclopropane");

    assert_eq!(molecule.nodes().len(), 3);
    assert_eq!(molecule.bonds().len(), 3); // Cycle fermé

    // Vérifier qu'il y a une liaison qui ferme le cycle (0-2)
    let closing_bond = molecule
        .bonds()
        .iter()
        .find(|b| (b.source() == 0 && b.target() == 2) || (b.source() == 2 && b.target() == 0));
    assert!(closing_bond.is_some(), "Should have a closing bond");
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_cyclobutane() {
    // C1CCC1 = cyclobutane (cycle à 4 carbones)
    let molecule = parse("C1CCC1").expect("Failed to parse cyclobutane");

    assert_eq!(molecule.nodes().len(), 4);
    assert_eq!(molecule.bonds().len(), 4);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_cyclopentane() {
    // C1CCCC1 = cyclopentane (cycle à 5 carbones)
    let molecule = parse("C1CCCC1").expect("Failed to parse cyclopentane");

    assert_eq!(molecule.nodes().len(), 5);
    assert_eq!(molecule.bonds().len(), 5);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_cyclohexane() {
    // C1CCCCC1 = cyclohexane (cycle à 6 carbones)
    let molecule = parse("C1CCCCC1").expect("Failed to parse cyclohexane");

    assert_eq!(molecule.nodes().len(), 6);
    assert_eq!(molecule.bonds().len(), 6);

    // Chaque carbone dans cyclohexane a 2 hydrogènes
    for node in molecule.nodes() {
        assert_eq!(node.hydrogens(), 2);
    }
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_cyclohexene() {
    // C1=CCCCC1 = cyclohexène (cycle avec une double liaison)
    let molecule = parse("C1=CCCCC1").expect("Failed to parse cyclohexene");

    assert_eq!(molecule.nodes().len(), 6);
    assert_eq!(molecule.bonds().len(), 6);

    // Vérifier qu'il y a exactement une liaison double
    let double_bonds: Vec<_> = molecule
        .bonds()
        .iter()
        .filter(|b| *b.kind() == BondType::Double)
        .collect();
    assert_eq!(double_bonds.len(), 1);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_benzene() {
    // c1ccccc1 = benzène (cycle aromatique)
    let molecule = parse("c1ccccc1").expect("Failed to parse benzene");

    assert_eq!(molecule.nodes().len(), 6);
    assert_eq!(molecule.bonds().len(), 6);

    // Tous les atomes doivent être aromatiques
    for node in molecule.nodes() {
        assert_eq!(node.aromatic(), true);
    }

    // Toutes les liaisons doivent être aromatiques
    for bond in molecule.bonds() {
        assert_eq!(*bond.kind(), BondType::Aromatic);
    }
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_multiple_ring_closures() {
    // C12CC1CC2 = spiro[2.2]pentane (deux cycles partageant un atome)
    let molecule = parse("C12CC1CC2").expect("Failed to parse spiro compound");

    // Structure: atome 0 est partagé par deux cycles de 3
    assert_eq!(molecule.nodes().len(), 5);
    // 4 liaisons linéaires + 2 fermetures de cycle = 6 liaisons
    assert_eq!(molecule.bonds().len(), 6);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_fused_rings() {
    // C1CC2CCCCC2C1 = décaline (deux cyclohexanes fusionnés)
    let molecule = parse("C1CC2CCCCC2C1").expect("Failed to parse decalin");

    assert_eq!(molecule.nodes().len(), 10);
    // 10 atomes, 11 liaisons pour deux cycles fusionnés
    assert_eq!(molecule.bonds().len(), 11);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_ring_with_branch() {
    // C1CC(C)CC1 = méthylcyclopentane
    let molecule = parse("C1CC(C)CC1").expect("Failed to parse methylcyclopentane");

    assert_eq!(molecule.nodes().len(), 6);
    assert_eq!(molecule.bonds().len(), 6); // 5 pour le cycle + 1 pour la branche
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_two_digit_ring() {
    // Pour les grands cycles, utiliser %10, %11, etc.
    // C%10CCCCCCCCC%10 = cyclodécane
    let molecule = parse("C%10CCCCCCCCC%10").expect("Failed to parse cyclodecane");

    assert_eq!(molecule.nodes().len(), 10);
    assert_eq!(molecule.bonds().len(), 10);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_multiple_two_digit_rings() {
    // Utilisation de plusieurs identifiants à deux chiffres
    let molecule =
        parse("C%10%11CC%10CC%11").expect("Failed to parse molecule with multiple ring closures");

    // Vérifier que les cycles sont correctement fermés
    assert!(molecule.bonds().len() >= 4);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_naphthalene() {
    // c1ccc2ccccc2c1 = naphtalène (deux benzènes fusionnés)
    let molecule = parse("c1ccc2ccccc2c1").expect("Failed to parse naphthalene");

    assert_eq!(molecule.nodes().len(), 10);
    assert_eq!(molecule.bonds().len(), 11);

    // Tous les atomes doivent être aromatiques
    for node in molecule.nodes() {
        assert_eq!(node.aromatic(), true);
    }
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_cyclopropene() {
    // C1=CC1 = cyclopropène (cycle à 3 avec double liaison)
    let molecule = parse("C1=CC1").expect("Failed to parse cyclopropene");

    assert_eq!(molecule.nodes().len(), 3);
    assert_eq!(molecule.bonds().len(), 3);

    // Vérifier qu'il y a une liaison double
    let double_bonds: Vec<_> = molecule
        .bonds()
        .iter()
        .filter(|b| *b.kind() == BondType::Double)
        .collect();
    assert_eq!(double_bonds.len(), 1);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_cubane() {
    // C12C3C4C1C5C4C3C25 = cubane (structure cubique)
    let molecule = parse("C12C3C4C1C5C4C3C25").expect("Failed to parse cubane");

    assert_eq!(molecule.nodes().len(), 8); // 8 sommets du cube
    assert_eq!(molecule.bonds().len(), 12); // 12 arêtes du cube
}
