# bigsmiles-rs

A Rust library for parsing [SMILES](http://opensmiles.org/opensmiles.html) and [BigSMILES](https://pubs.acs.org/doi/10.1021/acscentsci.9b00476) molecular notations.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/Peariforme/bigsmiles-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Peariforme/bigsmiles-rs/actions/workflows/ci.yml)

## Overview

**bigsmiles-rs** provides parsers for two related chemical line notations:

- **SMILES** (Simplified Molecular-Input Line-Entry System) - A widely-used notation for representing molecular structures
- **BigSMILES** - An extension of SMILES for describing macromolecules and polymers with stochastic structures

## Features

- 🧪 OpenSMILES specification support (stereochemistry in progress)
- 🔬 BigSMILES extensions for polymer chemistry
- 🦀 Pure Rust, no dependencies on external chemistry libraries
- 📝 Detailed error messages with position information

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
smiles-core = "0.1"
# Or for BigSMILES support:
bigsmiles-core = "0.1"
```

## Usage

### Parsing SMILES

```rust
use smiles_core::parse;

fn main() -> Result<(), smiles_core::ParserError> {
    // Parse ethanol
    let molecule = parse("CCO")?;

    // Access atoms and bonds
    for node in molecule.nodes() {
        println!("Element: {}, Hydrogens: {}, Aromatic: {}",
            node.atom().element(),
            node.hydrogens(),
            node.aromatic());
    }

    for bond in molecule.bonds() {
        println!("Bond: {} -> {} ({:?})",
            bond.source(),
            bond.target(),
            bond.kind());
    }

    Ok(())
}
```

### Parsing BigSMILES

```rust
use bigsmiles_core::parse;

fn main() -> Result<(), bigsmiles_core::ParseError> {
    // Parse polystyrene
    let polystyrene = parse("{[]CC(c1ccccc1)[]}")?;

    // Parse a copolymer
    let copolymer = parse("{[$]CC[$],[$]CC(C)[$]}")?;

    Ok(())
}
```

## Public API

### Core Types

#### `parse(smiles: &str) -> Result<Molecule, ParserError>`
Main entry point to parse a SMILES string into a molecular graph.

#### `Molecule`
Represents a parsed molecular structure.

| Method | Return Type | Description |
|--------|-------------|-------------|
| `nodes()` | `&[Node]` | All atoms in the molecule |
| `bonds()` | `&[Bond]` | All bonds between atoms |

#### `Node`
Represents an atom with its properties.

| Method | Return Type | Description |
|--------|-------------|-------------|
| `atom()` | `&Atom` | The underlying atom data |
| `aromatic()` | `bool` | Whether the atom is aromatic |
| `hydrogens()` | `u8` | Number of hydrogen atoms (0-9) |
| `class()` | `Option<u16>` | Atom class label (0-999) |

#### `Atom`
Core atom properties.

| Method | Return Type | Description |
|--------|-------------|-------------|
| `element()` | `&AtomSymbol` | Element symbol |
| `charge()` | `i8` | Formal charge (-15 to +15) |
| `isotope()` | `Option<u16>` | Mass number (0-999) |

#### `Bond`
Represents a bond between two atoms.

| Method | Return Type | Description |
|--------|-------------|-------------|
| `kind()` | `&BondType` | Type of bond |
| `source()` | `u16` | Index of source atom |
| `target()` | `u16` | Index of target atom |

#### `BondType`
```rust
pub enum BondType {
    Simple,      // -  (single bond)
    Double,      // =  (double bond)
    Triple,      // #  (triple bond)
    Quadruple,   // $  (quadruple bond)
    Aromatic,    // :  (aromatic bond)
    Disconnected // .  (no bond, separate fragments)
}
```

#### `AtomSymbol`
Supports all elements of the periodic table, plus:
- `Organic(OrganicAtom)` - B, C, N, O, P, S, F, Cl, Br, I (implicit hydrogens)
- `Wildcard` - `*` (matches any atom)

## Examples

### SMILES Notation

| SMILES | Molecule |
|--------|----------|
| `C` | Methane |
| `CC` | Ethane |
| `CCO` | Ethanol |
| `C=O` | Formaldehyde |
| `C#N` | Hydrogen cyanide |
| `C1CCCCC1` | Cyclohexane |
| `c1ccccc1` | Benzene |
| `CC(=O)O` | Acetic acid |
| `[Na+]` | Sodium ion |
| `[Cu+2]` | Copper(II) ion |
| `[13CH4]` | Carbon-13 methane |
| `[CH3:1]` | Methyl with class label |
| `*CC*` | Wildcard chain |

### BigSMILES Notation

| BigSMILES | Polymer |
|-----------|---------|
| `{[]CC[]}` | Polyethylene |
| `{[]CC(c1ccccc1)[]}` | Polystyrene |
| `{[$]CC[$],[$]CC(C)[$]}` | Ethylene-propylene copolymer |
| `CC{[$]CC[$]}CC` | α,ω-dimethyl polyethylene |

## Documentation

- [OpenSMILES Specification](http://opensmiles.org/opensmiles.html)
- [BigSMILES Paper](https://pubs.acs.org/doi/10.1021/acscentsci.9b00476) - Lin, T.-S. et al. ACS Central Science 2019
- [BigSMILES Documentation](https://olsenlabmit.github.io/BigSMILES/docs/line_notation.html)
- [API Documentation](https://docs.rs/bigsmiles-rs) (coming soon)

## Project Structure

```
bigsmiles-rs/
├── crates/
│   ├── smiles-core/      # Core SMILES parser
│   └── bigsmiles-core/   # BigSMILES extensions
└── tests/                # Integration tests
```

## Roadmap

### SMILES Parser
- [x] Organic atoms (B, C, N, O, P, S, F, Cl, Br, I)
- [x] Bracket atoms `[...]`
  - [x] All periodic table elements
  - [x] Isotopes `[13C]`, `[2H]`
  - [x] Charges `[NH4+]`, `[O-2]`
  - [x] Explicit hydrogens `[CH4]`
  - [x] Atom classes `[CH3:1]`
- [x] Bonds
  - [x] Single `-`
  - [x] Double `=`
  - [x] Triple `#`
  - [x] Quadruple `$`
  - [x] Aromatic `:`
- [x] Branches `()`
- [x] Ring closures (single digit and `%nn`)
- [x] Aromatic atoms (lowercase)
- [x] Wildcard `*`
- [x] Disconnected structures `.`
- [ ] Stereochemistry
  - [ ] Tetrahedral chirality `@`, `@@`
  - [ ] Double bond geometry `/`, `\`
- [ ] `to_smiles()` - Convert molecule back to SMILES string

### BigSMILES Parser
- [ ] Stochastic objects `{...}`
- [ ] Bond descriptors `[$]`, `[<]`, `[>]`
- [ ] End groups

### Bindings
- [ ] WebAssembly bindings
- [ ] Python bindings

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## References

1. Weininger, D. "SMILES, a chemical language and information system." J. Chem. Inf. Comput. Sci. 1988, 28, 31-36.
2. Lin, T.-S. et al. "BigSMILES: A Structurally-Based Line Notation for Describing Macromolecules." ACS Central Science 2019, 5, 1523-1531.
3. [OpenSMILES Specification](http://opensmiles.org/opensmiles.html)
4. [Depth-First: SMILES Formal Grammar](https://depth-first.com/articles/2020/12/21/smiles-formal-grammar-revisited/)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Richard Huot - Polymer chemist & software developer

---

*This project was created as a learning exercise in Rust programming while applying domain knowledge in polymer chemistry.*
