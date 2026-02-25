use std::fmt;

use opensmiles::Molecule;

use super::bond_descriptor::BondDescriptor;

/// A stochastic fragment: one `[bd]SMILES[bd]` unit inside a stochastic object.
///
/// Each fragment represents one possible repeat unit (before `;`) or end group (after `;`).
///
/// # Example
///
/// For the input `[$]CC(C)[$]` (polypropylene repeat unit):
/// - `left` = `[$]` (NonDirectional, no index)
/// - `smiles_raw` = `"CC(C)"` (original string, preserved verbatim for display)
/// - `molecule` = propane (3 C atoms, 2 bonds)
/// - `left_atom` = 0  (first written C connects to the left BD)
/// - `right_atom` = 1  (second written C connects to the right BD; the branch C is at index 2)
/// - `right` = `[$]` (NonDirectional, no index)
///
/// # Connection atoms
///
/// In BigSMILES the atom ordering within the SMILES string is semantically significant:
/// - The **left** bond descriptor always connects to the **first written atom** (index 0).
/// - The **right** bond descriptor connects to the **last atom on the main chain** —
///   i.e., the last atom encountered at parenthesis depth 0.
///
/// Re-canonicalising the SMILES (e.g. `CC(C)` → `CCC`) would silently change which
/// atoms bear the descriptors, turning polypropylene into polypropane.  `smiles_raw`
/// preserves the original notation; the parsed `molecule` and `{left,right}_atom` are
/// kept for structural analysis and future polymer-chain generation.
#[derive(Debug, Clone, PartialEq)]
pub struct StochasticFragment {
    /// Left bond descriptor.
    pub left: BondDescriptor,
    /// Original SMILES string as written, used for display.
    ///
    /// Preserving the raw string ensures the bond descriptor connection points are
    /// not silently reordered by SMILES canonicalisation.
    pub smiles_raw: String,
    /// Parsed molecular structure, available for structural analysis.
    pub molecule: Molecule,
    /// Index into `molecule.nodes()` of the atom bonded to the **left** bond descriptor.
    ///
    /// Always `0` — the first atom written in the SMILES string.
    pub left_atom: usize,
    /// Index into `molecule.nodes()` of the atom bonded to the **right** bond descriptor.
    ///
    /// This is the last atom encountered on the **main chain** (parenthesis depth 0).
    /// For `CC(C)` this is `1` (the branched carbon), not `2` (the methyl branch).
    pub right_atom: usize,
    /// Right bond descriptor.
    pub right: BondDescriptor,
}

impl fmt::Display for StochasticFragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.left, self.smiles_raw, self.right)
    }
}
