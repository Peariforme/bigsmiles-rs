//! Erreurs liées aux atomes.

use thiserror::Error;

/// Erreurs pouvant survenir lors de la création ou manipulation d'un atome.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum AtomError {
    /// La charge spécifiée est hors limites (doit être entre -15 et +15).
    #[error("charge invalide: {0} (doit être entre -15 et +15)")]
    InvalidCharge(i8),

    /// L'isotope spécifié est trop grand (maximum 999).
    #[error("isotope invalide: {0} (maximum 999)")]
    InvalidIsotope(u16),

    /// L'ordre de liaison est requis mais n'a pas été fourni.
    #[error("ordre de liaison manquant pour calculer les hydrogènes implicites")]
    MissingBondOrder,

    /// L'élément spécifié n'est pas reconnu.
    #[error("élément inconnu: '{0}'")]
    UnknownElement(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_messages_are_descriptive() {
        assert_eq!(
            AtomError::InvalidCharge(-20).to_string(),
            "charge invalide: -20 (doit être entre -15 et +15)"
        );

        assert_eq!(
            AtomError::InvalidIsotope(1000).to_string(),
            "isotope invalide: 1000 (maximum 999)"
        );

        assert_eq!(
            AtomError::MissingBondOrder.to_string(),
            "ordre de liaison manquant pour calculer les hydrogènes implicites"
        );

        assert_eq!(
            AtomError::UnknownElement("Xy".to_string()).to_string(),
            "élément inconnu: 'Xy'"
        );
    }
}
