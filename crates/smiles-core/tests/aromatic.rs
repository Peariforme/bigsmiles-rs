//! Tests des atomes aromatiques
//!
//! Ces tests vérifient le parsing des atomes aromatiques (minuscules):
//! - Carbones aromatiques (`c`)
//! - Hétéroatomes aromatiques (`n`, `o`, `s`)
//! - Cycles aromatiques courants (pyridine, furane, thiophène)

use smiles_core::{parse, AtomSymbol, BondType, OrganicAtom};

#[test]
#[ignore] // Pas encore implémenté
fn parse_aromatic_carbon() {
    // c = carbone aromatique seul
    let molecule = parse("c").expect("Failed to parse aromatic carbon");

    assert_eq!(molecule.nodes().len(), 1);
    assert_eq!(molecule.nodes()[0].aromatic(), true);
    assert_eq!(
        *molecule.nodes()[0].atom().element(),
        AtomSymbol::Organic(OrganicAtom::C)
    );
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_aromatic_nitrogen() {
    // n = azote aromatique
    let molecule = parse("n").expect("Failed to parse aromatic nitrogen");

    assert_eq!(molecule.nodes()[0].aromatic(), true);
    assert_eq!(
        *molecule.nodes()[0].atom().element(),
        AtomSymbol::Organic(OrganicAtom::N)
    );
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_aromatic_oxygen() {
    // o = oxygène aromatique
    let molecule = parse("o").expect("Failed to parse aromatic oxygen");

    assert_eq!(molecule.nodes()[0].aromatic(), true);
    assert_eq!(
        *molecule.nodes()[0].atom().element(),
        AtomSymbol::Organic(OrganicAtom::O)
    );
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_aromatic_sulfur() {
    // s = soufre aromatique
    let molecule = parse("s").expect("Failed to parse aromatic sulfur");

    assert_eq!(molecule.nodes()[0].aromatic(), true);
    assert_eq!(
        *molecule.nodes()[0].atom().element(),
        AtomSymbol::Organic(OrganicAtom::S)
    );
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_benzene() {
    // c1ccccc1 = benzène : 6 carbones aromatiques en cycle
    let molecule = parse("c1ccccc1").expect("Failed to parse benzene");

    // Vérifier le nombre de nodes et bonds
    assert_eq!(molecule.nodes().len(), 6);
    assert_eq!(molecule.bonds().len(), 6); // cycle fermé

    // Tous les atomes doivent être aromatiques avec les mêmes propriétés
    for node in molecule.nodes() {
        assert_eq!(
            *node.atom().element(),
            AtomSymbol::Organic(OrganicAtom::C)
        );
        assert_eq!(node.atom().charge(), 0);
        assert_eq!(node.atom().isotope(), None);
        assert_eq!(node.aromatic(), true);
        assert_eq!(node.class(), None);
        assert_eq!(node.hydrogens(), 1); // chaque carbone aromatique a 1 hydrogène
    }

    // Vérifier que toutes les liaisons sont aromatiques
    for bond in molecule.bonds() {
        assert_eq!(*bond.kind(), BondType::Aromatic);
    }
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_pyridine() {
    // c1ccncc1 = pyridine (cycle aromatique avec azote)
    let molecule = parse("c1ccncc1").expect("Failed to parse pyridine");

    assert_eq!(molecule.nodes().len(), 6);
    assert_eq!(molecule.bonds().len(), 6);

    // 5 carbones aromatiques + 1 azote aromatique
    let carbons: Vec<_> = molecule
        .nodes()
        .iter()
        .filter(|n| *n.atom().element() == AtomSymbol::Organic(OrganicAtom::C))
        .collect();
    let nitrogens: Vec<_> = molecule
        .nodes()
        .iter()
        .filter(|n| *n.atom().element() == AtomSymbol::Organic(OrganicAtom::N))
        .collect();

    assert_eq!(carbons.len(), 5);
    assert_eq!(nitrogens.len(), 1);

    // Tous doivent être aromatiques
    for node in molecule.nodes() {
        assert_eq!(node.aromatic(), true);
    }
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_furan() {
    // c1ccoc1 = furane (cycle aromatique avec oxygène)
    let molecule = parse("c1ccoc1").expect("Failed to parse furan");

    assert_eq!(molecule.nodes().len(), 5);
    assert_eq!(molecule.bonds().len(), 5);

    // Vérifier qu'il y a un oxygène aromatique
    let oxygen = molecule
        .nodes()
        .iter()
        .find(|n| *n.atom().element() == AtomSymbol::Organic(OrganicAtom::O));
    assert!(oxygen.is_some());
    assert_eq!(oxygen.unwrap().aromatic(), true);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_thiophene() {
    // c1ccsc1 = thiophène (cycle aromatique avec soufre)
    let molecule = parse("c1ccsc1").expect("Failed to parse thiophene");

    assert_eq!(molecule.nodes().len(), 5);

    // Vérifier qu'il y a un soufre aromatique
    let sulfur = molecule
        .nodes()
        .iter()
        .find(|n| *n.atom().element() == AtomSymbol::Organic(OrganicAtom::S));
    assert!(sulfur.is_some());
    assert_eq!(sulfur.unwrap().aromatic(), true);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_pyrrole() {
    // c1cc[nH]c1 = pyrrole (cycle aromatique avec NH)
    let molecule = parse("c1cc[nH]c1").expect("Failed to parse pyrrole");

    assert_eq!(molecule.nodes().len(), 5);
    assert_eq!(molecule.bonds().len(), 5);

    // Vérifier l'azote avec son hydrogène
    let nitrogen = molecule
        .nodes()
        .iter()
        .find(|n| *n.atom().element() == AtomSymbol::Organic(OrganicAtom::N));
    assert!(nitrogen.is_some());
    assert_eq!(nitrogen.unwrap().hydrogens(), 1);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_imidazole() {
    // c1cnc[nH]1 = imidazole (deux azotes dans le cycle)
    let molecule = parse("c1cnc[nH]1").expect("Failed to parse imidazole");

    assert_eq!(molecule.nodes().len(), 5);

    // Compter les azotes
    let nitrogens: Vec<_> = molecule
        .nodes()
        .iter()
        .filter(|n| *n.atom().element() == AtomSymbol::Organic(OrganicAtom::N))
        .collect();
    assert_eq!(nitrogens.len(), 2);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_indole() {
    // c1ccc2[nH]ccc2c1 = indole (benzène fusionné avec pyrrole)
    let molecule = parse("c1ccc2[nH]ccc2c1").expect("Failed to parse indole");

    assert_eq!(molecule.nodes().len(), 9);
    // 9 atomes avec 10 liaisons (deux cycles fusionnés)
    assert_eq!(molecule.bonds().len(), 10);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_mixed_aromatic_aliphatic() {
    // Cc1ccccc1 = toluène (méthyle + benzène)
    let molecule = parse("Cc1ccccc1").expect("Failed to parse toluene");

    assert_eq!(molecule.nodes().len(), 7);

    // Le premier carbone (méthyle) n'est pas aromatique
    assert_eq!(molecule.nodes()[0].aromatic(), false);

    // Les 6 autres carbones sont aromatiques
    for i in 1..7 {
        assert_eq!(molecule.nodes()[i].aromatic(), true);
    }
}
