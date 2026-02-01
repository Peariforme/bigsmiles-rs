#[derive(Debug, Clone, PartialEq)]
pub enum AtomError {
    InvalidCharge(i8),   // charge invalide
    InvalidIsotope(u16), // isotope > 999
    MissingBondOrder,
    UnknownElement(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeError {
    InvalidHydrogen(u8), // Nombre d'hydrogen invalide
    UndefinedHydrogen,
    InvalidClass(u16),   // Class invalide
    UndefinedAromatic,
    BondOrderMandatoryForOrganicAtom,
    AtomError(AtomError),
}

impl From<AtomError> for NodeError {
    fn from(err: AtomError) -> NodeError {
        NodeError::AtomError(err)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    NotYetImplemented,
    TooManyNodes, // plus de 65535 noeud trouvé
    MoleculeError(MoleculeError),
    NodeError(NodeError)
}

impl From<MoleculeError> for ParserError {
    fn from(err: MoleculeError) -> ParserError {
        ParserError::MoleculeError(err)
    }
}

impl From<NodeError> for ParserError {
    fn from(err: NodeError) -> ParserError {
        ParserError::NodeError(err)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoleculeError {
    NodeError(NodeError),
    AtomError(AtomError)
}

impl From<NodeError> for MoleculeError {
    fn from(err: NodeError) -> MoleculeError {
        MoleculeError::NodeError(err)
    }
}

impl From<AtomError> for MoleculeError {
    fn from(err: AtomError) -> MoleculeError {
        MoleculeError::AtomError(err)
    }
}