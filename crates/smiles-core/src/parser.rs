use std::iter::Peekable;
use std::str::Chars;

use crate::error::ParserError;
use crate::{BondType, Molecule, MoleculeBuilder};

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
    position: usize,
    builder: MoleculeBuilder,
    next_bond_type: Option<BondType>,
    next_bond_source: Option<u16>,
    branch_bond_type: Option<BondType>,
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
            // Organic Atom
            if c.is_ascii_uppercase() {
                let elem = self.parse_element_symbol(c);
                self.builder
                    .add_atom(elem, 0, None, Some(false), None, None)?;
                self.connect_current_atom()?;
            // Brackets Atom
            } else if c == '[' {
                let (elem, charge, isotope, aromatic, hydrogen, class) =
                    self.parse_bracket_atom()?;
                self.builder
                    .add_atom(elem, charge, isotope, aromatic, hydrogen, class)?;
                self.connect_current_atom()?;

            // Explicit bond
            } else if c == '-' || c == '=' || c == '#' || c == '$' {
                self.next_bond_type = Some(BondType::try_from(&c)?);
                if self.builder.nodes().len() == 0 {
                    self.branch_bond_type = self.next_bond_type;
                }

            // Branches
            } else if c == '(' {
                self.parse_branch()?;
            } else {
                return Err(ParserError::NotYetImplemented);
            }
            // cas 3 minuscule
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
        let mut elem = String::new();
        elem.push(c);
        if self.peek().is_some_and(|value| value.is_ascii_lowercase()) {
            elem.push(self.next().expect("unreachable error"));
        }
        elem
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

        isotope = self.parse_isotope();

        let first_char = self.next().ok_or(ParserError::UnexpectedEndOfInput(
            "Element identifier".to_string(),
        ))?;
        if !first_char.is_alphabetic() {
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

        Ok((elem, charge, isotope, Some(false), hydrogen, class))
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

        let current_atom: u16 = (self.builder.nodes().len() - 1)
            .try_into()
            .map_err(|_| ParserError::TooManyNodes)?;
        if let Some(src) = self.next_bond_source {
            self.builder.add_bond(
                src,
                current_atom,
                self.next_bond_type.take().unwrap_or(BondType::Simple),
            );
        }
        self.next_bond_source = Some(current_atom);
        Ok(())
    }
}

pub fn parse(input: &str) -> Result<Molecule, ParserError> {
    let parser = Parser::new(input);
    let (builder, _) = parser.parse()?;
    Ok(builder.build()?)
}
