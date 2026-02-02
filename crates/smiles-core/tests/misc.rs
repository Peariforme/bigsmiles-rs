//! Tests divers
//!
//! Ces tests couvrent:
//! - Wildcards (`*`)
//! - Molécules complexes réelles
//! - Cas limites

use smiles_core::{parse, AtomSymbol};

// ============================================================================
// Wildcards
// ============================================================================

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

#[test]
#[ignore] // Pas encore implémenté
fn parse_wildcard_in_ring() {
    // C1*CC1 = cycle avec wildcard
    let molecule = parse("C1*CC1").expect("Failed to parse ring with wildcard");

    assert_eq!(molecule.nodes().len(), 4);
    assert_eq!(molecule.bonds().len(), 4);
}

// ============================================================================
// Molécules réelles complexes
// ============================================================================

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
    // Version simplifiée sans stéréochimie: OCC1OC(O)C(O)C(O)C1O
    let molecule = parse("OCC1OC(O)C(O)C(O)C1O").expect("Failed to parse glucose");

    assert_eq!(molecule.nodes().len(), 12); // 6 C + 6 O
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_ethanol_explicit() {
    // [CH3][CH2][OH] = éthanol avec tout explicite
    let molecule = parse("[CH3][CH2][OH]").expect("Failed to parse explicit ethanol");

    assert_eq!(molecule.nodes().len(), 3);
    assert_eq!(molecule.bonds().len(), 2);

    assert_eq!(molecule.nodes()[0].hydrogens(), 3);
    assert_eq!(molecule.nodes()[1].hydrogens(), 2);
    assert_eq!(molecule.nodes()[2].hydrogens(), 1);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_acetone() {
    // CC(=O)C = acétone
    let molecule = parse("CC(=O)C").expect("Failed to parse acetone");

    assert_eq!(molecule.nodes().len(), 4);
    assert_eq!(molecule.bonds().len(), 3);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_acetic_acid() {
    // CC(=O)O = acide acétique
    let molecule = parse("CC(=O)O").expect("Failed to parse acetic acid");

    assert_eq!(molecule.nodes().len(), 4);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_benzaldehyde() {
    // c1ccccc1C=O = benzaldéhyde
    let molecule = parse("c1ccccc1C=O").expect("Failed to parse benzaldehyde");

    assert_eq!(molecule.nodes().len(), 8);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_aniline() {
    // Nc1ccccc1 = aniline
    let molecule = parse("Nc1ccccc1").expect("Failed to parse aniline");

    assert_eq!(molecule.nodes().len(), 7);
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_phenol() {
    // Oc1ccccc1 = phénol
    let molecule = parse("Oc1ccccc1").expect("Failed to parse phenol");

    assert_eq!(molecule.nodes().len(), 7);
}

// ============================================================================
// Cas limites et validation
// ============================================================================

#[test]
fn parse_empty_string() {
    // Chaîne vide devrait produire une erreur ou une molécule vide
    let result = parse("");
    // Selon l'implémentation, cela peut être Ok avec 0 atomes ou Err
    if let Ok(molecule) = result {
        assert_eq!(molecule.nodes().len(), 0);
    }
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_single_bond_at_start() {
    // -C devrait être invalide (liaison sans atome précédent)
    let result = parse("-C");
    assert!(result.is_err(), "Bond at start should be invalid");
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_double_bond_at_end() {
    // C= devrait être invalide (liaison sans atome suivant)
    let result = parse("C=");
    assert!(result.is_err(), "Bond at end should be invalid");
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_unclosed_bracket() {
    // [C devrait être invalide
    let result = parse("[C");
    assert!(result.is_err(), "Unclosed bracket should be invalid");
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_unclosed_ring() {
    // C1CC devrait être invalide (cycle non fermé)
    let result = parse("C1CC");
    assert!(result.is_err(), "Unclosed ring should be invalid");
}

#[test]
#[ignore] // Pas encore implémenté
fn parse_mismatched_ring_bonds() {
    // C=1CC1 vs C1CC=1 - les types de liaisons aux fermetures doivent correspondre
    // C1CC=1 est valide (liaison simple puis double = double)
    // C=1CC1 est valide (liaison double puis simple = double)
    // C=1CC-1 devrait être invalide (double vs simple explicite)
    let result = parse("C=1CC-1");
    assert!(
        result.is_err(),
        "Mismatched ring bond types should be invalid"
    );
}
