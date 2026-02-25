//! AST types for the BigSMILES line notation.
//!
//! The central type is [`BigSmiles`], a sequence of [`BigSmilesSegment`]s that
//! alternate between plain SMILES fragments and [`StochasticObject`]s.

mod bigsmiles;
mod bond_descriptor;
mod stochastic_fragment;
mod stochastic_object;

pub use bigsmiles::{BigSmiles, BigSmilesSegment};
pub use bond_descriptor::{BondDescriptor, BondDescriptorKind};
pub use stochastic_fragment::StochasticFragment;
pub use stochastic_object::StochasticObject;
