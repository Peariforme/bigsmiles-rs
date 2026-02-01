//! Erreurs liées au parsing SMILES.

use thiserror::Error;

use super::{MoleculeError, NodeError};

/// Erreurs pouvant survenir lors du parsing d'une chaîne SMILES.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParserError {
    /// Fonctionnalité pas encore implémentée.
    #[error("fonctionnalité pas encore implémentée")]
    NotYetImplemented,

    /// La molécule contient trop de nœuds (maximum 65535).
    #[error("trop de nœuds dans la molécule (maximum 65535)")]
    TooManyNodes,

    /// Caractère inattendu dans la chaîne SMILES.
    #[error("caractère inattendu '{0}' à la position {1}")]
    UnexpectedCharacter(char, usize),

    /// Fin de chaîne inattendue.
    #[error("fin de chaîne inattendue, attendu: {0}")]
    UnexpectedEndOfInput(String),

    /// Crochet fermant manquant.
    #[error("crochet fermant ']' manquant")]
    UnclosedBracket,

    /// Parenthèse fermante manquante.
    #[error("parenthèse fermante ')' manquante")]
    UnclosedParenthesis,

    /// Cycle non fermé.
    #[error("cycle {0} non fermé")]
    UnclosedRing(u8),

    /// Types de liaison incompatibles pour la fermeture de cycle.
    #[error("types de liaison incompatibles pour le cycle {0}")]
    MismatchedRingBond(u8),

    /// Liaison sans atome précédent.
    #[error("liaison sans atome précédent")]
    BondWithoutPrecedingAtom,

    /// Liaison sans atome suivant.
    #[error("liaison sans atome suivant")]
    BondWithoutFollowingAtom,

    /// Erreur provenant de la construction de molécule.
    #[error(transparent)]
    MoleculeError(#[from] MoleculeError),

    /// Erreur provenant d'un nœud.
    #[error(transparent)]
    NodeError(#[from] NodeError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_messages_are_descriptive() {
        assert_eq!(
            ParserError::NotYetImplemented.to_string(),
            "fonctionnalité pas encore implémentée"
        );

        assert_eq!(
            ParserError::TooManyNodes.to_string(),
            "trop de nœuds dans la molécule (maximum 65535)"
        );

        assert_eq!(
            ParserError::UnexpectedCharacter('$', 5).to_string(),
            "caractère inattendu '$' à la position 5"
        );

        assert_eq!(
            ParserError::UnexpectedEndOfInput("élément".to_string()).to_string(),
            "fin de chaîne inattendue, attendu: élément"
        );

        assert_eq!(
            ParserError::UnclosedBracket.to_string(),
            "crochet fermant ']' manquant"
        );

        assert_eq!(
            ParserError::UnclosedParenthesis.to_string(),
            "parenthèse fermante ')' manquante"
        );

        assert_eq!(
            ParserError::UnclosedRing(1).to_string(),
            "cycle 1 non fermé"
        );

        assert_eq!(
            ParserError::MismatchedRingBond(2).to_string(),
            "types de liaison incompatibles pour le cycle 2"
        );

        assert_eq!(
            ParserError::BondWithoutPrecedingAtom.to_string(),
            "liaison sans atome précédent"
        );

        assert_eq!(
            ParserError::BondWithoutFollowingAtom.to_string(),
            "liaison sans atome suivant"
        );
    }

    #[test]
    fn molecule_error_conversion() {
        let mol_err = MoleculeError::NodeError(super::super::NodeError::UndefinedHydrogen);
        let parser_err: ParserError = mol_err.into();

        assert!(matches!(parser_err, ParserError::MoleculeError(_)));
    }

    #[test]
    fn node_error_conversion() {
        let node_err = super::super::NodeError::InvalidHydrogen(99);
        let parser_err: ParserError = node_err.into();

        assert!(matches!(parser_err, ParserError::NodeError(_)));
        assert_eq!(parser_err.to_string(), "nombre d'hydrogènes invalide: 99");
    }
}
