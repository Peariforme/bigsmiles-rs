use std::collections::HashMap;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

use crate::error::ParserError;
use crate::{AtomSymbol, BondType, Molecule, MoleculeBuilder};

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
    position: usize,
    builder: MoleculeBuilder,
    next_bond_type: Option<BondType>,
    next_bond_source: Option<u16>,
    branch_bond_type: Option<BondType>,
    cycles_target: HashMap<u8, u16>,
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
        }
    }

    fn new_with_offset(input: &'a str, offset: usize) -> Self {
        Parser {
            chars: input.chars().peekable(),
            position: offset,
            builder: MoleculeBuilder::new(),
            next_bond_type: None,
            next_bond_source: None,
            branch_bond_type: None,
            cycles_target: HashMap::new(),
        }
    }

    fn next(&mut self) -> Option<char> {
        self.position += 1;
        self.chars.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn parse(mut self) -> Result<(MoleculeBuilder, Option<BondType>), ParserError> {
        while let Some(c) = self.next() {
            // Atom
            if c.is_ascii_alphabetic() || c == '*' {
                let elem = self.parse_element_symbol(c);
                // Aromaticity is indicated by lowercase letters (c, n, o, etc.)
                // Wildcard '*' outside brackets is non-aromatic by default
                let aromatic = Some(c.is_ascii_lowercase());
                self.builder.add_atom(elem, 0, None, aromatic, None, None)?;
                self.connect_current_atom()?;
            // Brackets Atom
            } else if c == '[' {
                let (elem, charge, isotope, aromatic, hydrogen, class) =
                    self.parse_bracket_atom()?;
                self.builder
                    .add_atom(elem, charge, isotope, aromatic, hydrogen, class)?;
                self.connect_current_atom()?;

            // Explicit bond
            } else if c == '-' || c == '=' || c == '#' || c == '$' || c == '.' || c == ':' {
                self.next_bond_type = Some(BondType::try_from(&c)?);
                if self.builder.nodes().len() == 0 {
                    self.branch_bond_type = self.next_bond_type;
                }

            // Branches
            } else if c == '(' {
                self.parse_branch()?;
            // cycles
            } else if c == '%' || c.is_ascii_digit() {
                let cycle_number: u8;
                if c == '%' {
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
                    cycle_number = first_u8 * 10 + second_u8;
                } else {
                    cycle_number = c.to_digit(10).expect("Unreachable error") as u8;
                }

                // If the key already exists, close the ring
                if let Some(n) = self.cycles_target.get(&cycle_number) {
                    self.connect_ring_closure(*n)?;
                    // Remove the key so it can be reused
                    self.cycles_target.remove(&cycle_number);
                // Otherwise, this is the start of a new ring
                } else {
                    self.cycles_target.insert(
                        cycle_number,
                        self.next_bond_source
                            .ok_or(ParserError::UnexpectedCharacter(c, self.position))?,
                    );
                }
            } else {
                return Err(ParserError::NotYetImplemented);
            }
        }

        if !self.cycles_target.is_empty() {
            return Err(ParserError::UnclosedRing(
                self.cycles_target.into_keys().collect(),
            ));
        }

        Ok((self.builder, self.branch_bond_type))
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
        if s == "" {
            return Err(ParserError::EmptyBranch);
        }
        let branch_parser = Parser::new_with_offset(&s, position);
        let (branch_builder, branch_bond_type) = branch_parser.parse()?;
        let bond_type = branch_bond_type.unwrap_or(BondType::Simple);
        self.builder
            .add_branch(branch_builder, bond_type, self.next_bond_source);
        if self.next_bond_source.is_none() {
            self.next_bond_source = Some(0);
        }
        Ok(())
    }

    fn parse_element_symbol(&mut self, c: char) -> String {
        if c.is_ascii_uppercase() {
            if let Some(&next_c) = self.peek() {
                if next_c.is_ascii_lowercase() {
                    let two_letter = format!("{}{}", c, next_c);
                    if AtomSymbol::from_str(&two_letter).is_ok() {
                        self.next();
                        return two_letter;
                    }
                }
            }
        }

        c.to_string()
    }

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
        ),
        ParserError,
    > {
        let isotope: Option<u16>;
        let hydrogen: Option<u8>;
        let elem: String;
        let charge: i8;
        let class: Option<u16>;
        let aromatic: Option<bool>;

        isotope = self.parse_isotope();

        let first_char = self.next().ok_or(ParserError::UnexpectedEndOfInput(
            "Element identifier".to_string(),
        ))?;
        if !first_char.is_alphabetic() && first_char != '*' {
            return Err(ParserError::MissingElementInBracketAtom);
        }
        elem = self.parse_element_symbol(first_char);

        hydrogen = self.parse_hydrogen()?;

        charge = self.parse_charge()?;

        class = self.parse_class();

        match self.next() {
            Some(']') => (),
            None => return Err(ParserError::UnexpectedEndOfInput("]".to_string())),
            Some(c) => return Err(ParserError::UnexpectedCharacter(c, self.position)),
        }

        aromatic = Some(elem.to_lowercase() == elem);

        Ok((elem, charge, isotope, aromatic, hydrogen, class))
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

    fn parse_hydrogen(&mut self) -> Result<Option<u8>, ParserError> {
        match self.peek() {
            None => return Err(ParserError::UnexpectedEndOfInput("]".to_string())),
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
            } else {
                if charge > 0 {
                    return Ok(builder
                        .parse::<i8>()
                        .map_err(|_| ParserError::ChargeOutOfRange(builder))?);
                } else if charge < 0 {
                    return Ok(0 - builder
                        .parse::<i8>()
                        .map_err(|_| ParserError::ChargeOutOfRange(builder))?);
                } else {
                    return Err(ParserError::ChargeWithoutSign);
                }
            }
        }

        match self.next() {
            Some(c) => return Err(ParserError::UnexpectedCharacter(c, self.position)),
            None => return Err(ParserError::UnexpectedEndOfInput("]".to_string())),
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

    fn connect_ring_closure(&mut self, target: u16) -> Result<(), ParserError> {
        if self.builder.nodes().is_empty() {
            return Err(ParserError::NoAtomToBond);
        }
        let current_atom = self.get_current_atom_index()?;
        self.add_bond_between(current_atom, target);
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
}

pub fn parse(input: &str) -> Result<Molecule, ParserError> {
    let parser = Parser::new(input);
    let (builder, _) = parser.parse()?;
    Ok(builder.build()?)
}
