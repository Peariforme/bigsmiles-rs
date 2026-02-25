use thiserror::Error;

/// Errors that can occur when parsing a BigSMILES string.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParseError {
    /// Unexpected end of input.
    #[error("unexpected end of input at position {0}")]
    UnexpectedEnd(usize),

    /// Unexpected character at the given position.
    #[error("unexpected character '{0}' at position {1}")]
    UnexpectedChar(char, usize),

    /// A stochastic object `{...}` was opened but never closed.
    #[error("unclosed stochastic object '{{...'")]
    UnclosedStochasticObject,

    /// A bond descriptor `[...]` is syntactically invalid.
    #[error("invalid bond descriptor at position {0}")]
    InvalidBondDescriptor(usize),

    /// A stochastic fragment has no SMILES between its bond descriptors.
    #[error("empty SMILES in stochastic fragment")]
    EmptySmiles,

    /// The SMILES fragment embedded in a BigSMILES string failed to parse.
    #[error("SMILES parse error: {0}")]
    SmilesError(String),
}

impl From<opensmiles::ParserError> for ParseError {
    fn from(e: opensmiles::ParserError) -> Self {
        ParseError::SmilesError(e.to_string())
    }
}
