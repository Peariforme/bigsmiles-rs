use std::fmt;

use opensmiles::Molecule;

use super::stochastic_object::StochasticObject;

/// A segment in a BigSMILES string: either a plain SMILES fragment or a stochastic object.
#[derive(Debug, Clone, PartialEq)]
pub enum BigSmilesSegment {
    /// A plain SMILES molecule fragment (e.g. the `CC` parts in `CC{[$]CC[$]}CC`).
    Smiles(Molecule),
    /// A stochastic object (e.g. `{[$]CC[$]}`).
    Stochastic(StochasticObject),
}

/// A parsed BigSMILES string: a sequence of SMILES fragments and stochastic objects.
///
/// # Examples
///
/// ```rust
/// use bigsmiles::parse;
///
/// let pe = parse("{[]CC[]}").unwrap();        // polyethylene
/// let pe_end = parse("CC{[$]CC[$]}CC").unwrap(); // α,ω-dimethyl polyethylene
/// assert_eq!(pe_end.segments.len(), 3);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BigSmiles {
    /// Ordered sequence of segments making up the BigSMILES string.
    pub segments: Vec<BigSmilesSegment>,
}

impl fmt::Display for BigSmiles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for seg in &self.segments {
            match seg {
                BigSmilesSegment::Smiles(mol) => write!(f, "{}", mol)?,
                BigSmilesSegment::Stochastic(obj) => write!(f, "{}", obj)?,
            }
        }
        Ok(())
    }
}
