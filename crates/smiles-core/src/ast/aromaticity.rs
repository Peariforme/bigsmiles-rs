use super::atom::{AtomSymbol, OrganicAtom};
use super::graph::Ring;
use super::molecule::Molecule;
use crate::MoleculeError;

/// Result of aromaticity validation for a single ring.
#[derive(Debug, Clone, PartialEq)]
pub struct AromaticityCheck {
    pub ring: Ring,
    pub pi_electrons: Option<u8>,
    pub is_valid: bool,
}

/// Determines the pi electron contribution of an atom in an aromatic ring.
///
/// Returns `None` if the contribution cannot be determined (e.g., Wildcard).
fn pi_electron_contribution(molecule: &Molecule, node_idx: u16) -> Option<u8> {
    let node = &molecule.nodes()[node_idx as usize];
    let element = node.atom().element();
    let charge = node.atom().charge();
    let has_h = node.hydrogens() > 0;

    match element {
        AtomSymbol::Organic(OrganicAtom::C) => match charge {
            -1 => Some(2), // cyclopentadienyl anion
            1 => Some(0),  // tropylium cation
            _ => Some(1),  // standard aromatic carbon
        },
        AtomSymbol::Organic(OrganicAtom::N) => {
            // Pyrrole-type N ([nH]): donates lone pair → 2 pi electrons
            // Pyridine-type N (n): contributes 1 pi electron from p orbital
            if has_h {
                Some(2)
            } else {
                Some(1)
            }
        }
        AtomSymbol::Organic(OrganicAtom::O) => Some(2), // furan-type: lone pair donor
        AtomSymbol::Organic(OrganicAtom::S) => Some(2), // thiophene-type: lone pair donor
        AtomSymbol::Organic(OrganicAtom::B) => Some(0), // empty p orbital
        AtomSymbol::Organic(OrganicAtom::P) => {
            if has_h { Some(2) } else { Some(1) }
        }
        AtomSymbol::Se => Some(2), // analogous to S
        AtomSymbol::As => {
            if has_h { Some(2) } else { Some(1) }
        }
        AtomSymbol::Te => Some(2), // analogous to Se
        AtomSymbol::Wildcard => None,
        _ => None,
    }
}

/// Checks whether a pi electron count satisfies Hückel's rule (4n+2).
fn satisfies_huckel(pi_electrons: u8) -> bool {
    pi_electrons >= 2 && (pi_electrons - 2) % 4 == 0
}

/// Validates aromaticity of all aromatic rings in a molecule.
///
/// Returns a vector of `AromaticityCheck` results, one per aromatic ring.
pub fn validate_aromaticity(molecule: &Molecule) -> Vec<AromaticityCheck> {
    let rings = molecule.aromatic_rings();
    let mut results = Vec::with_capacity(rings.len());

    for ring in rings {
        let mut total_pi: u8 = 0;
        let mut determinable = true;

        for &node_idx in &ring.nodes {
            match pi_electron_contribution(molecule, node_idx) {
                Some(contrib) => {
                    total_pi = total_pi.saturating_add(contrib);
                }
                None => {
                    determinable = false;
                    break;
                }
            }
        }

        if determinable {
            results.push(AromaticityCheck {
                ring,
                pi_electrons: Some(total_pi),
                is_valid: satisfies_huckel(total_pi),
            });
        } else {
            results.push(AromaticityCheck {
                ring,
                pi_electrons: None,
                is_valid: true, // undeterminable -> skip validation
            });
        }
    }

    results
}

/// Validates that all aromatic rings in the molecule satisfy Hückel's rule.
///
/// Returns `Ok(())` if all rings are valid or undeterminable.
/// Returns `Err(MoleculeError::HuckelViolation)` on the first invalid ring.
pub fn require_valid_aromaticity(molecule: &Molecule) -> Result<(), MoleculeError> {
    for check in validate_aromaticity(molecule) {
        if !check.is_valid {
            return Err(MoleculeError::HuckelViolation {
                ring: check.ring.nodes,
                pi_electrons: check.pi_electrons.unwrap_or(0),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn benzene_is_valid() {
        let mol = parse("c1ccccc1").unwrap();
        let checks = validate_aromaticity(&mol);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].pi_electrons, Some(6));
        assert!(checks[0].is_valid);
    }

    #[test]
    fn pyridine_is_valid() {
        let mol = parse("c1ccncc1").unwrap();
        let checks = validate_aromaticity(&mol);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].pi_electrons, Some(6));
        assert!(checks[0].is_valid);
    }

    #[test]
    fn pyrrole_is_valid() {
        let mol = parse("c1cc[nH]c1").unwrap();
        let checks = validate_aromaticity(&mol);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].pi_electrons, Some(6));
        assert!(checks[0].is_valid);
    }

    #[test]
    fn furan_is_valid() {
        let mol = parse("c1ccoc1").unwrap();
        let checks = validate_aromaticity(&mol);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].pi_electrons, Some(6));
        assert!(checks[0].is_valid);
    }

    #[test]
    fn thiophene_is_valid() {
        let mol = parse("c1ccsc1").unwrap();
        let checks = validate_aromaticity(&mol);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].pi_electrons, Some(6));
        assert!(checks[0].is_valid);
    }

    #[test]
    fn imidazole_is_valid() {
        let mol = parse("c1cnc[nH]1").unwrap();
        let checks = validate_aromaticity(&mol);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].pi_electrons, Some(6));
        assert!(checks[0].is_valid);
    }

    #[test]
    fn cyclopentadienyl_anion_is_valid() {
        let mol = parse("[c-]1cccc1").unwrap();
        let checks = validate_aromaticity(&mol);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].pi_electrons, Some(6));
        assert!(checks[0].is_valid);
    }

    #[test]
    fn huckel_validation_passes_for_benzene() {
        let mol = parse("c1ccccc1").unwrap();
        assert!(require_valid_aromaticity(&mol).is_ok());
    }

    #[test]
    fn satisfies_huckel_values() {
        assert!(!satisfies_huckel(0));
        assert!(!satisfies_huckel(1));
        assert!(satisfies_huckel(2));
        assert!(!satisfies_huckel(3));
        assert!(!satisfies_huckel(4));
        assert!(!satisfies_huckel(5));
        assert!(satisfies_huckel(6));
        assert!(!satisfies_huckel(7));
        assert!(!satisfies_huckel(8));
        assert!(!satisfies_huckel(9));
        assert!(satisfies_huckel(10));
        assert!(satisfies_huckel(14));
        assert!(satisfies_huckel(18));
        assert!(satisfies_huckel(22));
    }

    #[test]
    fn non_aromatic_molecule_returns_empty() {
        let mol = parse("CCCCCC").unwrap();
        let checks = validate_aromaticity(&mol);
        assert!(checks.is_empty());
    }

    #[test]
    fn naphthalene_both_rings_valid() {
        let mol = parse("c1ccc2ccccc2c1").unwrap();
        let checks = validate_aromaticity(&mol);
        assert_eq!(checks.len(), 2);
        for check in &checks {
            assert!(check.is_valid);
        }
    }
}
