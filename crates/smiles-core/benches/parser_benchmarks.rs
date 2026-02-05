//! Benchmarks for SMILES parsing performance.
//!
//! Run with: cargo bench -p smiles-core --features parallel
//!
//! This compares sequential vs parallel parsing and measures
//! performance across different molecule sizes and complexities.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use smiles_core::{parse, Molecule};

#[cfg(feature = "parallel")]
use smiles_core::parser_parallel::{parse_batch, parse_batch_ok};

// Sample SMILES strings for benchmarking
const SIMPLE_MOLECULES: &[&str] = &[
    "C",        // methane
    "CC",       // ethane
    "CCC",      // propane
    "CCCC",     // butane
    "CCCCC",    // pentane
    "CCCCCC",   // hexane
    "CCCCCCC",  // heptane
    "CCCCCCCC", // octane
];

const BRANCHED_MOLECULES: &[&str] = &[
    "CC(C)C",                  // isobutane
    "CC(C)(C)C",               // neopentane
    "CC(C)CC(C)C",             // 2,4-dimethylpentane
    "CC(C)(C)CC(C)(C)C",       // 2,2,4,4-tetramethylpentane
    "CC(C)C(C)C(C)C",          // multiple branches
    "C(C(C(C(C)C)C)C)C",       // nested branches
    "CC(C)(C)C(C)(C)C(C)(C)C", // heavy branching
];

const CYCLIC_MOLECULES: &[&str] = &[
    "C1CC1",      // cyclopropane
    "C1CCC1",     // cyclobutane
    "C1CCCC1",    // cyclopentane
    "C1CCCCC1",   // cyclohexane
    "C1CCCCCC1",  // cycloheptane
    "C1CCCCCCC1", // cyclooctane
];

const AROMATIC_MOLECULES: &[&str] = &[
    "c1ccccc1",                  // benzene
    "c1ccc2ccccc2c1",            // naphthalene
    "c1cc2cccc3cccc4cccc1c4c32", // pyrene
    "c1ccccc1c2ccccc2",          // biphenyl
    "c1ccc(c2ccccc2)cc1",        // another biphenyl notation
];

const COMPLEX_MOLECULES: &[&str] = &[
    "CC(=O)O",                      // acetic acid
    "CCO",                          // ethanol
    "CC(=O)OC",                     // methyl acetate
    "c1ccc(O)cc1",                  // phenol
    "CC(C)Cc1ccc(C(C)C(=O)O)cc1",   // ibuprofen-like
    "CN1C=NC2=C1C(=O)N(C(=O)N2C)C", // caffeine-like
];

/// Generate a linear alkane chain of n carbons: C, CC, CCC, ...
fn generate_linear_alkane(n: usize) -> String {
    "C".repeat(n)
}

/// Generate a branched molecule with multiple methyl branches
/// Pattern: C(C)(C)C(C)(C)C(C)(C)...
fn generate_branched_chain(branch_count: usize) -> String {
    if branch_count == 0 {
        return "C".to_string();
    }
    let mut s = String::with_capacity(branch_count * 8);
    for i in 0..branch_count {
        if i > 0 {
            s.push_str("C(C)(C)");
        } else {
            s.push_str("CC(C)(C)");
        }
    }
    s.push('C');
    s
}

/// Generate a star-shaped molecule with n branches from a central carbon
/// Pattern: C(C)(C)(C)... - simpler than dendrimers, works with current parser
fn generate_star_molecule(branches: usize) -> String {
    if branches == 0 {
        return "C".to_string();
    }
    let mut s = String::from("C");
    for _ in 0..branches {
        s.push_str("(C)");
    }
    s
}

/// Generate a comb polymer structure
/// Pattern: C(C)C(C)C(C)C... - main chain with pendant groups
fn generate_comb_polymer(length: usize) -> String {
    if length == 0 {
        return "C".to_string();
    }
    let mut s = String::new();
    for i in 0..length {
        if i > 0 {
            s.push_str("C(C)");
        } else {
            s.push_str("CC(C)");
        }
    }
    s
}

/// Calculate approximate memory size of a molecule
fn molecule_memory_size(mol: &Molecule) -> usize {
    use std::mem::{size_of, size_of_val};

    // Base struct size
    let base_size = size_of::<Molecule>();

    // Nodes and bonds slices
    let nodes_size = size_of_val(mol.nodes());
    let bonds_size = size_of_val(mol.bonds());

    base_size + nodes_size + bonds_size
}

// Generate a large dataset for batch testing
fn generate_batch_dataset(size: usize) -> Vec<&'static str> {
    let all_molecules: Vec<&str> = SIMPLE_MOLECULES
        .iter()
        .chain(BRANCHED_MOLECULES.iter())
        .chain(CYCLIC_MOLECULES.iter())
        .chain(AROMATIC_MOLECULES.iter())
        .chain(COMPLEX_MOLECULES.iter())
        .copied()
        .collect();

    (0..size)
        .map(|i| all_molecules[i % all_molecules.len()])
        .collect()
}

