use std::str::FromStr;

use crate::AtomError;

#[derive(Debug, Clone, PartialEq, Copy)]
#[rustfmt::skip]
pub enum AtomSymbol {
    H, He,
    Li, Be, Ne,
    Na, Mg, Al, Si, Ar,
    K, Ca, Sc, Ti, V, Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Kr,
    Rb, Sr, Y, Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te, Xe,
    Cs, Ba, Lu, Hf, Ta, W, Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn,
    Fr, Ra, Lr, Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og,
    La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, 
    Ac, Th, Pa, U, Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No,
    Wildcard,
    Organic(OrganicAtom)
}

impl FromStr for AtomSymbol {
    type Err = AtomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H" => Ok(AtomSymbol::H),
            "He" => Ok(AtomSymbol::He),
            "Li" => Ok(AtomSymbol::Li),
            "Be" => Ok(AtomSymbol::Be),
            "B" => Ok(AtomSymbol::Organic(OrganicAtom::B)),
            "C" => Ok(AtomSymbol::Organic(OrganicAtom::C)),
            "N" => Ok(AtomSymbol::Organic(OrganicAtom::N)),
            "O" => Ok(AtomSymbol::Organic(OrganicAtom::O)),
            "F" => Ok(AtomSymbol::Organic(OrganicAtom::F)),
            "Ne" => Ok(AtomSymbol::Ne),
            "Na" => Ok(AtomSymbol::Na),
            "Mg" => Ok(AtomSymbol::Mg),
            "Al" => Ok(AtomSymbol::Al),
            "Si" => Ok(AtomSymbol::Si),
            "P" => Ok(AtomSymbol::Organic(OrganicAtom::P)),
            "S" => Ok(AtomSymbol::Organic(OrganicAtom::S)),
            "Cl" => Ok(AtomSymbol::Organic(OrganicAtom::Cl)),
            "Ar" => Ok(AtomSymbol::Ar),
            "K" => Ok(AtomSymbol::K),
            "Ca" => Ok(AtomSymbol::Ca),
            "Sc" => Ok(AtomSymbol::Sc),
            "Ti" => Ok(AtomSymbol::Ti),
            "V" => Ok(AtomSymbol::V),
            "Cr" => Ok(AtomSymbol::Cr),
            "Mn" => Ok(AtomSymbol::Mn),
            "Fe" => Ok(AtomSymbol::Fe),
            "Co" => Ok(AtomSymbol::Co),
            "Ni" => Ok(AtomSymbol::Ni),
            "Cu" => Ok(AtomSymbol::Cu),
            "Zn" => Ok(AtomSymbol::Zn),
            "Ga" => Ok(AtomSymbol::Ga),
            "Ge" => Ok(AtomSymbol::Ge),
            "As" => Ok(AtomSymbol::As),
            "Se" => Ok(AtomSymbol::Se),
            "Br" => Ok(AtomSymbol::Organic(OrganicAtom::Br)),
            "Kr" => Ok(AtomSymbol::Kr),
            "Rb" => Ok(AtomSymbol::Rb),
            "Sr" => Ok(AtomSymbol::Sr),
            "Y" => Ok(AtomSymbol::Y),
            "Zr" => Ok(AtomSymbol::Zr),
            "Nb" => Ok(AtomSymbol::Nb),
            "Mo" => Ok(AtomSymbol::Mo),
            "Tc" => Ok(AtomSymbol::Tc),
            "Ru" => Ok(AtomSymbol::Ru),
            "Rh" => Ok(AtomSymbol::Rh),
            "Pd" => Ok(AtomSymbol::Pd),
            "Ag" => Ok(AtomSymbol::Ag),
            "Cd" => Ok(AtomSymbol::Cd),
            "In" => Ok(AtomSymbol::In),
            "Sn" => Ok(AtomSymbol::Sn),
            "Sb" => Ok(AtomSymbol::Sb),
            "Te" => Ok(AtomSymbol::Te),
            "I" => Ok(AtomSymbol::Organic(OrganicAtom::I)),
            "Xe" => Ok(AtomSymbol::Xe),
            "Cs" => Ok(AtomSymbol::Cs),
            "Ba" => Ok(AtomSymbol::Ba),
            "La" => Ok(AtomSymbol::La),
            "Ce" => Ok(AtomSymbol::Ce),
            "Pr" => Ok(AtomSymbol::Pr),
            "Nd" => Ok(AtomSymbol::Nd),
            "Pm" => Ok(AtomSymbol::Pm),
            "Sm" => Ok(AtomSymbol::Sm),
            "Eu" => Ok(AtomSymbol::Eu),
            "Gd" => Ok(AtomSymbol::Gd),
            "Tb" => Ok(AtomSymbol::Tb),
            "Dy" => Ok(AtomSymbol::Dy),
            "Ho" => Ok(AtomSymbol::Ho),
            "Er" => Ok(AtomSymbol::Er),
            "Tm" => Ok(AtomSymbol::Tm),
            "Yb" => Ok(AtomSymbol::Yb),
            "Lu" => Ok(AtomSymbol::Lu),
            "Hf" => Ok(AtomSymbol::Hf),
            "Ta" => Ok(AtomSymbol::Ta),
            "W" => Ok(AtomSymbol::W),
            "Re" => Ok(AtomSymbol::Re),
            "Os" => Ok(AtomSymbol::Os),
            "Ir" => Ok(AtomSymbol::Ir),
            "Pt" => Ok(AtomSymbol::Pt),
            "Au" => Ok(AtomSymbol::Au),
            "Hg" => Ok(AtomSymbol::Hg),
            "Tl" => Ok(AtomSymbol::Tl),
            "Pb" => Ok(AtomSymbol::Pb),
            "Bi" => Ok(AtomSymbol::Bi),
            "Po" => Ok(AtomSymbol::Po),
            "At" => Ok(AtomSymbol::At),
            "Rn" => Ok(AtomSymbol::Rn),
            "Fr" => Ok(AtomSymbol::Fr),
            "Ra" => Ok(AtomSymbol::Ra),
            "Ac" => Ok(AtomSymbol::Ac),
            "Th" => Ok(AtomSymbol::Th),
            "Pa" => Ok(AtomSymbol::Pa),
            "U" => Ok(AtomSymbol::U),
            "Np" => Ok(AtomSymbol::Np),
            "Pu" => Ok(AtomSymbol::Pu),
            "Am" => Ok(AtomSymbol::Am),
            "Cm" => Ok(AtomSymbol::Cm),
            "Bk" => Ok(AtomSymbol::Bk),
            "Cf" => Ok(AtomSymbol::Cf),
            "Es" => Ok(AtomSymbol::Es),
            "Fm" => Ok(AtomSymbol::Fm),
            "Md" => Ok(AtomSymbol::Md),
            "No" => Ok(AtomSymbol::No),
            "Lr" => Ok(AtomSymbol::Lr),
            "Rf" => Ok(AtomSymbol::Rf),
            "Db" => Ok(AtomSymbol::Db),
            "Sg" => Ok(AtomSymbol::Sg),
            "Bh" => Ok(AtomSymbol::Bh),
            "Hs" => Ok(AtomSymbol::Hs),
            "Mt" => Ok(AtomSymbol::Mt),
            "Ds" => Ok(AtomSymbol::Ds),
            "Rg" => Ok(AtomSymbol::Rg),
            "Cn" => Ok(AtomSymbol::Cn),
            "Nh" => Ok(AtomSymbol::Nh),
            "Fl" => Ok(AtomSymbol::Fl),
            "Mc" => Ok(AtomSymbol::Mc),
            "Lv" => Ok(AtomSymbol::Lv),
            "Ts" => Ok(AtomSymbol::Ts),
            "Og" => Ok(AtomSymbol::Og),
            "*" => Ok(AtomSymbol::Wildcard),
            _ => Err(AtomError::UnknownElement(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum OrganicAtom {
    B,
    C,
    N,
    O,
    P,
    S,
    F,
    Cl,
    Br,
    I,
}

impl OrganicAtom {
    pub fn valence(&self) -> &'static [u8] {
        match self {
            OrganicAtom::B => &[3],
            OrganicAtom::C => &[4],
            OrganicAtom::N => &[3, 5],
            OrganicAtom::O => &[2],
            OrganicAtom::P => &[3, 5],
            OrganicAtom::S => &[2, 4, 6],
            OrganicAtom::F => &[1],
            OrganicAtom::Cl => &[1],
            OrganicAtom::Br => &[1],
            OrganicAtom::I => &[1],
        }
    }

    pub fn implicit_hydrogens(&self, bond_order_sum: u8) -> u8 {
        for v in self.valence() {
            if *v >= bond_order_sum {
                return *v - bond_order_sum;
            }
        }

        0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    element: AtomSymbol,
    charge: i8,           // -15 to +15
    isotope: Option<u16>, // 0 to 999 ou unspecified
}

impl Atom {
    pub fn new(element: AtomSymbol, charge: i8, isotope: Option<u16>) -> Result<Atom, AtomError> {
        if charge < -15 || charge > 15 {
            return Err(AtomError::InvalidCharge(charge));
        }

        match isotope {
            None => (),
            Some(value) => {
                if value > 999 {
                    return Err(AtomError::InvalidIsotope(value));
                }
            }
        }

        Ok(Atom {
            element,
            charge,
            isotope,
        })
    }

    pub fn implicit_hydrogens(&self, bond_order_sum: Option<u8>) -> Result<u8, AtomError> {
        if let AtomSymbol::Organic(organic) = self.element() {
            Ok(organic.implicit_hydrogens(bond_order_sum.ok_or(AtomError::MissingBondOrder)?))
        } else {
            Ok(0)
        }
    }

    pub fn is_organic(&self) -> bool {
        matches!(self.element, AtomSymbol::Organic(_))
    }

    pub fn element(&self) -> &AtomSymbol {
        &self.element
    }

    pub fn charge(&self) -> i8 {
        self.charge
    }

    pub fn isotope(&self) -> Option<u16> {
        self.isotope
    }
}