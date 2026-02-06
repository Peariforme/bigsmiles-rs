use crate::{
    ast::atom::{Atom, AtomSymbol},
    ast::chirality::Chirality,
    NodeError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    atom: Atom,
    aromatic: bool,
    hydrogens: u8,
    class: Option<u16>,
    chirality: Option<Chirality>,
}

impl Node {
    pub fn new(
        atom: Atom,
        aromatic: bool,
        hydrogens: u8,
        class: Option<u16>,
        chirality: Option<Chirality>,
    ) -> Result<Node, NodeError> {
        if hydrogens > 9 {
            return Err(NodeError::InvalidHydrogen(hydrogens));
        }

        match class {
            None => (),
            Some(value) => {
                if value > 999 {
                    return Err(NodeError::InvalidClass(value));
                }
            }
        }

        if aromatic && !atom.element().can_be_aromatic() {
            return Err(NodeError::InvalidAromaticElement(*atom.element()));
        }

        Ok(Node {
            atom,
            aromatic,
            hydrogens,
            class,
            chirality,
        })
    }

    pub fn atom(&self) -> &Atom {
        &self.atom
    }

    pub fn aromatic(&self) -> bool {
        self.aromatic
    }

    pub fn hydrogens(&self) -> u8 {
        self.hydrogens
    }

    pub fn class(&self) -> Option<u16> {
        self.class
    }

    pub fn chirality(&self) -> Option<Chirality> {
        self.chirality
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeBuilder {
    atom: Atom,
    aromatic: Option<bool>,
    hydrogens: Option<u8>,
    class: Option<u16>,
    chirality: Option<Chirality>,
}

impl NodeBuilder {
    pub fn new(
        element: AtomSymbol,
        charge: i8,
        isotope: Option<u16>,
        aromatic: Option<bool>,
        hydrogens: Option<u8>,
        class: Option<u16>,
        chirality: Option<Chirality>,
    ) -> Result<NodeBuilder, NodeError> {
        let atom = Atom::new(element, charge, isotope)?;

        Ok(NodeBuilder {
            atom,
            aromatic,
            hydrogens,
            class,
            chirality,
        })
    }

    pub fn atom(&self) -> &Atom {
        &self.atom
    }

    pub fn aromatic(&self) -> Option<bool> {
        self.aromatic
    }

    pub fn set_aromatic(&mut self, b: bool) -> &mut Self {
        self.aromatic = Some(b);
        self
    }

    pub fn hydrogens(&self) -> Option<u8> {
        self.hydrogens
    }

    pub fn set_hydrogens(&mut self, h: u8) -> &mut Self {
        self.hydrogens = Some(h);
        self
    }

    pub fn class(&self) -> Option<u16> {
        self.class
    }

    pub fn set_class(&mut self, c: u16) -> &mut Self {
        self.class = Some(c);
        self
    }

    pub fn chirality(&self) -> Option<Chirality> {
        self.chirality
    }

    pub fn build(mut self, bond_order_sum: Option<u8>) -> Result<Node, NodeError> {
        if self.hydrogens.is_none() {
            self.set_hydrogens(self.atom.implicit_hydrogens(bond_order_sum)?);
        }

        Node::new(
            self.atom,
            self.aromatic.ok_or(NodeError::UndefinedAromatic)?,
            self.hydrogens.ok_or(NodeError::UndefinedHydrogen)?,
            self.class,
            self.chirality,
        )
    }
}
