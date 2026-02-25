//! # bigsmiles-core
//!
//! A BigSMILES parser for polymer and macromolecule notation.
//!
//! BigSMILES extends SMILES with stochastic objects `{...}` to represent
//! repeat units in polymers.
//!
//! ## References
//!
//! - [BigSMILES Paper](https://pubs.acs.org/doi/10.1021/acscentsci.9b00476)
//! - [BigSMILES Documentation](https://olsenlabmit.github.io/BigSMILES/docs/line_notation.html)
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use bigsmiles_core::parse;
//!
//! let polymer = parse("{[]CC(c1ccccc1)[]}")?; // polystyrene
//! ```

// Re-export opensmiles for convenience
pub use opensmiles;

// TODO: Uncomment these as you implement each module
// pub mod ast;
// pub mod error;
// pub mod parser;

/// Placeholder - implement this after smiles-core is complete!
pub fn parse(_input: &str) -> Result<(), &'static str> {
    todo!("Implement the BigSMILES parser!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Remove when implementing
    fn parse_polyethylene() {
        let result = parse("{[]CC[]}");
        assert!(result.is_ok());
    }

    #[test]
    #[ignore]
    fn parse_polystyrene() {
        let result = parse("{[]CC(c1ccccc1)[]}");
        assert!(result.is_ok());
    }
}
