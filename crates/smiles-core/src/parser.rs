use crate::error::ParserError;
use crate::{BondType, Molecule, MoleculeBuilder};

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
            builder.add_atom(elem, 0, None, Some(false), None, None)?;

            let current_atom: u16 = (builder.nodes().len() - 1)
                .try_into()
                .map_err(|_| ParserError::TooManyNodes)?;
            // Create the bond only if there is a preceding atom
            if let Some(src) = next_bond_source {
                builder.add_bond(
                    src,
                    current_atom,
                    next_bond_type.take().unwrap_or(BondType::Simple),
                );
            }

            next_bond_source = Some(current_atom);
        // Brackets Atom
        } else if c == '[' {
            let mut closed_bracket = false;
            let mut isotope_calculated = false;
            let mut isotope: Option<u16> = None;
            let mut isotope_builder = String::new();
            let mut calculating_hydrogen = false;
            let mut hydrogen: Option<u8> = None;
            let mut hydrogen_builder = String::new();
            let mut elem = String::new();
            let mut elem_calculated = false;
            let mut charge: i8 = 0;
            let mut charge_builder = String::new();
            let mut calculating_charge = false;
            let mut class: Option<u16> = None;
            let mut class_builder = String::new();
            let mut calculating_class = false;
            while let Some(c) = chars.next() {
                // Calcul de l'isotope
                if c.is_numeric() && isotope.is_none() && !isotope_calculated {
                    isotope_builder.push(c);
                }

                // Calcul de l'élément
                if c.is_alphabetic() && !elem_calculated {
                    isotope = isotope_builder.parse::<u16>().ok();
                    isotope_calculated = true;

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
                    elem_calculated = true;
                }

                // Calcul des hydrogènes
                if c == 'H' && elem_calculated {
                    hydrogen = Some(1);
                    calculating_hydrogen = true;
                }

                if c.is_numeric() && calculating_hydrogen {
                    hydrogen_builder.push(c);
                }

                // Calcul de la charge
                if c == '+' || c == '-' {
                    calculating_hydrogen = false;
                    calculating_charge = true;
                    match c {
                        '+' => charge += 1,
                        '-' => charge -= 1,
                        _ => ()
                    }
                }

                if c.is_numeric() && calculating_charge {
                    charge_builder.push(c);
                }

                // Calcul de la classe
                if c == ':' {
                    calculating_hydrogen = false;
                    calculating_charge = false;
                    calculating_class = true;
                }

                if c.is_numeric() && calculating_class {
                    class_builder.push(c);
                }

                if c == ']' {
                    if hydrogen.is_none() {
                        hydrogen = Some(0);
                    }

                    if hydrogen_builder != "" {
                        hydrogen = Some(hydrogen_builder.parse::<u8>().map_err(|_| ParserError::HydrogenOutOfRange(hydrogen_builder))?);
                    }

                    if charge_builder != "" {
                        if charge > 0 {
                            charge = charge_builder.parse::<i8>().map_err(|_| ParserError::ChargeOutOfRange(charge_builder))?;
                        } else if charge < 0 {
                            charge = 0 - charge_builder.parse::<i8>().map_err(|_| ParserError::ChargeOutOfRange(charge_builder))?;
                        }
                    }
                    class = class_builder.parse::<u16>().ok();
                    closed_bracket = true;
                    break;
                }
            }

            if !closed_bracket {
                return Err(ParserError::UnclosedBracket);
            }

            builder.add_atom(elem, charge, isotope, Some(false), hydrogen, class)?;

            let current_atom: u16 = (builder.nodes().len() - 1)
                .try_into()
                .map_err(|_| ParserError::TooManyNodes)?;
            // Create the bond only if there is a preceding atom
            if let Some(src) = next_bond_source {
                builder.add_bond(
                    src,
                    current_atom,
                    next_bond_type.take().unwrap_or(BondType::Simple),
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
            let mut s = String::new();
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
            return Err(ParserError::NotYetImplemented);
        }
        // cas 3 minuscule
    }

    Ok((builder, branch_bond_type))
}

pub fn parse(input: &str) -> Result<Molecule, ParserError> {
    let (builder, _) = parse_internal(input)?;
    Ok(builder.build()?)
}
