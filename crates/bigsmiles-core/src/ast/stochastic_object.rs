use std::fmt;

use super::bond_descriptor::BondDescriptor;
use super::stochastic_fragment::StochasticFragment;

/// A BigSMILES stochastic object `{...}`.
///
/// Represents a statistical mixture of repeat units describing a polymer.
///
/// # Structure
///
/// ```text
/// { [left_end]? [bd]smiles[bd] , ... ; [bd]smiles[bd] , ... [right_end]? }
///                ^─── repeat_units ───^   ^─── end_groups ───^
/// ```
///
/// The optional `left_end` and `right_end` bond descriptors (directional `[>]`/`[<]` or
/// non-directional `[$]`) describe how this stochastic object connects to the rest of the
/// molecule. An absent terminal means the object has no outer connection on that side.
///
/// # Examples
///
/// | BigSMILES          | left_end | repeat_units | right_end |
/// |--------------------|----------|--------------|-----------|
/// | `{[]CC[]}`         | –        | `[]CC[]`     | –         |
/// | `{[$]CC[$]}`       | –        | `[$]CC[$]`   | –         |
/// | `{[>][<]CC[>][<]}` | `[>]`    | `[<]CC[>]`   | `[<]`     |
#[derive(Debug, Clone, PartialEq)]
pub struct StochasticObject {
    /// Bond descriptor connecting this object to the preceding SMILES fragment (if any).
    pub left_end: Option<BondDescriptor>,
    /// Repeat unit fragments (comma-separated before `;`).
    pub repeat_units: Vec<StochasticFragment>,
    /// End group fragments (comma-separated after `;`).
    pub end_groups: Vec<StochasticFragment>,
    /// Bond descriptor connecting this object to the following SMILES fragment (if any).
    pub right_end: Option<BondDescriptor>,
}

impl fmt::Display for StochasticObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        if let Some(le) = &self.left_end {
            write!(f, "{}", le)?;
        }
        for (i, ru) in self.repeat_units.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", ru)?;
        }
        if !self.end_groups.is_empty() {
            write!(f, ";")?;
            for (i, eg) in self.end_groups.iter().enumerate() {
                if i > 0 {
                    write!(f, ",")?;
                }
                write!(f, "{}", eg)?;
            }
        }
        if let Some(re) = &self.right_end {
            write!(f, "{}", re)?;
        }
        write!(f, "}}")
    }
}
