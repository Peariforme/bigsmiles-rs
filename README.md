# bigsmiles-rs

A Rust library for parsing [SMILES](http://opensmiles.org/opensmiles.html) and [BigSMILES](https://pubs.acs.org/doi/10.1021/acscentsci.9b00476) molecular notations.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/Peariforme/bigsmiles-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Peariforme/bigsmiles-rs/actions/workflows/ci.yml)
[![Benchmark](https://github.com/Peariforme/bigsmiles-rs/actions/workflows/benchmark.yml/badge.svg)](https://peariforme.github.io/bigsmiles-rs/dev/bench/)
[![Benchmark Comparison](https://img.shields.io/badge/benchmarks-comparison-blue)](https://peariforme.github.io/bigsmiles-rs/dev/bench/compare.html)

## Overview

**bigsmiles-rs** provides parsers for two related chemical line notations:

- **SMILES** (Simplified Molecular-Input Line-Entry System) - A widely-used notation for representing molecular structures
- **BigSMILES** - An extension of SMILES for describing macromolecules and polymers with stochastic structures

## Features

- 🧪 Full OpenSMILES specification support (see [compliance status](#opensmiles-compliance) below)
- 🔬 BigSMILES extensions for polymer chemistry
- 🦀 Pure Rust, no dependencies on external chemistry libraries
- 📝 Detailed error messages with position information
- ⚡ Optional parallel batch parsing with Rayon (~4x speedup on large datasets)
- 🧬 Optional Hückel's rule aromaticity validation (4n+2 π-electron check)

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

### Parallel Batch Parsing

For processing large datasets, enable the `parallel` feature for multi-threaded parsing:

```toml
[dependencies]
smiles-core = { version = "0.1", features = ["parallel"] }
```

```rust
use smiles_core::parser_parallel::{parse_batch, parse_batch_with_stats};

fn main() {
    let smiles_dataset = vec!["CCO", "c1ccccc1", "CC(=O)O", /* ... thousands more */];

    // Parse all molecules in parallel
    let results = parse_batch(&smiles_dataset);

    // Or get detailed statistics
    let (molecules, errors, stats) = parse_batch_with_stats(&smiles_dataset);
    println!("Parsed {}/{} molecules ({:.1}% success rate)",
        stats.success_count, stats.total_count, stats.success_rate());
}
```

**Performance** (benchmark results on 4-core CPU):

| Batch Size | Sequential | Parallel | Speedup |
|------------|-----------|----------|---------|
| 100        | 76 µs     | 169 µs   | 0.45x   |
| 1,000      | 877 µs    | 396 µs   | **2.2x**  |
| 10,000     | 8.6 ms    | 2.2 ms   | **3.9x**  |

> Note: For small batches (<500), sequential parsing is faster due to thread synchronization overhead.

See the full [benchmark comparison dashboard](https://peariforme.github.io/bigsmiles-rs/dev/bench/compare.html) for interactive charts including sequential vs parallel crossover, linear vs branched chain scaling, Hückel validation overhead, and reference molecule comparisons.

### Aromaticity Validation

Enable the `huckel-validation` feature to reject aromatic SMILES that violate Hückel's rule (4n+2 π electrons):

```toml
[dependencies]
smiles-core = { version = "0.1", features = ["huckel-validation"] }
```

With this feature enabled, `parse()` will return `MoleculeError::HuckelViolation` for chemically invalid aromatic rings. Without the feature, the validation API is still available for explicit use:

```rust
use smiles_core::{parse, ast::aromaticity::validate_aromaticity};

let mol = parse("c1ccccc1")?;  // benzene
let checks = validate_aromaticity(&mol);
assert!(checks[0].is_valid);               // 6 π-electrons → 4(1)+2 ✓
assert_eq!(checks[0].pi_electrons, Some(6));
```

### Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `parallel` | off | Multi-threaded batch parsing with Rayon |
| `huckel-validation` | off | Reject invalid aromatic rings in `parse()` |

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
    Simple,       // -  (single bond)
    Double,       // =  (double bond)
    Triple,       // #  (triple bond)
    Quadruple,    // $  (quadruple bond)
    Aromatic,     // :  (aromatic bond)
    Disconnected, // .  (no bond, separate fragments)
    Up,           // /  (directional bond, cis/trans)
    Down,         // \  (directional bond, cis/trans)
}
```

#### `AtomSymbol`
Supports all elements of the periodic table, plus:
- `Organic(OrganicAtom)` - B, C, N, O, P, S, F, Cl, Br, I (implicit hydrogens)
- `Wildcard` - `*` (matches any atom)

### Parallel API (requires `parallel` feature)

#### `parse_batch(inputs: &[&str]) -> Vec<Result<Molecule, ParserError>>`
Parse multiple SMILES strings in parallel, preserving order.

#### `parse_batch_ok(inputs: &[&str]) -> Vec<Molecule>`
Parse multiple SMILES, returning only successful results (errors silently skipped).

#### `parse_batch_indexed(inputs: &[&str]) -> Vec<(usize, Result<Molecule, ParserError>)>`
Parse with index tracking, useful for identifying which inputs failed.

#### `parse_batch_with_stats(inputs: &[&str]) -> (Vec<Molecule>, Vec<(usize, ParserError)>, BatchParseStats)`
Parse with full error tracking and statistics.

#### `BatchParseStats`
| Field | Type | Description |
|-------|------|-------------|
| `success_count` | `usize` | Number of successfully parsed molecules |
| `error_count` | `usize` | Number of failed parses |
| `total_count` | `usize` | Total number of inputs |
| `success_rate()` | `f64` | Success percentage (0.0 to 100.0) |

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

## OpenSMILES Compliance

Detailed compliance status against the [OpenSMILES specification](http://opensmiles.org/opensmiles.html) v1.0.

### Section 3.1 — Atoms

| Feature | Status | Notes |
|---------|--------|-------|
| Atomic symbols (114 elements) | ✅ Done | All IUPAC elements supported in bracket notation |
| Organic subset (B, C, N, O, P, S, F, Cl, Br, I) | ✅ Done | Implicit hydrogens computed from valence rules |
| Wildcard `*` | ✅ Done | Bare and bracketed `[*]`, can be aromatic |
| Bracket atom syntax `[...]` | ✅ Done | Full `[isotope? symbol chiral? hcount? charge? class?]` |
| Isotopes `[13C]`, `[2H]` | ✅ Done | Range 0-999, leading zeros handled, `[0S] != [S]` |
| Charges `[NH4+]`, `[O-2]` | ✅ Done | Range -15 to +15 |
| Deprecated charges `[Cu++]`, `[O--]` | ✅ Done | Accepted for backwards compatibility |
| Explicit hydrogens `[CH4]` | ✅ Done | Range H0-H9 |
| Atom classes `[CH3:1]` | ✅ Done | Range 0-999 |
| Implicit hydrogen calculation | ✅ Done | Correct valence tables for organic subset |
| `[HH1]` rejection | ✅ Done | Hydrogen atom with hydrogen count is rejected |
| Aromatic two-letter symbols `[se]`, `[as]` | ✅ Done | Lowercase two-letter aromatic symbols parsed in brackets |

### Section 3.2 — Bonds

| Feature | Status | Notes |
|---------|--------|-------|
| Single `-` | ✅ Done | Explicit and implicit |
| Double `=` | ✅ Done | |
| Triple `#` | ✅ Done | |
| Quadruple `$` | ✅ Done | |
| Aromatic `:` | ✅ Done | Auto-detected between aromatic atoms |
| Directional `/` and `\` | ✅ Done | For cis/trans double bond geometry |
| Implicit bond (adjacency) | ✅ Done | Single between aliphatic, aromatic between aromatic atoms |

### Section 3.3 — Branches

| Feature | Status | Notes |
|---------|--------|-------|
| Branch syntax `()` | ✅ Done | Unlimited nesting depth |
| Nested/stacked branches | ✅ Done | |
| Bond type in branch `(=O)` | ✅ Done | |
| Dot in branch `(.C)` | ✅ Done | |

### Section 3.4 — Rings

| Feature | Status | Notes |
|---------|--------|-------|
| Single-digit ring closures (0-9) | ✅ Done | Including ring number 0 |
| Two-digit ring closures `%nn` (10-99) | ✅ Done | |
| `%01` matches `1` (number-based matching) | ✅ Done | Ring digits interpreted as numbers |
| Bond at ring open `C=1CCCCC1` | ✅ Done | |
| Bond at ring close `C1CCCCC=1` | ✅ Done | |
| Bond on both sides (must match) | ✅ Done | Mismatched ring bonds rejected |
| Ring number reuse | ✅ Done | Numbers freed after pairing |
| Spiro atoms (multiple rnums) | ✅ Done | |
| Unclosed ring detection | ✅ Done | |
| Self-bond rejection `C11` | ✅ Done | Atom cannot be bonded to itself |
| Duplicate bond rejection `C12CCCCC12` | ✅ Done | Two atoms cannot be joined by more than one bond |

### Section 3.5 — Aromaticity

| Feature | Status | Notes |
|---------|--------|-------|
| Lowercase aromatic atoms `c`, `n`, `o`, `s`, `p`, `b` | ✅ Done | |
| Aromatic bond detection | ✅ Done | Implicit between adjacent aromatic atoms |
| Kekule form acceptance | ✅ Done | Uppercase + explicit double bonds |
| Elements that can be aromatic | ✅ Done | C, N, O, S, P, B, Se, As, Te, `*` |
| Aromatic `[se]`, `[as]` bracket notation | ✅ Done | Two-letter lowercase aromatic symbols parsed |
| **Hückel's rule validation** | ✅ Done | Opt-in via `huckel-validation` feature; ring detection + π-electron counting + 4n+2 check |

### Stereochemistry (Section not yet in final spec)

| Feature | Status | Notes |
|---------|--------|-------|
| Tetrahedral `@`, `@@` | ✅ Done | `@TH1`, `@TH2` explicit forms too |
| Allenal `@AL1`, `@AL2` | ✅ Done | |
| Square planar `@SP1`-`@SP3` | ✅ Done | |
| Trigonal bipyramidal `@TB1`-`@TB20` | ✅ Done | |
| Octahedral `@OH1`-`@OH30` | ✅ Done | |
| Double bond geometry `/`, `\` | ✅ Done | Cis/trans encoding |

### General Grammar / Parsing

| Feature | Status | Notes |
|---------|--------|-------|
| Empty SMILES string | ✅ Done | Returns empty molecule |
| Whitespace terminator | ✅ Done | SPACE/TAB/LF/CR terminates parsing; trailing content ignored |
| Disconnected structures `.` | ✅ Done | Dot resets chain without creating a bond (per spec, `dot` is not a `bond`) |

### Summary

| Category | Compliant | Missing |
|----------|-----------|---------|
| Atoms | 12 | 0 |
| Bonds | 7 | 0 |
| Branches | 4 | 0 |
| Rings | 11 | 0 |
| Aromaticity | 6 | 0 |
| Stereochemistry | 6 | 0 |
| Grammar | 3 | 0 |
| **Total** | **49** | **0** |

## Roadmap

### SMILES Parser — Remaining OpenSMILES Issues
- [x] Parse aromatic two-letter bracket symbols `[se]`, `[as]`, `[te]`
- [x] Reject `[HH1]` — hydrogen atom with hydrogen count
- [x] Reject self-bonds `C11` — atom bonded to itself
- [x] Reject duplicate bonds `C12CCCCC12` — two bonds between same atom pair
- [x] Whitespace terminator — stop parsing at SPACE/TAB/LF/CR
- [x] Fix disconnected structures — `.` no longer creates a bond in the graph
- [x] Hückel's rule aromaticity validation (4N+2 π-electron check)

### SMILES Parser — Beyond OpenSMILES
- [x] Parallel batch parsing (optional `parallel` feature)
- [x] Element data (atomic number, standard mass, valence electrons for all 118 elements)
- [ ] `to_smiles()` — convert molecule back to SMILES string
- [ ] Canonical SMILES output
- [ ] SMILES normalization

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