fn bench_single_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_parse");

    // Benchmark simple molecules
    for smiles in SIMPLE_MOLECULES.iter().take(4) {
        group.bench_with_input(BenchmarkId::new("simple", smiles), smiles, |b, s| {
            b.iter(|| parse(black_box(s)))
        });
    }

    // Benchmark branched molecules
    for smiles in BRANCHED_MOLECULES.iter().take(4) {
        group.bench_with_input(BenchmarkId::new("branched", smiles), smiles, |b, s| {
            b.iter(|| parse(black_box(s)))
        });
    }

    // Benchmark cyclic molecules
    for smiles in CYCLIC_MOLECULES.iter().take(4) {
        group.bench_with_input(BenchmarkId::new("cyclic", smiles), smiles, |b, s| {
            b.iter(|| parse(black_box(s)))
        });
    }

    // Benchmark aromatic molecules
    for smiles in AROMATIC_MOLECULES.iter().take(4) {
        group.bench_with_input(BenchmarkId::new("aromatic", smiles), smiles, |b, s| {
            b.iter(|| parse(black_box(s)))
        });
    }

    group.finish();
}

fn bench_sequential_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_batch");

    for size in [10, 100, 1000, 10000].iter() {
        let dataset = generate_batch_dataset(*size);
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::new("sequential", size), &dataset, |b, data| {
            b.iter(|| {
                let results: Vec<_> = data.iter().map(|s| parse(black_box(s))).collect();
                black_box(results)
            })
        });
    }

    group.finish();
}

#[cfg(feature = "parallel")]
fn bench_parallel_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_batch");

    for size in [10, 100, 1000, 10000].iter() {
        let dataset = generate_batch_dataset(*size);
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::new("parallel", size), &dataset, |b, data| {
            b.iter(|| black_box(parse_batch(black_box(data))))
        });
    }

    group.finish();
}

#[cfg(feature = "parallel")]
fn bench_sequential_vs_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("seq_vs_parallel");

    for size in [100, 1000, 10000].iter() {
        let dataset = generate_batch_dataset(*size);
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("sequential", size), &dataset, |b, data| {
            b.iter(|| {
                let results: Vec<_> = data.iter().map(|s| parse(black_box(s))).collect();
                black_box(results)
            })
        });

        group.bench_with_input(BenchmarkId::new("parallel", size), &dataset, |b, data| {
            b.iter(|| black_box(parse_batch(black_box(data))))
        });
    }

    group.finish();
}

fn bench_molecule_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("complexity");

    // Linear alkanes of increasing length
    let linear_alkanes = [
        "C",
        "CCCCCCCCCC",                     // 10 carbons
        "CCCCCCCCCCCCCCCCCCCC",           // 20 carbons
        "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCC", // 30 carbons
    ];

    for smiles in linear_alkanes.iter() {
        group.bench_with_input(BenchmarkId::new("linear", smiles.len()), smiles, |b, s| {
            b.iter(|| parse(black_box(s)))
        });
    }

    // Branching complexity
    let branching = [
        "CC",
        "CC(C)C",
        "CC(C)(C)C",
        "CC(C)(C)C(C)(C)C",
        "CC(C)(C)C(C)(C)C(C)(C)C",
    ];

    for smiles in branching.iter() {
        let branch_count = smiles.matches('(').count();
        group.bench_with_input(
            BenchmarkId::new("branches", branch_count),
            smiles,
            |b, s| b.iter(|| parse(black_box(s))),
        );
    }

    group.finish();
}

#[cfg(feature = "parallel")]
fn bench_parallel_scaling(c: &mut Criterion) {
    use std::time::Duration;

    let mut group = c.benchmark_group("parallel_scaling");
    group.measurement_time(Duration::from_secs(3));

    // Test how parallelism scales with batch size (reduced set for CI speed)
    for size in [100, 1000, 10000].iter() {
        let dataset = generate_batch_dataset(*size);

        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("parallel", size), &dataset, |b, data| {
            b.iter(|| black_box(parse_batch_ok(black_box(data))))
        });
    }

    group.finish();
}

