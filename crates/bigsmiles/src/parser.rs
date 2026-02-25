use crate::{
    ast::{
        BigSmiles, BigSmilesSegment, BondDescriptor, BondDescriptorKind, StochasticFragment,
        StochasticObject,
    },
    error::ParseError,
};

/// Parse a BigSMILES string into a [`BigSmiles`] AST.
///
/// BigSMILES extends SMILES with stochastic objects `{...}` for polymer notation.
///
/// # Examples
///
/// ```rust
/// use bigsmiles::parse;
///
/// let pe = parse("{[]CC[]}").unwrap();
/// let ps = parse("{[]CC(c1ccccc1)[]}").unwrap();
/// let copo = parse("{[$]CC[$],[$]CC(C)[$]}").unwrap();
/// let pe_end = parse("CC{[$]CC[$]}CC").unwrap();
/// ```
pub fn parse(input: &str) -> Result<BigSmiles, ParseError> {
    Parser::new(input).parse_bigsmiles()
}

// ── Connection-atom helper ────────────────────────────────────────────────────

/// Returns the index (in the parsed `Molecule` node list) of the atom bonded
/// to the **right** bond descriptor — i.e., the last atom written on the
/// *main chain* (parenthesis depth 0) of the SMILES string.
///
/// In `CC(C)` the main-chain atoms are the first two Cs (indices 0 and 1);
/// the methyl branch C (index 2) is at depth 1, so this returns `1`.
///
/// The left connection atom is always index `0` (the first atom written).
fn right_connection_atom(smiles: &str) -> usize {
    let mut atom_idx: usize = 0;
    let mut right: usize = 0;
    let mut depth: usize = 0;
    let bytes = smiles.as_bytes();
    let mut i = 0;

    // Advance `i` past any ring-closure tokens (single digit or `%NN`).
    // Ring closures appear directly after an atom symbol or `]`.
    macro_rules! skip_ring_closures {
        () => {
            while i < bytes.len() {
                if bytes[i] == b'%' && i + 2 < bytes.len() {
                    i += 3; // %NN
                } else if bytes[i].is_ascii_digit() {
                    i += 1;
                } else {
                    break;
                }
            }
        };
    }

    while i < bytes.len() {
        match bytes[i] {
            b'(' => {
                depth += 1;
                i += 1;
            }
            b')' => {
                depth -= 1;
                i += 1;
            }
            b'[' => {
                // Bracket atom — consume until matching `]`
                i += 1;
                while i < bytes.len() && bytes[i] != b']' {
                    i += 1;
                }
                if i < bytes.len() {
                    i += 1; // skip `]`
                }
                skip_ring_closures!();
                if depth == 0 {
                    right = atom_idx;
                }
                atom_idx += 1;
            }
            // Bond characters and dot — not atoms
            b'-' | b'=' | b'#' | b'$' | b':' | b'/' | b'\\' | b'.' => {
                i += 1;
            }
            b'*' => {
                i += 1;
                skip_ring_closures!();
                if depth == 0 {
                    right = atom_idx;
                }
                atom_idx += 1;
            }
            c if c.is_ascii_alphabetic() => {
                // Two-letter organic atoms: Cl, Br
                if (c == b'C' && i + 1 < bytes.len() && bytes[i + 1] == b'l')
                    || (c == b'B' && i + 1 < bytes.len() && bytes[i + 1] == b'r')
                {
                    i += 2;
                } else {
                    i += 1;
                }
                skip_ring_closures!();
                if depth == 0 {
                    right = atom_idx;
                }
                atom_idx += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    right
}

// ── Internal parser ──────────────────────────────────────────────────────────

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    // ── Primitive helpers ────────────────────────────────────────────────────

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn consume(&mut self) -> Option<char> {
        let c = self.input[self.pos..].chars().next()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn expect(&mut self, expected: char) -> Result<(), ParseError> {
        match self.consume() {
            Some(c) if c == expected => Ok(()),
            Some(c) => Err(ParseError::UnexpectedChar(c, self.pos - c.len_utf8())),
            None => Err(ParseError::UnexpectedEnd(self.pos)),
        }
    }

    // ── Bond-descriptor lookahead ────────────────────────────────────────────

    /// Returns true if the byte at `pos` starts a bond descriptor `[$/</>/]]`.
    ///
    /// Bond descriptors start with `[` followed by `$`, `<`, `>`, or `]` (empty).
    /// This is unambiguous because SMILES bracket atoms always start with a letter,
    /// digit, or `*` after `[`.
    fn is_bd_at(&self, pos: usize) -> bool {
        let bytes = self.input.as_bytes();
        if pos >= bytes.len() || bytes[pos] != b'[' {
            return false;
        }
        let next = pos + 1;
        next < bytes.len() && matches!(bytes[next], b'$' | b'<' | b'>' | b']')
    }

    fn is_bd_here(&self) -> bool {
        self.is_bd_at(self.pos)
    }

    /// Scans past a bond descriptor at `from_pos` (without mutating `self.pos`).
    /// Returns the position immediately after the closing `]`, or `None` if no
    /// valid BD is found at `from_pos`.
    fn skip_bd_at(&self, from_pos: usize) -> Option<usize> {
        let bytes = self.input.as_bytes();
        if !self.is_bd_at(from_pos) {
            return None;
        }
        let mut p = from_pos + 1; // skip `[`
                                  // skip descriptor type char (`$`, `<`, `>`) — or nothing for `[]`
        if p < bytes.len() && matches!(bytes[p], b'$' | b'<' | b'>') {
            p += 1;
        }
        // skip optional digits
        while p < bytes.len() && bytes[p].is_ascii_digit() {
            p += 1;
        }
        // expect `]`
        if p < bytes.len() && bytes[p] == b']' {
            Some(p + 1)
        } else {
            None
        }
    }

    /// Returns the first char after the bond descriptor starting at `self.pos`,
    /// or `None` if the BD is malformed or at end-of-input.
    fn peek_after_current_bd(&self) -> Option<char> {
        let after = self.skip_bd_at(self.pos)?;
        self.input[after..].chars().next()
    }

    /// Returns true if the token immediately following the BD at `self.pos`
    /// is itself another bond descriptor.
    fn is_bd_after_current_bd(&self) -> bool {
        match self.skip_bd_at(self.pos) {
            Some(after) => self.is_bd_at(after),
            None => false,
        }
    }

    // ── SMILES substring extraction ──────────────────────────────────────────

    /// Extracts a SMILES substring from `self.pos` up to (but not including)
    /// the next BigSMILES structural delimiter when *outside* a stochastic object.
    /// Delimiters: `{` (start of stochastic object).
    ///
    /// Correctly skips over SMILES bracket atoms `[element...]`.
    fn extract_outer_smiles(&mut self) -> &'a str {
        let start = self.pos;
        loop {
            match self.peek() {
                None | Some('{') => break,
                Some('[') => {
                    // SMILES bracket atom — consume until matching `]`
                    self.pos += 1;
                    loop {
                        match self.consume() {
                            Some(']') | None => break,
                            _ => {}
                        }
                    }
                }
                Some(c) => {
                    self.pos += c.len_utf8();
                }
            }
        }
        &self.input[start..self.pos]
    }

    /// Extracts a SMILES substring from `self.pos` up to (but not including)
    /// the next BigSMILES delimiter when *inside* a stochastic object.
    /// Delimiters: `[bd`, `,`, `;`, `}`.
    ///
    /// Correctly skips over SMILES bracket atoms `[element...]`.
    fn extract_inner_smiles(&mut self) -> &'a str {
        let start = self.pos;
        loop {
            match self.peek() {
                None | Some('}') | Some(',') | Some(';') => break,
                Some('[') if self.is_bd_here() => break,
                Some('[') => {
                    // SMILES bracket atom — consume until matching `]`
                    self.pos += 1;
                    loop {
                        match self.consume() {
                            Some(']') | None => break,
                            _ => {}
                        }
                    }
                }
                Some(c) => {
                    self.pos += c.len_utf8();
                }
            }
        }
        &self.input[start..self.pos]
    }

    // ── Bond descriptor parser ───────────────────────────────────────────────

    fn parse_bond_descriptor(&mut self) -> Result<BondDescriptor, ParseError> {
        let pos_before = self.pos;
        self.expect('[')?;
        let kind = match self.peek() {
            Some(']') => BondDescriptorKind::NoBond,
            Some('$') => {
                self.consume();
                BondDescriptorKind::NonDirectional
            }
            Some('<') => {
                self.consume();
                BondDescriptorKind::Head
            }
            Some('>') => {
                self.consume();
                BondDescriptorKind::Tail
            }
            Some(c) => return Err(ParseError::UnexpectedChar(c, self.pos)),
            None => return Err(ParseError::UnexpectedEnd(self.pos)),
        };
        // Optional numeric index
        let mut index_digits = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                index_digits.push(c);
                self.consume();
            } else {
                break;
            }
        }
        let index = if index_digits.is_empty() {
            None
        } else {
            Some(
                index_digits
                    .parse::<u32>()
                    .map_err(|_| ParseError::InvalidBondDescriptor(pos_before))?,
            )
        };
        self.expect(']')
            .map_err(|_| ParseError::InvalidBondDescriptor(pos_before))?;
        Ok(BondDescriptor { kind, index })
    }

    // ── Stochastic fragment parser ───────────────────────────────────────────

    fn parse_stochastic_fragment(&mut self) -> Result<StochasticFragment, ParseError> {
        let left = self.parse_bond_descriptor()?;
        let smiles_str = self.extract_inner_smiles();
        if smiles_str.is_empty() {
            return Err(ParseError::EmptySmiles);
        }
        let smiles_raw = smiles_str.to_owned();
        let right_atom = right_connection_atom(smiles_str);
        let molecule = opensmiles::parse(smiles_str)?;
        let right = self.parse_bond_descriptor()?;
        Ok(StochasticFragment {
            left,
            smiles_raw,
            molecule,
            left_atom: 0,
            right_atom,
            right,
        })
    }

    // ── Stochastic object parser ─────────────────────────────────────────────

    fn parse_stochastic_object(&mut self) -> Result<StochasticObject, ParseError> {
        self.expect('{')?;

        let mut obj = StochasticObject {
            left_end: None,
            repeat_units: Vec::new(),
            end_groups: Vec::new(),
            right_end: None,
        };

        // Handle empty stochastic object `{}`
        if self.peek() == Some('}') {
            self.consume();
            return Ok(obj);
        }

        // Detect `left_end`: if the current BD is immediately followed by another BD,
        // the first one is the "outer" terminal connecting to the left SMILES fragment.
        // Example: `{[>][<]CC[>][<]}` → left_end = `[>]`, first fragment = `[<]CC[>]`
        if self.is_bd_here() && self.is_bd_after_current_bd() {
            obj.left_end = Some(self.parse_bond_descriptor()?);
        }

        // Parse repeat units (and optionally end groups after `;`)
        let mut in_end_groups = false;
        loop {
            // Detect `right_end`: a BD immediately before `}` (no SMILES follows it).
            // This BD is the outer terminal connecting to the right SMILES fragment.
            if self.is_bd_here() && self.peek_after_current_bd() == Some('}') {
                obj.right_end = Some(self.parse_bond_descriptor()?);
                self.expect('}')?;
                return Ok(obj);
            }

            // Parse a full stochastic fragment: [bd]SMILES[bd]
            let frag = self.parse_stochastic_fragment()?;
            if in_end_groups {
                obj.end_groups.push(frag);
            } else {
                obj.repeat_units.push(frag);
            }

            // Decide what to do next
            match self.peek() {
                Some(',') => {
                    self.consume();
                }
                Some(';') => {
                    self.consume();
                    in_end_groups = true;
                }
                Some('}') => {
                    self.consume();
                    return Ok(obj);
                }
                // A bond descriptor immediately before `}` is the right terminal.
                Some('[') if self.is_bd_here() && self.peek_after_current_bd() == Some('}') => {
                    obj.right_end = Some(self.parse_bond_descriptor()?);
                    self.expect('}')?;
                    return Ok(obj);
                }
                None => return Err(ParseError::UnclosedStochasticObject),
                Some(c) => return Err(ParseError::UnexpectedChar(c, self.pos)),
            }
        }
    }

    // ── Top-level BigSMILES parser ───────────────────────────────────────────

    fn parse_bigsmiles(mut self) -> Result<BigSmiles, ParseError> {
        let mut segments = Vec::new();

        while self.pos < self.input.len() {
            match self.peek() {
                Some('{') => {
                    let obj = self.parse_stochastic_object()?;
                    segments.push(BigSmilesSegment::Stochastic(obj));
                }
                Some(_) => {
                    let smiles_str = self.extract_outer_smiles();
                    if !smiles_str.is_empty() {
                        let mol = opensmiles::parse(smiles_str)?;
                        segments.push(BigSmilesSegment::Smiles(mol));
                    }
                }
                None => break,
            }
        }

        Ok(BigSmiles { segments })
    }
}
