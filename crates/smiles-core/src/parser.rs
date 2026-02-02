use crate::{BondType, MoleculeBuilder, Molecule};
use crate::error::ParserError;


fn parse_internal(input: &str) -> Result<(MoleculeBuilder, Option<BondType>), ParserError> {
    let mut builder = MoleculeBuilder::new();

    let mut chars = input.chars().peekable();
    let mut next_bond_type: Option<BondType> = None;
    let mut next_bond_source: Option<u16> = None;
    let mut branch_bond_type: Option<BondType> = None;

    while let Some(c) = chars.next() {
        // Organic Atom
        if c.is_ascii_uppercase() {
            let mut elem = String::new();
            elem.push(c);
            match chars.peek() {
                None => (),
                Some(value) => {
                    if value.is_ascii_lowercase() {
                        elem.push(*value);
                        chars.next();
                    }
                }
            }
            builder.add_atom(
                elem,
                0,
                None,
                Some(false),
                None,
                None,
            )?;

            let current_atom: u16 = (builder.nodes().len() - 1).try_into().map_err(|_| ParserError::TooManyNodes)?;
            // Create the bond only if there is a preceding atom
            if let Some(src) = next_bond_source {
                builder.add_bond(
                    src,
                    current_atom,
                    next_bond_type.take().unwrap_or(BondType::Simple)
                );
                
            }

            next_bond_source = Some(current_atom);
        // Explicit bond
        } else if c == '-' || c == '=' || c == '#' || c == '$' {
            next_bond_type = Some(BondType::try_from(&c)?);
            if builder.nodes().len() == 0 {
                branch_bond_type = next_bond_type;
            }
            
        // Branches
        } else if c == '(' {
            let mut s= String::new();
            let mut parenthesis_count: i8 = 1;
            while let Some(c) = chars.next() {
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

            let (branch_builder, branch_bond_type) = parse_internal(&s)?;
            let bond_type = branch_bond_type.unwrap_or(BondType::Simple);
            builder.add_branch(branch_builder, bond_type, next_bond_source);

            if next_bond_source.is_none() {
                next_bond_source = Some(0);
            }
        } else {
            return Err(ParserError::NotYetImplemented)
        }
        // cas 3 minuscule
    }

    Ok((builder, branch_bond_type))
}

pub fn parse(input: &str) -> Result<Molecule, ParserError> {
    let (builder, _) = parse_internal(input)?;
    Ok(builder.build()?)
}
