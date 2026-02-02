//! # smiles-core
//!
//! A SMILES parser following the [OpenSMILES specification](http://opensmiles.org/opensmiles.html).
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use smiles_core::parse;
//!
//! let molecule = parse("CCO")?; // ethanol
//! ```
//!
//! ## Grammar
//!
//! This parser implements an LL(1) grammar based on the formal grammar
//! described at <https://depth-first.com/articles/2020/12/21/smiles-formal-grammar-revisited/>

pub mod ast;
// pub mod element;
mod error;
pub mod parser;
// mod display;

// Re-export public API
pub use ast::*;
pub use error::*;
pub use parser::*;
