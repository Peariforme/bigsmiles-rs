//! Integration tests for `BigSmiles::prefix_segments` and `BigSmiles::suffix_segments`.

use bigsmiles::{parse, BigSmilesSegment};

// ── prefix_segments ───────────────────────────────────────────────────────────

#[test]
fn prefix_segments_alpha_methyl() {
    // "CC{[$]CC[$]}" — one SMILES fragment before the stochastic object
    let bs = parse("CC{[$]CC[$]}").unwrap();
    let prefix = bs.prefix_segments();
    assert_eq!(prefix.len(), 1);
    assert!(matches!(prefix[0], BigSmilesSegment::Smiles(_)));
}

#[test]
fn prefix_segments_none_when_starts_with_stochastic() {
    // "{[$]CC[$]}CC" — no fragment before the stochastic object
    let bs = parse("{[$]CC[$]}CC").unwrap();
    assert!(bs.prefix_segments().is_empty());
}

#[test]
fn prefix_segments_empty_when_no_stochastic() {
    // Pure SMILES (no stochastic object) — returns empty slice
    let bs = parse("CCO").unwrap();
    assert!(bs.prefix_segments().is_empty());
}

#[test]
fn prefix_segments_only_up_to_first_stochastic() {
    // "CC{[$]CC[$]}{[$]OO[$]}" — prefix is just the leading "CC", not between the two stochastics
    let bs = parse("CC{[$]CC[$]}{[$]OO[$]}").unwrap();
    let prefix = bs.prefix_segments();
    assert_eq!(prefix.len(), 1);
    assert!(matches!(prefix[0], BigSmilesSegment::Smiles(_)));
}

// ── suffix_segments ───────────────────────────────────────────────────────────

#[test]
fn suffix_segments_omega_methyl() {
    // "{[$]CC[$]}CC" — one SMILES fragment after the stochastic object
    let bs = parse("{[$]CC[$]}CC").unwrap();
    let suffix = bs.suffix_segments();
    assert_eq!(suffix.len(), 1);
    assert!(matches!(suffix[0], BigSmilesSegment::Smiles(_)));
}

#[test]
fn suffix_segments_none_when_ends_with_stochastic() {
    // "CC{[$]CC[$]}" — no fragment after the stochastic object
    let bs = parse("CC{[$]CC[$]}").unwrap();
    assert!(bs.suffix_segments().is_empty());
}

#[test]
fn suffix_segments_empty_when_no_stochastic() {
    // Pure SMILES (no stochastic object) — returns empty slice
    let bs = parse("CCO").unwrap();
    assert!(bs.suffix_segments().is_empty());
}

#[test]
fn suffix_segments_only_after_last_stochastic() {
    // "{[$]CC[$]}{[$]OO[$]}CC" — suffix is just the trailing "CC"
    let bs = parse("{[$]CC[$]}{[$]OO[$]}CC").unwrap();
    let suffix = bs.suffix_segments();
    assert_eq!(suffix.len(), 1);
    assert!(matches!(suffix[0], BigSmilesSegment::Smiles(_)));
}

// ── both together ─────────────────────────────────────────────────────────────

#[test]
fn prefix_and_suffix_alpha_omega_dimethyl() {
    // "CC{[$]CC[$]}CC" — one prefix, one suffix
    let bs = parse("CC{[$]CC[$]}CC").unwrap();
    assert_eq!(bs.prefix_segments().len(), 1);
    assert_eq!(bs.suffix_segments().len(), 1);
}

#[test]
fn prefix_and_suffix_only_stochastic() {
    // "{[$]CC[$]}" — no prefix, no suffix
    let bs = parse("{[$]CC[$]}").unwrap();
    assert!(bs.prefix_segments().is_empty());
    assert!(bs.suffix_segments().is_empty());
}
