use crate::{BondType, MoleculeBuilder, Molecule};
use crate::error::ParserError;


pub fn parse(input: &str) -> Result<Molecule, ParserError> {
    let mut builder = MoleculeBuilder::new();

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
            builder.add_atom(
                elem,
                0,
                None,
                Some(false),
                None,
                None,
            )?;

            // Si on a déja un noeud alors il faut faire un bond
            if builder.nodes().len() > 1 {
                let target: u16 = (builder.nodes().len() - 1).try_into().map_err(|_| ParserError::TooManyNodes)?;
                builder.add_bond(
                    target - 1,
                     target, // on reconverti en index de nodes pour faciler retranscrir dans le futur
                    BondType::Simple
                );
            }
        } else {
            return Err(ParserError::NotYetImplemented)
        }
        // cas 3 minuscule
    }

    Ok(builder.build()?)
}
