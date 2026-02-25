//! Integration tests — error cases.

use bigsmiles::{parse, ParseError};

// ── Structural errors ─────────────────────────────────────────────────────────

#[test]
fn error_unclosed_stochastic_object() {
    let err = parse("{[$]CC[$]").unwrap_err();
    assert!(
        matches!(err, ParseError::UnclosedStochasticObject),
        "got {err:?}"
    );
}

#[test]
fn error_empty_smiles_between_bond_descriptors() {
    // After terminal_left [$], the next fragment has no SMILES between its two BDs
    let err = parse("{[$][$][$]}").unwrap_err();
    assert!(matches!(err, ParseError::EmptySmiles), "got {err:?}");
}

#[test]
fn error_invalid_smiles_in_stochastic() {
    // SMILES "INVALID_SMILES" is not valid
    let err = parse("{[$]INVALID_SMILES[$]}").unwrap_err();
    assert!(matches!(err, ParseError::SmilesError(_)), "got {err:?}");
}

#[test]
fn error_invalid_smiles_as_outer_fragment() {
    // The outer SMILES "XYZ" is not a valid element
    let err = parse("XYZ{[$]CC[$]}").unwrap_err();
    assert!(matches!(err, ParseError::SmilesError(_)), "got {err:?}");
}

#[test]
fn error_unclosed_stochastic_missing_right_bd() {
    // Missing closing bond descriptor — SMILES goes up to `}` without a right BD
    let err = parse("{[$]CC}").unwrap_err();
    // The parser tries to parse a right BD but finds `}` → UnexpectedChar
    assert!(
        matches!(
            err,
            ParseError::UnexpectedChar(_, _) | ParseError::InvalidBondDescriptor(_)
        ),
        "got {err:?}"
    );
}

// ── Bond descriptor syntax errors ─────────────────────────────────────────────

#[test]
fn error_bond_descriptor_unclosed_bracket() {
    // `[$` without closing `]` inside the stochastic object
    let err = parse("{[$CC[$]}").unwrap_err();
    assert!(
        matches!(
            err,
            ParseError::InvalidBondDescriptor(_) | ParseError::UnexpectedChar(_, _)
        ),
        "got {err:?}"
    );
}
