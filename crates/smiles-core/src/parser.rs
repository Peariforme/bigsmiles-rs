use std::collections::HashMap;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

use crate::error::ParserError;
use crate::{AtomSymbol, BondType, Chirality, Molecule, MoleculeBuilder};

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
    position: usize,
    builder: MoleculeBuilder,
    next_bond_type: Option<BondType>,
    next_bond_source: Option<u16>,
    branch_bond_type: Option<BondType>,
    cycles_target: HashMap<u8, (u16, Option<BondType>)>, // (node_index, bond_type_at_open)
    node_offset: u16, // Offset for global node indexing (used in branches)
    deferred_ring_bonds: Vec<(u16, u16, BondType)>, // (main_target, local_source, bond_type)
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser {
            chars: input.chars().peekable(),
            position: 0,
            builder: MoleculeBuilder::new(),
            next_bond_type: None,
            next_bond_source: None,
            branch_bond_type: None,
            cycles_target: HashMap::new(),
            node_offset: 0,
            deferred_ring_bonds: Vec::new(),
        }
    }

    fn new_with_offset(
        input: &'a str,
        position_offset: usize,
        node_offset: u16,
        cycles_target: HashMap<u8, (u16, Option<BondType>)>,
    ) -> Self {
        Parser {
            chars: input.chars().peekable(),
            position: position_offset,
            builder: MoleculeBuilder::new(),
            next_bond_type: None,
            next_bond_source: None,
            branch_bond_type: None,
            cycles_target,
            node_offset,
            deferred_ring_bonds: Vec::new(),
        }
    }

    fn next(&mut self) -> Option<char> {
        self.position += 1;
        self.chars.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    #[allow(clippy::type_complexity)]
    fn parse(
        mut self,
    ) -> Result<
        (
            MoleculeBuilder,
            Option<BondType>,                     // branch_bond_type
            Option<BondType>,                     // next_bond_type (for dangling bond detection)
            HashMap<u8, (u16, Option<BondType>)>, // cycles_target
            Vec<(u16, u16, BondType)>,            // deferred_ring_bonds
        ),
        ParserError,
    > {
        while let Some(c) = self.next() {
            // Atom
            if c.is_ascii_alphabetic() || c == '*' {
                let elem = self.parse_element_symbol(c, false);
                // Aromaticity is indicated by lowercase letters (c, n, o, etc.)
                // Wildcard '*' outside brackets is non-aromatic by default
                let aromatic = Some(c.is_ascii_lowercase());
                self.builder
                    .add_atom(elem, 0, None, aromatic, None, None, None)?;
                self.connect_current_atom()?;
            // Brackets Atom
            } else if c == '[' {
                let (elem, charge, isotope, aromatic, hydrogen, class, chirality) =
                    self.parse_bracket_atom()?;
                self.builder
                    .add_atom(elem, charge, isotope, aromatic, hydrogen, class, chirality)?;
                self.connect_current_atom()?;

            // Dot separator (disconnected fragments) — resets the chain
            } else if c == '.' {
                self.next_bond_type = None;
                self.next_bond_source = None;
                if self.builder.nodes().is_empty() {
                    self.branch_bond_type = Some(BondType::Disconnected);
                }

            // Explicit bond
            } else if c == '-'
                || c == '='
                || c == '#'
                || c == '$'
                || c == ':'
                || c == '/'
                || c == '\\'
            {
                self.next_bond_type = Some(BondType::try_from(&c)?);
                if self.builder.nodes().is_empty() {
                    self.branch_bond_type = self.next_bond_type;
                }

            // Branches
            } else if c == '(' {
                self.parse_branch()?;
            // cycles
            } else if c == '%' || c.is_ascii_digit() {
                let cycle_number: u8 = if c == '%' {
                    let first = self.next().ok_or(ParserError::UnexpectedEndOfInput(
                        "cycle number".to_string(),
                    ))?;
                    let second = self.next().ok_or(ParserError::UnexpectedEndOfInput(
                        "cycle number".to_string(),
                    ))?;
                    let first_u8: u8 = first
                        .to_digit(10)
                        .ok_or(ParserError::UnexpectedCharacter(first, self.position))?
                        as u8;
                    let second_u8: u8 = second
                        .to_digit(10)
                        .ok_or(ParserError::UnexpectedCharacter(second, self.position))?
                        as u8;
                    first_u8 * 10 + second_u8
                } else {
                    c.to_digit(10).expect("Unreachable error") as u8
                };

                // If the key already exists, close the ring
                if let Some((target, bond_type_at_open)) =
                    self.cycles_target.get(&cycle_number).copied()
                {
                    let local_index = self.get_current_atom_index()?;
                    let global_source = self.node_offset + local_index;
                    let bond_type_at_close = self.next_bond_type.take();

                    // An atom cannot be bonded to itself (e.g., C11)
                    if global_source == target {
                        return Err(ParserError::SelfBond(cycle_number));
                    }

                    // Two atoms cannot be joined by more than one bond (e.g., C12CCCCC12)
                    if self.has_bond_between(target, global_source) {
                        return Err(ParserError::DuplicateBond(target, global_source));
                    }

                    // Validate ring bond types match if both are explicitly specified
                    if let (Some(open), Some(close)) = (bond_type_at_open, bond_type_at_close) {
                        if open != close {
                            return Err(ParserError::MismatchedRingBond(cycle_number));
                        }
                    }

                    // Check if the target is from a parent parser (before our node_offset)
                    if target < self.node_offset {
                        // Defer this bond - it connects to a node in the parent
                        // Use explicit bond type if specified, otherwise Simple
                        // (aromaticity will be determined by parent when creating the bond)
                        let ring_bond_type = bond_type_at_open
                            .or(bond_type_at_close)
                            .unwrap_or(BondType::Simple);
                        self.deferred_ring_bonds
                            .push((target, local_index, ring_bond_type));
                    } else {
                        // Target is within this parser's nodes
                        // Determine bond type from explicit specification or aromaticity
                        let ring_bond_type =
                            bond_type_at_open.or(bond_type_at_close).unwrap_or_else(|| {
                                // Adjust target index for local builder space
                                let local_target = target - self.node_offset;
                                let source_aromatic =
                                    self.builder.nodes()[local_index as usize].aromatic();
                                let target_aromatic =
                                    self.builder.nodes()[local_target as usize].aromatic();
                                if source_aromatic == Some(true) && target_aromatic == Some(true) {
                                    BondType::Aromatic
                                } else {
                                    BondType::Simple
                                }
                            });
                        self.connect_ring_closure(target, ring_bond_type)?;
                    }
                    // Remove the key so it can be reused
                    self.cycles_target.remove(&cycle_number);
                // Otherwise, this is the start of a new ring
                } else {
                    let local_index = self
                        .next_bond_source
                        .ok_or(ParserError::UnexpectedCharacter(c, self.position))?;
                    let global_index = self.node_offset + local_index;
                    let bond_type_at_open = self.next_bond_type.take();
                    self.cycles_target
                        .insert(cycle_number, (global_index, bond_type_at_open));
                }
            // Whitespace terminates the SMILES string (OpenSMILES spec)
            } else if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
                break;
            } else {
                return Err(ParserError::UnexpectedCharacter(c, self.position));
            }
        }

        Ok((
            self.builder,
            self.branch_bond_type,
            self.next_bond_type,
            self.cycles_target,
            self.deferred_ring_bonds,
        ))
    }

    fn parse_branch(&mut self) -> Result<(), ParserError> {
        let mut s = String::new();
        let mut parenthesis_count: i8 = 1;
        let position = self.position;
        while let Some(c) = self.next() {
            if c == '(' {
                parenthesis_count += 1;
            }

            if c == ')' {
                parenthesis_count -= 1;

                if parenthesis_count == 0 {
                    break;
                }
            }
            s.push(c);
        }
        if parenthesis_count > 0 {
            return Err(ParserError::UnclosedParenthesis);
        }
        if parenthesis_count < 0 {
            return Err(ParserError::UnopenedParenthesis);
        }
        if s.is_empty() {
            return Err(ParserError::EmptyBranch);
        }

        // Calculate the global node offset for the branch
        let branch_node_offset = self.node_offset + self.builder.nodes().len() as u16;

        // Pass cycles_target to branch so rings can span branch boundaries
        let branch_parser = Parser::new_with_offset(
            &s,
            position,
            branch_node_offset,
            std::mem::take(&mut self.cycles_target),
        );
        let (branch_builder, branch_bond_type, _, updated_cycles, deferred_bonds) =
            branch_parser.parse()?;

        // Restore the updated cycles_target
        self.cycles_target = updated_cycles;

        // Add the branch to the main builder
        // A dot at the start of a branch means disconnected — no bond to parent
        let bond_type = branch_bond_type.unwrap_or(BondType::Simple);
        let connect_source = if bond_type == BondType::Disconnected {
            None
        } else {
            self.next_bond_source
        };
        self.builder
            .add_branch(branch_builder, bond_type, connect_source);

        // Create deferred ring bonds (rings opened in parent, closed in branch)
        for (main_target, branch_local_source, ring_bond_type) in deferred_bonds {
            // branch_local_source needs to be adjusted to main molecule space
            let main_source = branch_node_offset + branch_local_source;
            self.builder
                .add_bond(main_target, main_source, ring_bond_type);
        }

        if self.next_bond_source.is_none() {
            self.next_bond_source = Some(0);
        }
        Ok(())
    }

    /// Parse element symbol. `in_bracket` controls whether all two-letter elements
    /// are allowed (true) or only organic subset Cl/Br (false).
    fn parse_element_symbol(&mut self, c: char, in_bracket: bool) -> String {
        if c.is_ascii_uppercase() {
            if let Some(&next_c) = self.peek() {
                if next_c.is_ascii_lowercase() {
                    let two_letter = format!("{}{}", c, next_c);
                    if in_bracket {
                        // In brackets, all valid two-letter elements are allowed
                        if AtomSymbol::from_str(&two_letter).is_ok() {
                            self.next();
                            return two_letter;
                        }
                    } else {
                        // Outside brackets, only Cl and Br are valid two-letter elements
                        if two_letter == "Cl" || two_letter == "Br" {
                            self.next();
                            return two_letter;
                        }
                    }
                }
            }
        } else if in_bracket && c.is_ascii_lowercase() {
            // Aromatic two-letter symbols: se, as, te (OpenSMILES spec)
            if let Some(&next_c) = self.peek() {
                if next_c.is_ascii_lowercase() {
                    let two_letter = format!("{}{}", c, next_c);
                    // from_str already normalizes to uppercase internally
                    if AtomSymbol::from_str(&two_letter).is_ok() {
                        self.next();
                        return two_letter;
                    }
                }
            }
        }

        c.to_string()
    }

    #[allow(clippy::type_complexity)]
    fn parse_bracket_atom(
        &mut self,
    ) -> Result<
        (
            String,
            i8,
            Option<u16>,
            Option<bool>,
            Option<u8>,
            Option<u16>,
            Option<Chirality>,
        ),
        ParserError,
    > {
        let isotope = self.parse_isotope();

        let first_char = self.next().ok_or(ParserError::UnexpectedEndOfInput(
            "Element identifier".to_string(),
        ))?;
        if !first_char.is_alphabetic() && first_char != '*' {
            return Err(ParserError::MissingElementInBracketAtom);
        }
        let elem = self.parse_element_symbol(first_char, true);

        let chirality = self.parse_chirality()?;
        let hydrogen = self.parse_hydrogen()?;
        let charge = self.parse_charge()?;
        let class = self.parse_class();

        match self.next() {
            Some(']') => (),
            None => Err(ParserError::UnexpectedEndOfInput("]".to_string()))?,
            Some(c) => Err(ParserError::UnexpectedCharacter(c, self.position))?,
        }

        // A hydrogen atom cannot have a hydrogen count (e.g., [HH1] is illegal)
        if elem.eq_ignore_ascii_case("H") {
            if let Some(h) = hydrogen {
                if h > 0 {
                    return Err(ParserError::HydrogenWithHydrogenCount);
                }
            }
        }

        let aromatic = Some(elem.to_lowercase() == elem);

        Ok((elem, charge, isotope, aromatic, hydrogen, class, chirality))
    }

    fn parse_isotope(&mut self) -> Option<u16> {
        let mut builder = String::new();
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            builder.push(self.next().unwrap());
        }
        builder.parse::<u16>().ok()
    }

    fn parse_class(&mut self) -> Option<u16> {
        if self.peek().is_some_and(|c| *c == ':') {
            self.next();
            let mut builder = String::new();
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                builder.push(self.next().unwrap());
            }
            builder.parse::<u16>().ok()
        } else {
            None
        }
    }

    fn parse_chirality(&mut self) -> Result<Option<Chirality>, ParserError> {
        if self.peek() != Some(&'@') {
            return Ok(None);
        }
        self.next(); // consume first '@'

        match self.peek() {
            Some(&'@') => {
                self.next();
                Ok(Some(Chirality::TH2))
            }
            Some(&'T') => {
                self.next();
                match self.next() {
                    Some('H') => self.parse_chirality_index(1, 2, |n| match n {
                        1 => Some(Chirality::TH1),
                        2 => Some(Chirality::TH2),
                        _ => None,
                    }),
                    Some('B') => self.parse_chirality_index(1, 20, |n| Chirality::tb(n as u8)),
                    Some(c) => Err(ParserError::InvalidChiralitySpec(
                        format!("@T{}", c),
                        self.position,
                    )),
                    None => Err(ParserError::UnexpectedEndOfInput(
                        "chirality class".to_string(),
                    )),
                }
            }
            Some(&'A') => {
                self.next();
                match self.next() {
                    Some('L') => self.parse_chirality_index(1, 2, |n| match n {
                        1 => Some(Chirality::AL1),
                        2 => Some(Chirality::AL2),
                        _ => None,
                    }),
                    Some(c) => Err(ParserError::InvalidChiralitySpec(
                        format!("@A{}", c),
                        self.position,
                    )),
                    None => Err(ParserError::UnexpectedEndOfInput(
                        "chirality class".to_string(),
                    )),
                }
            }
            Some(&'S') => {
                self.next();
                match self.next() {
                    Some('P') => self.parse_chirality_index(1, 3, |n| match n {
                        1 => Some(Chirality::SP1),
                        2 => Some(Chirality::SP2),
                        3 => Some(Chirality::SP3),
                        _ => None,
                    }),
                    Some(c) => Err(ParserError::InvalidChiralitySpec(
                        format!("@S{}", c),
                        self.position,
                    )),
                    None => Err(ParserError::UnexpectedEndOfInput(
                        "chirality class".to_string(),
                    )),
                }
            }
            Some(&'O') => {
                self.next();
                match self.next() {
                    Some('H') => self.parse_chirality_index(1, 30, |n| Chirality::oh(n as u8)),
                    Some(c) => Err(ParserError::InvalidChiralitySpec(
                        format!("@O{}", c),
                        self.position,
                    )),
                    None => Err(ParserError::UnexpectedEndOfInput(
                        "chirality class".to_string(),
                    )),
                }
            }
            _ => Ok(Some(Chirality::TH1)),
        }
    }

    /// Parse a chirality index (1 or 2 digit number) and map it via `f`.
    /// Returns an error if the number is outside `[min, max]` or if `f` returns None.
    fn parse_chirality_index(
        &mut self,
        min: u32,
        max: u32,
        f: impl FnOnce(u32) -> Option<Chirality>,
    ) -> Result<Option<Chirality>, ParserError> {
        let first = self.next().ok_or(ParserError::UnexpectedEndOfInput(
            "chirality index".to_string(),
        ))?;
        let pos = self.position;
        let first_digit = first
            .to_digit(10)
            .ok_or(ParserError::InvalidChiralityClass(first.to_string(), pos))?;

        let n = if let Some(&next_c) = self.peek() {
            if let Some(second_digit) = next_c.to_digit(10) {
                self.next();
                first_digit * 10 + second_digit
            } else {
                first_digit
            }
        } else {
            first_digit
        };

        if n < min || n > max {
            return Err(ParserError::InvalidChiralityClass(
                n.to_string(),
                self.position,
            ));
        }

        f(n).map(Some)
            .ok_or_else(|| ParserError::InvalidChiralityClass(n.to_string(), self.position))
    }

    fn parse_hydrogen(&mut self) -> Result<Option<u8>, ParserError> {
        match self.peek() {
            None => Err(ParserError::UnexpectedEndOfInput("]".to_string())),
            Some(&'H') => {
                self.next();
                let mut builder = String::new();
                while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                    builder.push(self.next().unwrap());
                }
                if builder.is_empty() {
                    Ok(Some(1))
                } else {
                    Ok(Some(
                        builder
                            .parse::<u8>()
                            .map_err(|_| ParserError::HydrogenOutOfRange(builder))?,
                    ))
                }
            }
            _ => Ok(Some(0)),
        }
    }

    fn parse_charge(&mut self) -> Result<i8, ParserError> {
        let mut charge: i8 = 0;
        let mut builder = String::new();
        while self
            .peek()
            .is_some_and(|c| c.is_ascii_digit() || *c == '+' || *c == '-')
        {
            match self.next() {
                Some('+') => charge += 1,
                Some('-') => charge -= 1,
                Some(c) if c.is_ascii_digit() => {
                    builder.push(c);
                    while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                        builder.push(self.next().unwrap());
                    }
                }
                _ => (),
            }
        }

        if self.peek().is_some_and(|c| *c == ':' || *c == ']') {
            if builder.is_empty() {
                return Ok(charge);
            } else if charge > 0 {
                return builder
                    .parse::<i8>()
                    .map_err(|_| ParserError::ChargeOutOfRange(builder));
            } else if charge < 0 {
                return Ok(0 - builder
                    .parse::<i8>()
                    .map_err(|_| ParserError::ChargeOutOfRange(builder))?);
            } else {
                return Err(ParserError::ChargeWithoutSign);
            }
        }

        match self.next() {
            Some(c) => Err(ParserError::UnexpectedCharacter(c, self.position)),
            None => Err(ParserError::UnexpectedEndOfInput("]".to_string())),
        }
    }

    fn connect_current_atom(&mut self) -> Result<(), ParserError> {
        if self.builder.nodes().is_empty() {
            return Err(ParserError::NoAtomToBond);
        }
        let current_atom = self.get_current_atom_index()?;

        if let Some(src) = self.next_bond_source {
            self.add_bond_between(src, current_atom);
        }
        self.next_bond_source = Some(current_atom);
        Ok(())
    }

    fn connect_ring_closure(
        &mut self,
        target: u16,
        bond_type: BondType,
    ) -> Result<(), ParserError> {
        if self.builder.nodes().is_empty() {
            return Err(ParserError::NoAtomToBond);
        }
        let current_atom = self.get_current_atom_index()?;
        self.builder.add_bond(current_atom, target, bond_type);
        Ok(())
    }

    fn get_current_atom_index(&self) -> Result<u16, ParserError> {
        let current_atom: u16 = (self.builder.nodes().len() - 1)
            .try_into()
            .map_err(|_| ParserError::TooManyNodes)?;
        Ok(current_atom)
    }

    fn add_bond_between(&mut self, source: u16, target: u16) {
        // Explicit bonds take priority, otherwise determine implicit bond type
        let bond_type = self.next_bond_type.take().unwrap_or(
            if self.builder.nodes()[source as usize].aromatic() == Some(true)
                && self.builder.nodes()[target as usize].aromatic() == Some(true)
            {
                BondType::Aromatic
            } else {
                BondType::Simple
            },
        );

        self.builder.add_bond(source, target, bond_type);
    }

    /// Check if a bond already exists between two atoms (in either direction).
    fn has_bond_between(&self, a: u16, b: u16) -> bool {
        // Check local bonds
        for bond in self.builder.bonds() {
            let (s, t) = (
                self.node_offset + bond.source(),
                self.node_offset + bond.target(),
            );
            if (s == a && t == b) || (s == b && t == a) {
                return true;
            }
        }
        // Check deferred ring bonds
        for &(target, source, _) in &self.deferred_ring_bonds {
            let s = self.node_offset + source;
            if (s == a && target == b) || (s == b && target == a) {
                return true;
            }
        }
        false
    }
}

pub fn parse(input: &str) -> Result<Molecule, ParserError> {
    let parser = Parser::new(input);
    let (builder, branch_bond_type, next_bond_type, cycles_target, _) = parser.parse()?;

    // Check for unclosed rings at the top level
    if !cycles_target.is_empty() {
        return Err(ParserError::UnclosedRing(
            cycles_target.into_keys().collect(),
        ));
    }

    // Check for bond at start (e.g., "-C") - only invalid at top level
    if branch_bond_type.is_some() {
        return Err(ParserError::BondWithoutPrecedingAtom);
    }

    // Check for dangling bond at end (e.g., "C=")
    // Note: if branch_bond_type was Some, we already returned above
    if next_bond_type.is_some() {
        return Err(ParserError::BondWithoutFollowingAtom);
    }

    Ok(builder.build()?)
}
