//! Tests de base pour le parsing SMILES
//!
//! Ces tests vérifient les fonctionnalités de base du parser:
//! - Atomes simples (méthane)
//! - Chaînes linéaires (éthane, éthanol)
//! - Atomes à deux lettres (chlorométhane)

use smiles_core::{parse, AtomSymbol, BondType, OrganicAtom};

#[test]
fn parse_methane() {
    // C = méthane : 1 carbone, 0 liaisons
    let molecule = parse("C").expect("Failed to parse methane");

    // Vérifier le nombre de nodes et bonds
    assert_eq!(molecule.nodes().len(), 1);
    assert_eq!(molecule.bonds().len(), 0);

    // Vérifier le premier atome
    let node = &molecule.nodes()[0];
    assert_eq!(*node.atom().element(), AtomSymbol::Organic(OrganicAtom::C));
    assert_eq!(node.atom().charge(), 0);
    assert_eq!(node.atom().isotope(), None);
    assert_eq!(node.aromatic(), false);
    assert_eq!(node.class(), None);
    assert_eq!(node.hydrogens(), 4);
}

#[test]
fn parse_ethane() {
    // CC = éthane : 2 carbones, 1 liaison simple
    let molecule = parse("CC").expect("Failed to parse ethane");

    // Vérifier le nombre de nodes et bonds
    assert_eq!(molecule.nodes().len(), 2);
    assert_eq!(molecule.bonds().len(), 1);

    // Vérifier le premier carbone (CH3-)
    let node0 = &molecule.nodes()[0];
    assert_eq!(*node0.atom().element(), AtomSymbol::Organic(OrganicAtom::C));
    assert_eq!(node0.atom().charge(), 0);
    assert_eq!(node0.atom().isotope(), None);
    assert_eq!(node0.aromatic(), false);
    assert_eq!(node0.class(), None);
    assert_eq!(node0.hydrogens(), 3);

    // Vérifier le deuxième carbone (-CH3)
    let node1 = &molecule.nodes()[1];
    assert_eq!(*node1.atom().element(), AtomSymbol::Organic(OrganicAtom::C));
    assert_eq!(node1.atom().charge(), 0);
    assert_eq!(node1.atom().isotope(), None);
    assert_eq!(node1.aromatic(), false);
    assert_eq!(node1.class(), None);
    assert_eq!(node1.hydrogens(), 3);

    // Vérifier la liaison : C(0) - C(1)
    let bond = &molecule.bonds()[0];
    assert_eq!(*bond.kind(), BondType::Simple);
    assert_eq!(bond.source(), 0);
    assert_eq!(bond.target(), 1);
}

#[test]
fn parse_ethanol() {
    // CCO = éthanol : 2 carbones + 1 oxygène, 2 liaisons simples
    let molecule = parse("CCO").expect("Failed to parse ethanol");

    // Vérifier le nombre de nodes et bonds
    assert_eq!(molecule.nodes().len(), 3);
    assert_eq!(molecule.bonds().len(), 2);

    // Vérifier le premier carbone (CH3-)
    let node0 = &molecule.nodes()[0];
    assert_eq!(*node0.atom().element(), AtomSymbol::Organic(OrganicAtom::C));
    assert_eq!(node0.atom().charge(), 0);
    assert_eq!(node0.atom().isotope(), None);
    assert_eq!(node0.aromatic(), false);
    assert_eq!(node0.class(), None);
    assert_eq!(node0.hydrogens(), 3);

    // Vérifier le deuxième carbone (-CH2-)
    let node1 = &molecule.nodes()[1];
    assert_eq!(*node1.atom().element(), AtomSymbol::Organic(OrganicAtom::C));
    assert_eq!(node1.atom().charge(), 0);
    assert_eq!(node1.atom().isotope(), None);
    assert_eq!(node1.aromatic(), false);
    assert_eq!(node1.class(), None);
    assert_eq!(node1.hydrogens(), 2);

    // Vérifier l'oxygène (-OH)
    let node2 = &molecule.nodes()[2];
    assert_eq!(*node2.atom().element(), AtomSymbol::Organic(OrganicAtom::O));
    assert_eq!(node2.atom().charge(), 0);
    assert_eq!(node2.atom().isotope(), None);
    assert_eq!(node2.aromatic(), false);
    assert_eq!(node2.class(), None);
    assert_eq!(node2.hydrogens(), 1);

    // Vérifier les liaisons : C(0) - C(1) - O(2)
    let bond0 = &molecule.bonds()[0];
    assert_eq!(*bond0.kind(), BondType::Simple);
    assert_eq!(bond0.source(), 0);
    assert_eq!(bond0.target(), 1);

    let bond1 = &molecule.bonds()[1];
    assert_eq!(*bond1.kind(), BondType::Simple);
    assert_eq!(bond1.source(), 1);
    assert_eq!(bond1.target(), 2);
}

#[test]
fn parse_chloromethane() {
    // CCl = chlorométhane : teste les atomes à 2 lettres
    let molecule = parse("CCl").expect("Failed to parse chloromethane");

    // Vérifier le nombre de nodes et bonds
    assert_eq!(molecule.nodes().len(), 2);
    assert_eq!(molecule.bonds().len(), 1);

    // Vérifier le carbone (CH3-)
    let node0 = &molecule.nodes()[0];
    assert_eq!(*node0.atom().element(), AtomSymbol::Organic(OrganicAtom::C));
    assert_eq!(node0.atom().charge(), 0);
    assert_eq!(node0.atom().isotope(), None);
    assert_eq!(node0.aromatic(), false);
    assert_eq!(node0.class(), None);
    assert_eq!(node0.hydrogens(), 3);

    // Vérifier le chlore (-Cl)
    let node1 = &molecule.nodes()[1];
    assert_eq!(*node1.atom().element(), AtomSymbol::Organic(OrganicAtom::Cl));
    assert_eq!(node1.atom().charge(), 0);
    assert_eq!(node1.atom().isotope(), None);
    assert_eq!(node1.aromatic(), false);
    assert_eq!(node1.class(), None);
    assert_eq!(node1.hydrogens(), 0);

    // Vérifier la liaison : C(0) - Cl(1)
    let bond = &molecule.bonds()[0];
    assert_eq!(*bond.kind(), BondType::Simple);
    assert_eq!(bond.source(), 0);
    assert_eq!(bond.target(), 1);
}
