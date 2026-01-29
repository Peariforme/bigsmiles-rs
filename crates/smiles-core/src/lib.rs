//! # smiles-core
//!
//! A SMILES parser following the [OpenSMILES specification](http://opensmiles.org/opensmiles.html).
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use smiles_core::parse;
//!
//! let molecule = parse("CCO")?; // ethanol
//! ```
//!
//! ## Grammar
//!
//! This parser implements an LL(1) grammar based on the formal grammar
//! described at <https://depth-first.com/articles/2020/12/21/smiles-formal-grammar-revisited/>

pub mod ast;
// pub mod element;
mod error;
pub mod parser;
// mod display;

// Re-export public API
pub use ast::*;
pub use error::*;
pub use parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_methane() {
        // C = méthane : 1 carbone, 0 liaisons
        let molecule = parse("C").expect("Failed to parse methane");

        // Vérifier le nombre de nodes et bonds
        assert_eq!(molecule.nodes().len(), 1);
        assert_eq!(molecule.bonds().len(), 0);

        // Vérifier le premier atome
        let node = &molecule.nodes()[0];
        assert_eq!(*node.atom().element(), AtomSymbol::C);
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

        // Vérifier les atomes
        assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::C);
        assert_eq!(*molecule.nodes()[1].atom().element(), AtomSymbol::C);

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

        // Vérifier les atomes
        assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::C);
        assert_eq!(*molecule.nodes()[1].atom().element(), AtomSymbol::C);
        assert_eq!(*molecule.nodes()[2].atom().element(), AtomSymbol::O);
        assert_eq!(molecule.nodes()[2].hydrogens(), 1);

        // Vérifier les liaisons : C(0) - C(1) - O(2)
        assert_eq!(molecule.bonds()[0].source(), 0);
        assert_eq!(molecule.bonds()[0].target(), 1);
        assert_eq!(molecule.bonds()[1].source(), 1);
        assert_eq!(molecule.bonds()[1].target(), 2);
    }

    #[test]
    fn parse_chloromethane() {
        // CCl = chlorométhane : teste les atomes à 2 lettres
        let molecule = parse("CCl").expect("Failed to parse chloromethane");

        assert_eq!(molecule.nodes().len(), 2);
        assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::C);
        assert_eq!(*molecule.nodes()[1].atom().element(), AtomSymbol::Cl);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_benzene() {
        // c1ccccc1 = benzène : 6 carbones aromatiques en cycle
        let molecule = parse("c1ccccc1").expect("Failed to parse benzene");

        assert_eq!(molecule.nodes().len(), 6);
        assert_eq!(molecule.bonds().len(), 6); // cycle fermé

        // Tous les atomes doivent être aromatiques
        for node in molecule.nodes() {
            assert_eq!(*node.atom().element(), AtomSymbol::C);
            assert_eq!(node.aromatic(), true);
        }
    }
}
