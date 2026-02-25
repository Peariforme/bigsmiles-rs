//! Parser benchmarks.
//!
//! Run with:  `cargo bench -p bigsmiles`
//! HTML report: `target/criterion/report/index.html`
//!
//! The `scaling` group measures throughput (MB/s) across inputs of different
//! lengths. A roughly constant throughput confirms O(n) parsing complexity.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use bigsmiles::parse;

// ── Individual molecule benchmarks ───────────────────────────────────────────

fn bench_simple(c: &mut Criterion) {
    c.bench_function("polyethylene_no_bond", |b| {
        b.iter(|| parse(black_box("{[]CC[]}")))
    });

    c.bench_function("polyethylene_non_directional", |b| {
        b.iter(|| parse(black_box("{[$]CC[$]}")))
    });

    c.bench_function("polystyrene", |b| {
        b.iter(|| parse(black_box("{[]CC(c1ccccc1)[]}")))
    });

    c.bench_function("isotactic_polypropylene_with_terminals", |b| {
        b.iter(|| parse(black_box("{[>][<]CC(C)[>][<]}")))
    });

    c.bench_function("alpha_omega_dimethyl_polyethylene", |b| {
        b.iter(|| parse(black_box("CC{[$]CC[$]}CC")))
    });
}

fn bench_copolymer(c: &mut Criterion) {
    c.bench_function("copolymer_2_units", |b| {
        b.iter(|| parse(black_box("{[$]CC[$],[$]CC(C)[$]}")))
    });

    c.bench_function("copolymer_with_end_groups", |b| {
        b.iter(|| parse(black_box("{[$]CC[$],[$]CC(C)[$];[$]CCO[$]}")))
    });
}

// ── O(n) scaling benchmark ────────────────────────────────────────────────────
//
// We generate a copolymer with `n` repeat units of the form `[$]CC[$]` and
// measure parsing throughput in bytes per second.
//
// If the parser is O(n) in input length, the throughput (MB/s) should be
// approximately constant regardless of `n`.

fn bench_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_scaling");

    for &n in &[1usize, 10, 50, 100, 500, 1000] {
        // Build: {[$]CC[$],[$]CC[$],...}  (n units)
        let unit = "[$]CC[$]";
        let inner: String = std::iter::repeat(unit)
            .take(n)
            .collect::<Vec<_>>()
            .join(",");
        let input = format!("{{{inner}}}");

        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(BenchmarkId::new("copolymer_units", n), &input, |b, s| {
            b.iter(|| parse(black_box(s)))
        });
    }

    group.finish();
}

// ── Benchmark with complex SMILES per unit ────────────────────────────────────

fn bench_complex_smiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_smiles_scaling");

    // Polystyrene unit: CC(c1ccccc1) — more atoms per repeat unit
    for &n in &[1usize, 10, 50, 100] {
        let unit = "[$]CC(c1ccccc1)[$]";
        let inner: String = std::iter::repeat(unit)
            .take(n)
            .collect::<Vec<_>>()
            .join(",");
        let input = format!("{{{inner}}}");

        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(BenchmarkId::new("polystyrene_units", n), &input, |b, s| {
            b.iter(|| parse(black_box(s)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_simple,
    bench_copolymer,
    bench_scaling,
    bench_complex_smiles,
);
criterion_main!(benches);
