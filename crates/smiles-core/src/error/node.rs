//! Erreurs liées aux nœuds (atomes dans le graphe moléculaire).

use thiserror::Error;

use super::AtomError;

/// Erreurs pouvant survenir lors de la création ou manipulation d'un nœud.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum NodeError {
    /// Le nombre d'hydrogènes spécifié est invalide.
    #[error("nombre d'hydrogènes invalide: {0}")]
    InvalidHydrogen(u8),

    /// Le nombre d'hydrogènes n'a pas été défini alors qu'il est requis.
    #[error("nombre d'hydrogènes non défini")]
    UndefinedHydrogen,

    /// La classe d'atome spécifiée est invalide.
    #[error("classe d'atome invalide: {0}")]
    InvalidClass(u16),

    /// L'aromaticité n'a pas été définie alors qu'elle est requise.
    #[error("aromaticité non définie")]
    UndefinedAromatic,

    /// L'ordre de liaison est obligatoire pour les atomes organiques.
    #[error("l'ordre de liaison est obligatoire pour les atomes organiques")]
    BondOrderMandatoryForOrganicAtom,

    /// Erreur provenant de l'atome sous-jacent.
    #[error(transparent)]
    AtomError(#[from] AtomError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_messages_are_descriptive() {
        assert_eq!(
            NodeError::InvalidHydrogen(10).to_string(),
            "nombre d'hydrogènes invalide: 10"
        );

        assert_eq!(
            NodeError::UndefinedHydrogen.to_string(),
            "nombre d'hydrogènes non défini"
        );

        assert_eq!(
            NodeError::InvalidClass(65535).to_string(),
            "classe d'atome invalide: 65535"
        );

        assert_eq!(
            NodeError::UndefinedAromatic.to_string(),
            "aromaticité non définie"
        );

        assert_eq!(
            NodeError::BondOrderMandatoryForOrganicAtom.to_string(),
            "l'ordre de liaison est obligatoire pour les atomes organiques"
        );
    }

    #[test]
    fn atom_error_conversion() {
        let atom_err = AtomError::InvalidCharge(20);
        let node_err: NodeError = atom_err.into();

        assert!(matches!(node_err, NodeError::AtomError(_)));
        assert_eq!(
            node_err.to_string(),
            "charge invalide: 20 (doit être entre -15 et +15)"
        );
    }
}
