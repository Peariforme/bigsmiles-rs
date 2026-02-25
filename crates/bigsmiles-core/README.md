# bigsmiles

A BigSMILES parser for polymer and macromolecule notation in Rust.

[![Crates.io](https://img.shields.io/crates/v/bigsmiles.svg)](https://crates.io/crates/bigsmiles)
[![docs.rs](https://docs.rs/bigsmiles/badge.svg)](https://docs.rs/bigsmiles)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/Peariforme/bigsmiles-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Peariforme/bigsmiles-rs/actions/workflows/ci.yml)

## What is BigSMILES?

[BigSMILES](https://pubs.acs.org/doi/10.1021/acscentsci.9b00476) is a line notation for polymers and macromolecules.
It extends SMILES with *stochastic objects* `{...}` that describe repeat units and end groups.

```
{[$]CC[$]}                    → polyethylene
{[$]CC[$],[$]CC(C)[$]}        → ethylene-propylene copolymer
CC{[$]CC[$]}CC                → α,ω-dimethyl polyethylene
{[>][<]CC(C)[>][<]}           → isotactic polypropylene
{[$]CC[$];[$]CCO[$]}          → polyethylene with hydroxyl end group
```

## Installation

```toml
[dependencies]
bigsmiles = "0.1"
```

## Usage

### Basic parsing

```rust
use bigsmiles::parse;

let pe = parse("{[$]CC[$]}").unwrap();          // polyethylene
let ps = parse("{[]CC(c1ccccc1)[]}").unwrap();  // polystyrene
let copo = parse("{[$]CC[$],[$]CC(C)[$]}").unwrap(); // copolymer

// Display produces the canonical BigSMILES string
println!("{}", pe);   // {[$]CC[$]}
```

### Inspecting the AST

```rust
use bigsmiles::{parse, BigSmilesSegment};

let result = parse("CC{[$]CC[$]}CC").unwrap();

for seg in &result.segments {
    match seg {
        BigSmilesSegment::Smiles(mol) => {
            println!("SMILES fragment: {} atoms", mol.nodes().len());
        }
        BigSmilesSegment::Stochastic(obj) => {
            println!("Stochastic object: {} repeat unit(s)", obj.repeat_units.len());
            for ru in &obj.repeat_units {
                println!("  repeat unit: {}", ru.smiles_raw);
                println!("  left BD: {:?}, right BD: {:?}", ru.left, ru.right);
                println!("  left atom index: {}, right atom index: {}", ru.left_atom, ru.right_atom);
            }
        }
    }
}
```

### Bond descriptors

BigSMILES uses bond descriptors to define how repeat units connect:

| Descriptor | Meaning |
|------------|---------|
| `[]`       | No-bond terminal (open end group) |
| `[$]`      | Non-directional (connects to any `[$]`) |
| `[<]`      | Head (connects to `[>]`) |
| `[>]`      | Tail (connects to `[<]`) |
| `[$1]`     | Indexed non-directional (connects to same index) |
| `[<2]`     | Indexed head |
| `[>2]`     | Indexed tail |

### Connection atoms

Each stochastic fragment records which atom index bonds to each descriptor:

- `left_atom` — always `0` (the first written atom)
- `right_atom` — the last atom on the main chain (depth 0), not counting branch atoms

For `CC(C)` (polypropylene): `left_atom = 0` (C0), `right_atom = 1` (C1, backbone).
The methyl branch C2 is *not* the connection atom.

### Error handling

```rust
use bigsmiles::{parse, ParseError};

match parse("{[$]CC[$]") {
    Ok(mol)  => println!("ok: {}", mol),
    Err(ParseError::UnclosedStochasticObject) => eprintln!("missing closing }"),
    Err(e)   => eprintln!("parse error: {}", e),
}
```

## Supported BigSMILES features

| Feature | Status |
|---------|--------|
| Stochastic objects `{...}` | ✅ |
| Non-directional descriptors `[$]` | ✅ |
| Directional descriptors `[<]` `[>]` | ✅ |
| No-bond descriptors `[]` | ✅ |
| Indexed descriptors `[$1]`, `[<2]`, `[>2]` | ✅ |
| Multiple repeat units (copolymers) `,` | ✅ |
| End groups `;` | ✅ |
| Outer terminals `{[>]...[<]}` | ✅ |
| Surrounding SMILES `CC{...}CC` | ✅ |
| Full OpenSMILES inside stochastic objects | ✅ |
| Connection atom tracking (`left_atom`, `right_atom`) | ✅ |
| Faithful round-trip display (topology-preserving) | ✅ |

## Relationship to `opensmiles`

`bigsmiles` depends on [`opensmiles`](https://crates.io/crates/opensmiles) (also part of this workspace)
for parsing the SMILES fragments inside stochastic objects. The `opensmiles` crate is re-exported
as `bigsmiles::opensmiles` for convenience.

## References

- Lin, T.-S. et al. "BigSMILES: A Structurally-Based Line Notation for Describing Macromolecules."
  *ACS Central Science* **2019**, 5, 1523–1531. <https://pubs.acs.org/doi/10.1021/acscentsci.9b00476>
- [BigSMILES Documentation](https://olsenlabmit.github.io/BigSMILES/docs/line_notation.html)
- [OpenSMILES Specification](http://opensmiles.org/opensmiles.html)

## License

MIT — see [LICENSE](https://github.com/Peariforme/bigsmiles-rs/blob/master/LICENSE).
