//! # bigsmiles
//!
//! A BigSMILES parser for polymer and macromolecule notation.
//!
//! BigSMILES extends SMILES with stochastic objects `{...}` to represent
//! repeat units in polymers.
//!
//! ## Quick Start
//!
//! ```rust
//! use bigsmiles::parse;
//!
//! let polymer = parse("{[]CC(c1ccccc1)[]}").unwrap(); // polystyrene
//! let pe = parse("CC{[$]CC[$]}CC").unwrap();          // dimethyl polyethylene
//! println!("{}", pe);
//! ```
//!
//! ## References
//!
//! - [BigSMILES Paper](https://pubs.acs.org/doi/10.1021/acscentsci.9b00476)
//! - [BigSMILES Documentation](https://olsenlabmit.github.io/BigSMILES/docs/line_notation.html)

// Re-export opensmiles for convenience
pub use opensmiles;

#[path = "ast/mod.rs"]
pub mod ast;
pub mod error;
pub mod parser;

// Re-export public API
pub use ast::*;
pub use error::*;
pub use parser::parse;
