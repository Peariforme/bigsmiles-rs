//! Types d'erreurs pour le parser SMILES.
//!
//! Ce module contient tous les types d'erreurs utilisés par le parser SMILES,
//! organisés par composant:
//!
//! - [`AtomError`] - Erreurs liées aux atomes (charge, isotope, élément)
//! - [`NodeError`] - Erreurs liées aux nœuds (hydrogènes, classe, aromaticité)
//! - [`MoleculeError`] - Erreurs liées à la construction de molécules
//! - [`ParserError`] - Erreurs liées au parsing de chaînes SMILES
//!
//! # Hiérarchie des erreurs
//!
//! ```text
//! ParserError
//! ├── MoleculeError
//! │   ├── NodeError
//! │   │   └── AtomError
//! │   └── AtomError
//! └── NodeError
//!     └── AtomError
//! ```
//!
//! Les conversions `From` sont implémentées pour permettre l'utilisation
//! de l'opérateur `?` à travers toute la hiérarchie.

mod atom;
mod molecule;
mod node;
mod parser;

pub use atom::AtomError;
pub use molecule::MoleculeError;
pub use node::NodeError;
pub use parser::ParserError;
