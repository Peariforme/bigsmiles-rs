//! Benchmarks for SMILES parsing performance.
//!
//! Run with: cargo bench -p smiles-core --features parallel
//!
//! This compares sequential vs parallel parsing and measures
//! performance across different molecule sizes and complexities.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use smiles_core::parse;

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
    "CC(C)C",              // isobutane
    "CC(C)(C)C",           // neopentane
    "CC(C)CC(C)C",         // 2,4-dimethylpentane
    "CC(C)(C)CC(C)(C)C",   // 2,2,4,4-tetramethylpentane
    "CC(C)C(C)C(C)C",      // multiple branches
    "C(C(C(C(C)C)C)C)C",   // nested branches
    "CC(C)(C)C(C)(C)C(C)(C)C", // heavy branching
];

const CYCLIC_MOLECULES: &[&str] = &[
    "C1CC1",              // cyclopropane
    "C1CCC1",             // cyclobutane
    "C1CCCC1",            // cyclopentane
    "C1CCCCC1",           // cyclohexane
    "C1CCCCCC1",          // cycloheptane
    "C1CCCCCCC1",         // cyclooctane
];

const AROMATIC_MOLECULES: &[&str] = &[
    "c1ccccc1",           // benzene
    "c1ccc2ccccc2c1",     // naphthalene
    "c1cc2cccc3cccc4cccc1c4c32", // pyrene
    "c1ccccc1c2ccccc2",   // biphenyl
    "c1ccc(c2ccccc2)cc1", // another biphenyl notation
];

const COMPLEX_MOLECULES: &[&str] = &[
    "CC(=O)O",                    // acetic acid
    "CCO",                        // ethanol
    "CC(=O)OC",                   // methyl acetate
    "c1ccc(O)cc1",                // phenol
    "CC(C)Cc1ccc(C(C)C(=O)O)cc1", // ibuprofen-like
    "CN1C=NC2=C1C(=O)N(C(=O)N2C)C", // caffeine-like
];

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
        group.bench_with_input(
            BenchmarkId::new("simple", smiles),
            smiles,
            |b, s| b.iter(|| parse(black_box(s))),
        );
    }

    // Benchmark branched molecules
    for smiles in BRANCHED_MOLECULES.iter().take(4) {
        group.bench_with_input(
            BenchmarkId::new("branched", smiles),
            smiles,
            |b, s| b.iter(|| parse(black_box(s))),
        );
    }

    // Benchmark cyclic molecules
    for smiles in CYCLIC_MOLECULES.iter().take(4) {
        group.bench_with_input(
            BenchmarkId::new("cyclic", smiles),
            smiles,
            |b, s| b.iter(|| parse(black_box(s))),
        );
    }

    // Benchmark aromatic molecules
    for smiles in AROMATIC_MOLECULES.iter().take(4) {
        group.bench_with_input(
            BenchmarkId::new("aromatic", smiles),
            smiles,
            |b, s| b.iter(|| parse(black_box(s))),
        );
    }

    group.finish();
}

fn bench_sequential_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_batch");

    for size in [10, 100, 1000, 10000].iter() {
        let dataset = generate_batch_dataset(*size);
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("sequential", size),
            &dataset,
            |b, data| {
                b.iter(|| {
                    let results: Vec<_> = data.iter().map(|s| parse(black_box(s))).collect();
                    black_box(results)
                })
            },
        );
    }

    group.finish();
}

#[cfg(feature = "parallel")]
fn bench_parallel_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_batch");

    for size in [10, 100, 1000, 10000].iter() {
        let dataset = generate_batch_dataset(*size);
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("parallel", size),
            &dataset,
            |b, data| b.iter(|| black_box(parse_batch(black_box(data)))),
        );
    }

    group.finish();
}

#[cfg(feature = "parallel")]
fn bench_sequential_vs_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("seq_vs_parallel");

    for size in [100, 1000, 10000].iter() {
        let dataset = generate_batch_dataset(*size);
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(
            BenchmarkId::new("sequential", size),
            &dataset,
            |b, data| {
                b.iter(|| {
                    let results: Vec<_> = data.iter().map(|s| parse(black_box(s))).collect();
                    black_box(results)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("parallel", size),
            &dataset,
            |b, data| b.iter(|| black_box(parse_batch(black_box(data)))),
        );
    }

    group.finish();
}

fn bench_molecule_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("complexity");

    // Linear alkanes of increasing length
    let linear_alkanes = [
        "C",
        "CCCCCCCCCC",                    // 10 carbons
        "CCCCCCCCCCCCCCCCCCCC",          // 20 carbons
        "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCC", // 30 carbons
    ];

    for smiles in linear_alkanes.iter() {
        group.bench_with_input(
            BenchmarkId::new("linear", smiles.len()),
            smiles,
            |b, s| b.iter(|| parse(black_box(s))),
        );
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
    group.measurement_time(Duration::from_secs(10));

    // Test how parallelism scales with batch size
    for size in [10, 50, 100, 500, 1000, 5000, 10000].iter() {
        let dataset = generate_batch_dataset(*size);

        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(
            BenchmarkId::new("parallel", size),
            &dataset,
            |b, data| b.iter(|| black_box(parse_batch_ok(black_box(data)))),
        );
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
);

#[cfg(not(feature = "parallel"))]
criterion_group!(
    benches,
    bench_single_parse,
    bench_sequential_batch,
    bench_molecule_complexity,
);

criterion_main!(benches);
