#[derive(Debug, Clone, PartialEq)]
pub enum AtomError {
    InvalidCharge(i8),   // charge invalide
    InvalidIsotope(u16), // isotope > 999
    UnknownElement(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeError {
    InvalidHydrogen(u8), // Nombre d'hydrogen invalide
    InvalidClass(u16),   // Class invalide
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
    NodeError(NodeError),
    AtomError(AtomError),
}

impl From<NodeError> for ParserError {
    fn from(err: NodeError) -> ParserError {
        ParserError::NodeError(err)
    }
}

impl From<AtomError> for ParserError {
    fn from(err: AtomError) -> ParserError {
        ParserError::AtomError(err)
    }
}
