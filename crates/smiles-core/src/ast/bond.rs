use crate::BondError;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BondType {
    Simple,
    Double,
    Triple,
    Quadruple,
    Aromatic,
    Disconnected
}

impl TryFrom<&char> for BondType {
    type Error = BondError;

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        match c {
            '-' => Ok(BondType::Simple),
            '=' => Ok(BondType::Double),
            '#' => Ok(BondType::Triple),
            '$' => Ok(BondType::Quadruple),
            '.' => Ok(BondType::Disconnected),
            ':' => Ok(BondType::Aromatic),
            _ => Err(BondError::UnknownBond(*c)),
        }
    }
}

impl BondType {
    pub fn electrons_involved(&self) -> u8 {
        match self {
            BondType::Simple => 2,
            BondType::Double => 4,
            BondType::Triple => 6,
            BondType::Quadruple => 8,
            BondType::Aromatic => 3,
            BondType::Disconnected => 0
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bond {
    kind: BondType,
    source: u16,
    target: u16,
}

impl Bond {
    pub fn new(kind: BondType, source: u16, target: u16) -> Bond {
        Bond {
            kind,
            source,
            target,
        }
    }

    pub fn kind(&self) -> &BondType {
        &self.kind
    }

    pub fn source(&self) -> u16 {
        self.source
    }

    pub fn target(&self) -> u16 {
        self.target
    }
}