/// Benchmark very large molecules
fn bench_large_molecules(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_molecules");

    // Linear chains (reduced set for CI speed)
    for size in [100, 1000, 10000].iter() {
        let smiles = generate_linear_alkane(*size);
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::new("linear_chain", size), &smiles, |b, s| {
            b.iter(|| parse(black_box(s)))
        });
    }

    // Heavily branched molecules
    for branches in [10, 25].iter() {
        let smiles = generate_branched_chain(*branches);
        group.bench_with_input(BenchmarkId::new("branched", branches), &smiles, |b, s| {
            b.iter(|| parse(black_box(s)))
        });
    }

    // Star molecules (many branches from central atom)
    for branches in [10, 25].iter() {
        let smiles = generate_star_molecule(*branches);
        let atom_count = branches + 1; // central + branches
        group.throughput(Throughput::Elements(atom_count as u64));
        group.bench_with_input(
            BenchmarkId::new("star_branches", branches),
            &smiles,
            |b, s| b.iter(|| parse(black_box(s))),
        );
    }

    // Comb polymers (main chain with pendant groups)
    for length in [25, 50].iter() {
        let smiles = generate_comb_polymer(*length);
        let atom_count = length * 2 + 1; // main chain + pendants
        group.throughput(Throughput::Elements(atom_count as u64));
        group.bench_with_input(BenchmarkId::new("comb_polymer", length), &smiles, |b, s| {
            b.iter(|| parse(black_box(s)))
        });
    }

    group.finish();
}

/// Benchmark memory usage of parsed molecules
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");

    // Test memory for different molecule sizes
    let test_cases = [
        ("methane", "C"),
        ("decane", "CCCCCCCCCC"),
        ("benzene", "c1ccccc1"),
        ("caffeine", "CN1C=NC2=C1C(=O)N(C(=O)N2C)C"),
    ];

    for (name, smiles) in test_cases.iter() {
        let mol = parse(smiles).expect("Should parse");
        let mem_size = molecule_memory_size(&mol);
        println!(
            "{}: {} nodes, {} bonds, ~{} bytes",
            name,
            mol.nodes().len(),
            mol.bonds().len(),
            mem_size
        );

        group.throughput(Throughput::Bytes(mem_size as u64));
        group.bench_with_input(BenchmarkId::new("parse", name), smiles, |b, s| {
            b.iter(|| parse(black_box(s)))
        });
    }

    // Large molecule memory test (reduced set for CI speed)
    for size in [100, 1000, 10000].iter() {
        let smiles = generate_linear_alkane(*size);
        let mol = parse(&smiles).expect("Should parse");
        let mem_size = molecule_memory_size(&mol);

        println!(
            "linear_chain_{}: {} atoms, {} bonds, ~{} bytes ({} bytes/atom, {:.2} MB total)",
            size,
            mol.nodes().len(),
            mol.bonds().len(),
            mem_size,
            mem_size / mol.nodes().len().max(1),
            mem_size as f64 / (1024.0 * 1024.0)
        );

        group.throughput(Throughput::Bytes(mem_size as u64));
        group.bench_with_input(BenchmarkId::new("linear_chain", size), &smiles, |b, s| {
            b.iter(|| parse(black_box(s)))
        });
    }

    // Star molecule memory test
    for branches in [10, 50].iter() {
        let smiles = generate_star_molecule(*branches);
        if let Ok(mol) = parse(&smiles) {
            let mem_size = molecule_memory_size(&mol);
            let atom_count = mol.nodes().len();

            println!(
                "star_{}_branches: {} atoms, {} bonds, ~{} bytes ({} bytes/atom)",
                branches,
                atom_count,
                mol.bonds().len(),
                mem_size,
                mem_size / atom_count.max(1)
            );

            group.throughput(Throughput::Bytes(mem_size as u64));
            group.bench_with_input(
                BenchmarkId::new("star_branches", branches),
                &smiles,
                |b, s| b.iter(|| parse(black_box(s))),
            );
        }
    }

    // Comb polymer memory test
    for length in [50].iter() {
        let smiles = generate_comb_polymer(*length);
        if let Ok(mol) = parse(&smiles) {
            let mem_size = molecule_memory_size(&mol);
            let atom_count = mol.nodes().len();

            println!(
                "comb_polymer_{}: {} atoms, {} bonds, ~{} bytes ({} bytes/atom)",
                length,
                atom_count,
                mol.bonds().len(),
                mem_size,
                mem_size / atom_count.max(1)
            );

            group.throughput(Throughput::Bytes(mem_size as u64));
            group.bench_with_input(BenchmarkId::new("comb_polymer", length), &smiles, |b, s| {
                b.iter(|| parse(black_box(s)))
            });
        }
    }

    group.finish();
}

// Conditional criterion groups based on features
#[cfg(feature = "parallel")]
criterion_group!(
    benches,
    bench_single_parse,
    bench_sequential_batch,
    bench_parallel_batch,
    bench_sequential_vs_parallel,
    bench_molecule_complexity,
    bench_parallel_scaling,
    bench_large_molecules,
    bench_memory_usage,
);

#[cfg(not(feature = "parallel"))]
criterion_group!(
    benches,
    bench_single_parse,
    bench_sequential_batch,
    bench_molecule_complexity,
    bench_large_molecules,
    bench_memory_usage,
);

criterion_main!(benches);
