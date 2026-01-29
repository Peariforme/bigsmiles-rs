# bigsmiles-rs

A Rust library for parsing [SMILES](http://opensmiles.org/opensmiles.html) and [BigSMILES](https://pubs.acs.org/doi/10.1021/acscentsci.9b00476) molecular notations.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

**bigsmiles-rs** provides parsers for two related chemical line notations:

- **SMILES** (Simplified Molecular-Input Line-Entry System) - A widely-used notation for representing molecular structures
- **BigSMILES** - An extension of SMILES for describing macromolecules and polymers with stochastic structures

## Features

- 🧪 Full OpenSMILES specification support
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

fn main() -> Result<(), smiles_core::ParseError> {
    // Parse ethanol
    let molecule = parse("CCO")?;
    println!("{:?}", molecule);

    // Parse benzene (aromatic)
    let benzene = parse("c1ccccc1")?;
    
    // Parse with stereochemistry
    let chiral = parse("[C@H](O)(F)Cl")?;
    
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

- [x] Project structure
- [ ] SMILES parser
  - [ ] Organic atoms
  - [ ] Bonds
  - [ ] Branches
  - [ ] Ring closures
  - [ ] Bracket atoms
  - [ ] Stereochemistry
- [ ] BigSMILES parser
  - [ ] Stochastic objects
  - [ ] Bond descriptors
  - [ ] End groups
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
