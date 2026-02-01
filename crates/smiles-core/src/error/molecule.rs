//! Erreurs liées aux molécules.

use thiserror::Error;

use super::{AtomError, NodeError};

/// Erreurs pouvant survenir lors de la construction d'une molécule.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum MoleculeError {
    /// Erreur provenant d'un nœud.
    #[error(transparent)]
    NodeError(#[from] NodeError),

    /// Erreur provenant d'un atome.
    #[error(transparent)]
    AtomError(#[from] AtomError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_error_conversion() {
        let node_err = NodeError::UndefinedHydrogen;
        let mol_err: MoleculeError = node_err.into();

        assert!(matches!(mol_err, MoleculeError::NodeError(_)));
        assert_eq!(mol_err.to_string(), "nombre d'hydrogènes non défini");
    }

    #[test]
    fn atom_error_conversion() {
        let atom_err = AtomError::UnknownElement("Zz".to_string());
        let mol_err: MoleculeError = atom_err.into();

        assert!(matches!(mol_err, MoleculeError::AtomError(_)));
        assert_eq!(mol_err.to_string(), "élément inconnu: 'Zz'");
    }
}
