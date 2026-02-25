use std::fmt;

/// The kind of a BigSMILES bond descriptor.
///
/// Bond descriptors appear inside stochastic objects `{...}` and indicate
/// how repeat units connect to each other and to terminal groups.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BondDescriptorKind {
    /// `[]` — no bond; the stochastic object has no connection to the surrounding molecule.
    NoBond,
    /// `[$]` — non-directional bond; pairs with any other `[$]` of the same index.
    NonDirectional,
    /// `[<]` — directional head bond; pairs with a [`Tail`] bond.
    Head,
    /// `[>]` — directional tail bond; pairs with a [`Head`] bond.
    Tail,
}

/// A BigSMILES bond descriptor: `[]`, `[$]`, `[<]`, `[>]`, `[$1]`, `[<2]`, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct BondDescriptor {
    /// The kind of descriptor.
    pub kind: BondDescriptorKind,
    /// Optional numeric index to disambiguate multiple descriptors of the same kind.
    pub index: Option<u32>,
}

impl fmt::Display for BondDescriptorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BondDescriptorKind::NoBond => Ok(()),
            BondDescriptorKind::NonDirectional => write!(f, "$"),
            BondDescriptorKind::Head => write!(f, "<"),
            BondDescriptorKind::Tail => write!(f, ">"),
        }
    }
}

impl fmt::Display for BondDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        write!(f, "{}", self.kind)?;
        if let Some(idx) = self.index {
            write!(f, "{}", idx)?;
        }
        write!(f, "]")
    }
}
