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
            assert_eq!(*node.atom().element(), AtomSymbol::Organic(OrganicAtom::C));
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

    // ========================================================================
    // Tests des liaisons explicites
    // ========================================================================

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

    // ========================================================================
    // Tests des atomes entre crochets - Hydrogènes explicites
    // ========================================================================

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_no_hydrogen() {
        // [C] = carbone sans hydrogène explicite (radical)
        let molecule = parse("[C]").expect("Failed to parse [C]");

        assert_eq!(molecule.nodes().len(), 1);

        let node = &molecule.nodes()[0];
        assert_eq!(*node.atom().element(), AtomSymbol::Organic(OrganicAtom::C));
        assert_eq!(node.hydrogens(), 0);
        assert_eq!(node.atom().charge(), 0);
        assert_eq!(node.atom().isotope(), None);
        assert_eq!(node.class(), None);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_with_hydrogen_count() {
        // [CH4] = méthane avec hydrogènes explicites
        let molecule = parse("[CH4]").expect("Failed to parse [CH4]");

        assert_eq!(molecule.nodes().len(), 1);

        let node = &molecule.nodes()[0];
        assert_eq!(*node.atom().element(), AtomSymbol::Organic(OrganicAtom::C));
        assert_eq!(node.hydrogens(), 4);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_hydrogen_implicit_one() {
        // [CH] = carbone avec 1 hydrogène (H sans nombre = 1)
        let molecule = parse("[CH]").expect("Failed to parse [CH]");

        assert_eq!(molecule.nodes().len(), 1);
        assert_eq!(molecule.nodes()[0].hydrogens(), 1);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_hydrogen_variations() {
        // Tester différentes valeurs d'hydrogènes
        let test_cases = [
            ("[CH]", 1),
            ("[CH2]", 2),
            ("[CH3]", 3),
            ("[NH2]", 2),
            ("[OH]", 1),
            ("[SH]", 1),
        ];

        for (smiles, expected_h) in test_cases {
            let molecule = parse(smiles).expect(&format!("Failed to parse {}", smiles));
            assert_eq!(
                molecule.nodes()[0].hydrogens(),
                expected_h,
                "Wrong hydrogen count for {}",
                smiles
            );
        }
    }

    // ========================================================================
    // Tests des atomes entre crochets - Charges
    // ========================================================================

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_positive_charge() {
        // [NH4+] = ion ammonium
        let molecule = parse("[NH4+]").expect("Failed to parse [NH4+]");

        assert_eq!(molecule.nodes().len(), 1);

        let node = &molecule.nodes()[0];
        assert_eq!(*node.atom().element(), AtomSymbol::Organic(OrganicAtom::N));
        assert_eq!(node.atom().charge(), 1);
        assert_eq!(node.hydrogens(), 4);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_negative_charge() {
        // [O-] = ion oxyde
        let molecule = parse("[O-]").expect("Failed to parse [O-]");

        assert_eq!(molecule.nodes().len(), 1);

        let node = &molecule.nodes()[0];
        assert_eq!(*node.atom().element(), AtomSymbol::Organic(OrganicAtom::O));
        assert_eq!(node.atom().charge(), -1);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_multiple_charge_plus() {
        // [Fe++] ou [Fe+2] = ion fer(II)
        let molecule1 = parse("[Fe++]").expect("Failed to parse [Fe++]");
        let molecule2 = parse("[Fe+2]").expect("Failed to parse [Fe+2]");

        assert_eq!(molecule1.nodes()[0].atom().charge(), 2);
        assert_eq!(molecule2.nodes()[0].atom().charge(), 2);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_multiple_charge_minus() {
        // [O--] ou [O-2] = ion oxyde doublement chargé
        let molecule1 = parse("[O--]").expect("Failed to parse [O--]");
        let molecule2 = parse("[O-2]").expect("Failed to parse [O-2]");

        assert_eq!(molecule1.nodes()[0].atom().charge(), -2);
        assert_eq!(molecule2.nodes()[0].atom().charge(), -2);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_high_charge() {
        // Charges élevées (jusqu'à +/-15 selon la spécification)
        let molecule_pos = parse("[Fe+3]").expect("Failed to parse [Fe+3]");
        let molecule_neg = parse("[P-3]").expect("Failed to parse [P-3]");

        assert_eq!(molecule_pos.nodes()[0].atom().charge(), 3);
        assert_eq!(molecule_neg.nodes()[0].atom().charge(), -3);
    }

    // ========================================================================
    // Tests des atomes entre crochets - Isotopes
    // ========================================================================

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_isotope() {
        // [13C] = carbone-13
        let molecule = parse("[13C]").expect("Failed to parse [13C]");

        assert_eq!(molecule.nodes().len(), 1);

        let node = &molecule.nodes()[0];
        assert_eq!(*node.atom().element(), AtomSymbol::Organic(OrganicAtom::C));
        assert_eq!(node.atom().isotope(), Some(13));
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_deuterium() {
        // [2H] = deutérium
        let molecule = parse("[2H]").expect("Failed to parse [2H]");

        assert_eq!(molecule.nodes().len(), 1);
        assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::H);
        assert_eq!(molecule.nodes()[0].atom().isotope(), Some(2));
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_tritium() {
        // [3H] = tritium
        let molecule = parse("[3H]").expect("Failed to parse [3H]");

        assert_eq!(molecule.nodes()[0].atom().isotope(), Some(3));
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_oxygen_18() {
        // [18O] = oxygène-18
        let molecule = parse("[18O]").expect("Failed to parse [18O]");

        assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::Organic(OrganicAtom::O));
        assert_eq!(molecule.nodes()[0].atom().isotope(), Some(18));
    }

    // ========================================================================
    // Tests des atomes entre crochets - Classes (atom mapping)
    // ========================================================================

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_class() {
        // [C:1] = carbone avec classe 1
        let molecule = parse("[C:1]").expect("Failed to parse [C:1]");

        assert_eq!(molecule.nodes().len(), 1);

        let node = &molecule.nodes()[0];
        assert_eq!(*node.atom().element(), AtomSymbol::Organic(OrganicAtom::C));
        assert_eq!(node.class(), Some(1));
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_class_variations() {
        // Tester différentes valeurs de classes
        let test_cases = [("[C:0]", 0), ("[C:1]", 1), ("[C:42]", 42), ("[C:999]", 999)];

        for (smiles, expected_class) in test_cases {
            let molecule = parse(smiles).expect(&format!("Failed to parse {}", smiles));
            assert_eq!(
                molecule.nodes()[0].class(),
                Some(expected_class),
                "Wrong class for {}",
                smiles
            );
        }
    }

    // ========================================================================
    // Tests des atomes entre crochets - Combinaisons
    // ========================================================================

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_hydrogen_and_charge() {
        // [NH4+] = ammonium : hydrogènes + charge
        let molecule = parse("[NH4+]").expect("Failed to parse [NH4+]");

        let node = &molecule.nodes()[0];
        assert_eq!(node.hydrogens(), 4);
        assert_eq!(node.atom().charge(), 1);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_isotope_and_hydrogen() {
        // [2HH] ou [2H2] = deutérium avec hydrogène
        // [13CH4] = méthane marqué au carbone-13
        let molecule = parse("[13CH4]").expect("Failed to parse [13CH4]");

        let node = &molecule.nodes()[0];
        assert_eq!(node.atom().isotope(), Some(13));
        assert_eq!(node.hydrogens(), 4);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_isotope_and_charge() {
        // [13C-] = carbone-13 anionique
        let molecule = parse("[13C-]").expect("Failed to parse [13C-]");

        let node = &molecule.nodes()[0];
        assert_eq!(node.atom().isotope(), Some(13));
        assert_eq!(node.atom().charge(), -1);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_hydrogen_and_class() {
        // [CH3:1] = méthyle avec classe
        let molecule = parse("[CH3:1]").expect("Failed to parse [CH3:1]");

        let node = &molecule.nodes()[0];
        assert_eq!(node.hydrogens(), 3);
        assert_eq!(node.class(), Some(1));
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_charge_and_class() {
        // [O-:2] = oxyde avec classe
        let molecule = parse("[O-:2]").expect("Failed to parse [O-:2]");

        let node = &molecule.nodes()[0];
        assert_eq!(node.atom().charge(), -1);
        assert_eq!(node.class(), Some(2));
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_isotope_and_class() {
        // [13C:1] = carbone-13 avec classe
        let molecule = parse("[13C:1]").expect("Failed to parse [13C:1]");

        let node = &molecule.nodes()[0];
        assert_eq!(node.atom().isotope(), Some(13));
        assert_eq!(node.class(), Some(1));
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_isotope_hydrogen_charge() {
        // [13CH3+] = méthyle cation marqué
        let molecule = parse("[13CH3+]").expect("Failed to parse [13CH3+]");

        let node = &molecule.nodes()[0];
        assert_eq!(node.atom().isotope(), Some(13));
        assert_eq!(node.hydrogens(), 3);
        assert_eq!(node.atom().charge(), 1);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_isotope_hydrogen_class() {
        // [13CH4:1] = méthane marqué avec classe
        let molecule = parse("[13CH4:1]").expect("Failed to parse [13CH4:1]");

        let node = &molecule.nodes()[0];
        assert_eq!(node.atom().isotope(), Some(13));
        assert_eq!(node.hydrogens(), 4);
        assert_eq!(node.class(), Some(1));
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_isotope_charge_class() {
        // [13C+:1] = carbone-13 cation avec classe
        let molecule = parse("[13C+:1]").expect("Failed to parse [13C+:1]");

        let node = &molecule.nodes()[0];
        assert_eq!(node.atom().isotope(), Some(13));
        assert_eq!(node.atom().charge(), 1);
        assert_eq!(node.class(), Some(1));
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_hydrogen_charge_class() {
        // [NH4+:1] = ammonium avec classe
        let molecule = parse("[NH4+:1]").expect("Failed to parse [NH4+:1]");

        let node = &molecule.nodes()[0];
        assert_eq!(node.hydrogens(), 4);
        assert_eq!(node.atom().charge(), 1);
        assert_eq!(node.class(), Some(1));
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_all_attributes() {
        // [13CH4+:1] = toutes les propriétés combinées
        let molecule = parse("[13CH4+:1]").expect("Failed to parse [13CH4+:1]");

        let node = &molecule.nodes()[0];
        assert_eq!(*node.atom().element(), AtomSymbol::Organic(OrganicAtom::C));
        assert_eq!(node.atom().isotope(), Some(13));
        assert_eq!(node.hydrogens(), 4);
        assert_eq!(node.atom().charge(), 1);
        assert_eq!(node.class(), Some(1));
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_atom_all_attributes_negative() {
        // [18OH-:5] = hydroxyde marqué avec classe
        let molecule = parse("[18OH-:5]").expect("Failed to parse [18OH-:5]");

        let node = &molecule.nodes()[0];
        assert_eq!(*node.atom().element(), AtomSymbol::Organic(OrganicAtom::O));
        assert_eq!(node.atom().isotope(), Some(18));
        assert_eq!(node.hydrogens(), 1);
        assert_eq!(node.atom().charge(), -1);
        assert_eq!(node.class(), Some(5));
    }

    // ========================================================================
    // Tests des branches
    // ========================================================================

    #[test]
    #[ignore] // Pas encore implémenté
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
    #[ignore] // Pas encore implémenté
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
    #[ignore] // Pas encore implémenté
    fn parse_nested_branches() {
        // CC(C(C)C)C = 2,3-diméthylbutane
        let molecule = parse("CC(C(C)C)C").expect("Failed to parse 2,3-dimethylbutane");

        assert_eq!(molecule.nodes().len(), 6);
        assert_eq!(molecule.bonds().len(), 5);
    }

    #[test]
    #[ignore] // Pas encore implémenté
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
    #[ignore] // Pas encore implémenté
    fn parse_branch_with_triple_bond() {
        // CC(C#N)C = isobutyronitrile
        let molecule = parse("CC(C#N)C").expect("Failed to parse molecule with triple bond in branch");

        // Vérifier qu'il y a une liaison triple
        let triple_bond = molecule
            .bonds()
            .iter()
            .find(|b| *b.kind() == BondType::Triple);
        assert!(triple_bond.is_some(), "Should have a triple bond");
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_long_branch() {
        // C(CCCC)C = hexane avec branche longue
        let molecule = parse("C(CCCC)C").expect("Failed to parse molecule with long branch");

        assert_eq!(molecule.nodes().len(), 6);
        assert_eq!(molecule.bonds().len(), 5);
    }

    // ========================================================================
    // Tests de stœchiométrie (structures déconnectées)
    // ========================================================================

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

        assert_eq!(*molecule.nodes()[1].atom().element(), AtomSymbol::Organic(OrganicAtom::Cl));
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
        assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::Organic(OrganicAtom::O));
        assert_eq!(molecule.nodes()[0].hydrogens(), 2);

        // Proton
        assert_eq!(*molecule.nodes()[1].atom().element(), AtomSymbol::H);
        assert_eq!(molecule.nodes()[1].atom().charge(), 1);
    }

    // ========================================================================
    // Tests des cycles
    // ========================================================================

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
    fn parse_multiple_ring_closures() {
        // C12CC1CC2 = spiro[2.2]pentane (deux cycles partageant un atome)
        let molecule = parse("C12CC1CC2").expect("Failed to parse spiro compound");

        // Structure: atome 0 est partagé par deux cycles de 3
        assert_eq!(molecule.nodes().len(), 5);
        // 2 liaisons linéaires + 2 fermetures de cycle
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

    // ========================================================================
    // Tests des atomes aromatiques
    // ========================================================================

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_aromatic_carbon() {
        // c = carbone aromatique seul
        let molecule = parse("c").expect("Failed to parse aromatic carbon");

        assert_eq!(molecule.nodes().len(), 1);
        assert_eq!(molecule.nodes()[0].aromatic(), true);
        assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::Organic(OrganicAtom::C));
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_aromatic_nitrogen() {
        // n = azote aromatique
        let molecule = parse("n").expect("Failed to parse aromatic nitrogen");

        assert_eq!(molecule.nodes()[0].aromatic(), true);
        assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::Organic(OrganicAtom::N));
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

    // ========================================================================
    // Tests du wildcard
    // ========================================================================

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_wildcard_atom() {
        // * = atome wildcard (n'importe quel atome)
        let molecule = parse("*").expect("Failed to parse wildcard");

        assert_eq!(molecule.nodes().len(), 1);
        assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::Wildcard);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_wildcard_in_chain() {
        // C*C = chaîne avec wildcard au milieu
        let molecule = parse("C*C").expect("Failed to parse C*C");

        assert_eq!(molecule.nodes().len(), 3);
        assert_eq!(molecule.bonds().len(), 2);

        assert_eq!(*molecule.nodes()[1].atom().element(), AtomSymbol::Wildcard);
    }

    // ========================================================================
    // Tests d'éléments non-organiques entre crochets
    // ========================================================================

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_metal() {
        // [Fe] = fer
        let molecule = parse("[Fe]").expect("Failed to parse [Fe]");

        assert_eq!(molecule.nodes().len(), 1);
        assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::Fe);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_noble_gas() {
        // [He] = hélium
        let molecule = parse("[He]").expect("Failed to parse [He]");

        assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::He);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_bracket_lanthanide() {
        // [La] = lanthane
        let molecule = parse("[La]").expect("Failed to parse [La]");

        assert_eq!(*molecule.nodes()[0].atom().element(), AtomSymbol::La);
    }

    // ========================================================================
    // Tests de molécules réelles complexes
    // ========================================================================

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_aspirin() {
        // CC(=O)Oc1ccccc1C(=O)O = aspirine
        let molecule = parse("CC(=O)Oc1ccccc1C(=O)O").expect("Failed to parse aspirin");

        assert_eq!(molecule.nodes().len(), 13);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_caffeine() {
        // Cn1cnc2c1c(=O)n(c(=O)n2C)C = caféine
        let molecule = parse("Cn1cnc2c1c(=O)n(c(=O)n2C)C").expect("Failed to parse caffeine");

        // Caféine a 14 atomes lourds (sans hydrogènes)
        assert_eq!(molecule.nodes().len(), 14);
    }

    #[test]
    #[ignore] // Pas encore implémenté
    fn parse_glucose() {
        // OC[C@H]1OC(O)[C@H](O)[C@@H](O)[C@@H]1O = D-glucose (forme cyclique)
        // Version simplifiée sans stéréochimie: OCC1OC(O)C(O)C(O)C1O
        let molecule = parse("OCC1OC(O)C(O)C(O)C1O").expect("Failed to parse glucose");

        assert_eq!(molecule.nodes().len(), 12); // 6 C + 6 O
    }
}
