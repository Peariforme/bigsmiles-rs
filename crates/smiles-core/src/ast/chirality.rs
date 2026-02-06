/// Chirality specification per the OpenSMILES specification.
///
/// Uses `#[repr(u8)]` starting at 1 so that `Option<Chirality>` benefits
/// from niche optimization and occupies exactly **1 byte** (0 = `None`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Chirality {
    // Tetrahedral
    TH1 = 1, TH2,
    // Allenal
    AL1, AL2,
    // Square Planar
    SP1, SP2, SP3,
    // Trigonal Bipyramidal (1-20)
    TB1, TB2, TB3, TB4, TB5, TB6, TB7, TB8, TB9, TB10,
    TB11, TB12, TB13, TB14, TB15, TB16, TB17, TB18, TB19, TB20,
    // Octahedral (1-30)
    OH1, OH2, OH3, OH4, OH5, OH6, OH7, OH8, OH9, OH10,
    OH11, OH12, OH13, OH14, OH15, OH16, OH17, OH18, OH19, OH20,
    OH21, OH22, OH23, OH24, OH25, OH26, OH27, OH28, OH29, OH30,
}

impl Chirality {
    /// Construct a trigonal-bipyramidal variant from an index (1–20).
    pub fn tb(n: u8) -> Option<Self> {
        // TB1 is the 8th variant (discriminant = 8)
        if (1..=20).contains(&n) {
            // SAFETY: TB1..TB20 are contiguous variants 8..27
            Some(unsafe { std::mem::transmute::<u8, Chirality>(7 + n) })
        } else {
            None
        }
    }

    /// Construct an octahedral variant from an index (1–30).
    pub fn oh(n: u8) -> Option<Self> {
        // OH1 is the 28th variant (discriminant = 28)
        if (1..=30).contains(&n) {
            // SAFETY: OH1..OH30 are contiguous variants 28..57
            Some(unsafe { std::mem::transmute::<u8, Chirality>(27 + n) })
        } else {
            None
        }
    }
}
