//! Tests de stœchiométrie (structures déconnectées)
//!
//! Ces tests vérifient le parsing des molécules avec le séparateur `.`:
//! - Molécules simples déconnectées `C.C`
//! - Composés ioniques `[Na+].[Cl-]`
//! - Mélanges complexes

use smiles_core::{parse, AtomSymbol, OrganicAtom};

#[test]
#[ignore] // Pas encore implémenté
fn parse_disconnected_simple() {
    // C.C = deux molécules de méthane séparées
    let molecule = parse("C.C").expect("Failed to parse C.C");

    assert_eq!(molecule.nodes().len(), 2);
    assert_eq!(molecule.bonds().len(), 0); // Pas de liaison entre les deux
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_disconnected_ionic() {
    // [Na+].[Cl-] = chlorure de sodium (sel)
    let molecule = parse("[Na+].[Cl-]").expect("Failed to parse NaCl");

    assert_eq!(molecule.nodes().len(), 2);
    assert_eq!(molecule.bonds().len(), 0);

    assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::Na);
    assert_eq!(molecule.nodes()[0].atom().charge(), 1);

    assert_eq!(
        *molecule.nodes()[1].atom().element(),
        AtomSymbol::Organic(OrganicAtom::Cl)
    );
    assert_eq!(molecule.nodes()[1].atom().charge(), -1);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_disconnected_multiple() {
    // C.C.C = trois molécules de méthane
    let molecule = parse("C.C.C").expect("Failed to parse C.C.C");

    assert_eq!(molecule.nodes().len(), 3);
    assert_eq!(molecule.bonds().len(), 0);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_disconnected_complex() {
    // CC.CC = deux molécules d'éthane
    let molecule = parse("CC.CC").expect("Failed to parse CC.CC");

    assert_eq!(molecule.nodes().len(), 4);
    assert_eq!(molecule.bonds().len(), 2); // Une liaison par molécule d'éthane

    // Vérifier que les liaisons sont dans les bonnes molécules
    let bond0 = &molecule.bonds()[0];
    let bond1 = &molecule.bonds()[1];

    // Première liaison : entre atomes 0 et 1
    assert_eq!(bond0.source(), 0);
    assert_eq!(bond0.target(), 1);

    // Deuxième liaison : entre atomes 2 et 3
    assert_eq!(bond1.source(), 2);
    assert_eq!(bond1.target(), 3);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_disconnected_with_branches() {
    // CC(C)C.CC = isobutane + éthane
    let molecule = parse("CC(C)C.CC").expect("Failed to parse CC(C)C.CC");

    assert_eq!(molecule.nodes().len(), 6);
    assert_eq!(molecule.bonds().len(), 4); // 3 pour isobutane + 1 pour éthane
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_water_hydronium() {
    // [OH2].[H+] = eau + proton (acide)
    let molecule = parse("[OH2].[H+]").expect("Failed to parse water + hydronium");

    assert_eq!(molecule.nodes().len(), 2);
    assert_eq!(molecule.bonds().len(), 0);

    // Eau
    assert_eq!(
        *molecule.nodes()[0].atom().element(),
        AtomSymbol::Organic(OrganicAtom::O)
    );
    assert_eq!(molecule.nodes()[0].hydrogens(), 2);

    // Proton
    assert_eq!(*molecule.nodes()[1].atom().element(), AtomSymbol::H);
    assert_eq!(molecule.nodes()[1].atom().charge(), 1);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_hydrate() {
    // CCO.[OH2] = éthanol + eau (représentation d'un hydrate)
    let molecule = parse("CCO.[OH2]").expect("Failed to parse ethanol hydrate");

    assert_eq!(molecule.nodes().len(), 4); // 3 pour éthanol + 1 pour eau
    assert_eq!(molecule.bonds().len(), 2); // Seulement les liaisons de l'éthanol
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_metal_complex() {
    // [Cu+2].[O-].[O-] = oxyde de cuivre (II)
    let molecule = parse("[Cu+2].[O-].[O-]").expect("Failed to parse copper oxide");

    assert_eq!(molecule.nodes().len(), 3);
    assert_eq!(molecule.bonds().len(), 0);

    assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::Cu);
    assert_eq!(molecule.nodes()[0].atom().charge(), 2);

    for i in 1..=2 {
        assert_eq!(
            *molecule.nodes()[i].atom().element(),
            AtomSymbol::Organic(OrganicAtom::O)
        );
        assert_eq!(molecule.nodes()[i].atom().charge(), -1);
    }
}
