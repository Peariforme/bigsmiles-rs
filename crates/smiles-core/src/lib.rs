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
//! ## Parallel Parsing
//!
//! Enable the `parallel` feature for multi-threaded batch parsing:
//!
//! ```rust,ignore
//! use smiles_core::parser_parallel::parse_batch;
//!
//! let smiles = vec!["CCO", "c1ccccc1", "CC(=O)O"];
//! let results = parse_batch(&smiles); // parsed in parallel
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
#[cfg(feature = "parallel")]
pub mod parser_parallel;
// mod display;

// Re-export public API
pub use ast::*;
pub use error::*;
pub use parser::*;
#[cfg(feature = "parallel")]
pub use parser_parallel::*;
