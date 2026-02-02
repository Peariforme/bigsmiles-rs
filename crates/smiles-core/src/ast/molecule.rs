use crate::{MoleculeError, NodeError, ast::{atom::AtomSymbol, bond::{Bond, BondType}, node::{Node, NodeBuilder}}};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Molecule {
    nodes: Vec<Node>,
    bonds: Vec<Bond>,
}

impl Molecule {
    pub fn new(nodes: Vec<Node>, bonds: Vec<Bond>) -> Molecule {
        Molecule { nodes, bonds }
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn bonds(&self) -> &[Bond] {
        &self.bonds
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoleculeBuilder {
    nodes: Vec<NodeBuilder>,
    bonds: Vec<Bond>,
}

impl MoleculeBuilder {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            bonds: Vec::new(),
        }
    }

    pub fn nodes(&self) -> &[NodeBuilder] {
        &self.nodes
    }

    pub fn bonds(&self) -> &[Bond] {
        &self.bonds
    }

    pub fn add_atom(
        &mut self,
        element: String,
        charge: i8,
        isotope: Option<u16>,
        aromatic: Option<bool>,
        hydrogens: Option<u8>,
        class: Option<u16>,
    ) -> Result<usize, NodeError> {
        self.nodes
            .push(NodeBuilder::new(AtomSymbol::from_str(&element)?, charge, isotope, aromatic, hydrogens, class)?);
        Ok(self.nodes.len() - 1)
    }

    pub fn add_branch(&mut self, m: MoleculeBuilder, bond_type: BondType, source: Option<u16>) {
        let node_count = self.nodes.len() as u16;
        if let Some(src) = source {
            self.add_bond( src, node_count, bond_type);
        }
        self.nodes.extend(m.nodes);
        for bond in m.bonds {
            self.add_bond(node_count + bond.source() + 1, node_count + bond.target() + 1, *bond.kind());
        }
    }

    pub fn add_bond(&mut self, source: u16, target: u16, kind: BondType) {
        self.bonds.push(Bond::new(kind, source, target));
    }

    // À la fin
    pub fn build(self) -> Result<Molecule, MoleculeError> {
        let mut nodes: Vec<Node> = Vec::new(); 
        let mut bond_orders_x2 = vec![0u8; self.nodes.len()];

        for bond in &self.bonds {
            bond_orders_x2[bond.source() as usize] += bond.kind().electrons_involved();
            bond_orders_x2[bond.target() as usize] += bond.kind().electrons_involved();
        }
        

        for (index, node) in self.nodes.into_iter().enumerate() {
            nodes.push(node.build(Some(bond_orders_x2[index] / 2))?);
        }

        Ok(Molecule {
            nodes,
            bonds: self.bonds
        })
    }
}