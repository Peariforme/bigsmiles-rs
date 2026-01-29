use crate::BondType;
use crate::ast::{AtomSymbol, Bond, Molecule, Node};
use crate::error::ParserError;
use std::str::FromStr;

pub fn parse(input: &str) -> Result<Molecule, ParserError> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut bonds: Vec<Bond> = Vec::new();

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        //lire le premier char
        // cas 1 [
        // cas 2 Lettre majuscule
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
            nodes.push(Node::new(
                AtomSymbol::from_str(&elem)?,
                0,
                None,
                false,
                4,
                None,
            )?);

            // Si on a déja un noeud alors il faut faire un bond
            if nodes.len() > 1 {
                let target: u16 = (nodes.len() - 1).try_into().map_err(|_| ParserError::TooManyNodes)?;
                bonds.push(Bond::new(
                    BondType::Simple,
                     target - 1, // on reconverti en index de nodes pour faciler retranscrir dans le futur
                    target
                ));
            }
        } else {
            return Err(ParserError::NotYetImplemented)
        }
        // cas 3 minuscule
    }

    Ok(Molecule::new(nodes, bonds))
}
