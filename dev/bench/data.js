window.BENCHMARK_DATA = {
  "lastUpdate": 1770322843173,
  "repoUrl": "https://github.com/Peariforme/bigsmiles-rs",
  "entries": {
    "SMILES Parser Benchmarks": [
      {
        "commit": {
          "author": {
            "email": "47952322+Peariforme@users.noreply.github.com",
            "name": "Peariforme",
            "username": "Peariforme"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a2e25df5ea0402bc11e1f6433efbee5861a6f868",
          "message": "perf: optimize benchmark workflow and add GitHub Pages integration (#11)\n\n* perf: optimize benchmark workflow and add GitHub Pages integration\n\n- Use github-action-benchmark for historical tracking on GitHub Pages\n- Remove duplicate benchmark run on merge (was running twice)\n- On PR: compare with baseline and comment results\n- On merge to main: save results as new baseline (single run)\n- Reduce measurement_time from 10s to 3s in parallel_scaling\n- Reduce max molecule sizes from 50000 to 10000 atoms\n- Reduce benchmark variants for faster CI execution\n- Add benchmark badge linking to GitHub Pages\n\nhttps://claude.ai/code/session_01RWxReT7bN6ApQatMvBTo4H\n\n* perf: keep 50000 nodes benchmark to track performance degradation\n\nhttps://claude.ai/code/session_01RWxReT7bN6ApQatMvBTo4H\n\n* fix: resolve clippy warning and benchmark output format\n\n- Fix single_element_loop warning by adding second size to comb polymer test\n- Use customSmallerIsBetter format with Python script to extract Criterion JSON\n- Criterion 0.5 doesn't support --output-format bencher\n\nhttps://claude.ai/code/session_01RWxReT7bN6ApQatMvBTo4H\n\n* fix: use cargo-criterion for JSON output and realistic polymer sizes\n\n- Replace Python script with cargo-criterion (Rust tooling)\n- Use criterionrs format compatible with github-action-benchmark\n- Update benchmark sizes to realistic polymer chain lengths:\n  - Linear chains: 100, 1000, 2000, 50000 monomers\n  - Comb polymers: 100, 500, 1000 monomers\n  - Star/branched: 10, 50 branches\n\nhttps://claude.ai/code/session_01RWxReT7bN6ApQatMvBTo4H\n\n* fix: use customSmallerIsBetter format with jq transformation\n\ncriterionrs is not a valid tool option for github-action-benchmark.\nUse jq to transform cargo-criterion JSON output to customSmallerIsBetter format.\n\nhttps://claude.ai/code/session_01RWxReT7bN6ApQatMvBTo4H\n\n---------\n\nCo-authored-by: Claude <noreply@anthropic.com>",
          "timestamp": "2026-02-05T19:21:41+01:00",
          "tree_id": "a616f858701f39964f5e0089f7fd6f30768444d2",
          "url": "https://github.com/Peariforme/bigsmiles-rs/commit/a2e25df5ea0402bc11e1f6433efbee5861a6f868"
        },
        "date": 1770318238917,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "single_parse/simple/C",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/simple/CC",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/simple/CCC",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/simple/CCCC",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/branched/CC(C)C",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/branched/CC(C)(C)C",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/branched/CC(C)CC(C)C",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/branched/CC(C)(C)CC(C)(C)C",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/cyclic/C1CC1",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/cyclic/C1CCC1",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/cyclic/C1CCCC1",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/cyclic/C1CCCCC1",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/aromatic/c1ccccc1",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/aromatic/c1ccc2ccccc2c1",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/aromatic/c1cc2cccc3cccc4cccc1c4c32",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "single_parse/aromatic/c1ccccc1c2ccccc2",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "sequential_batch/sequential/10",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "sequential_batch/sequential/100",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "sequential_batch/sequential/1000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "sequential_batch/sequential/10000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "parallel_batch/parallel/10",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "parallel_batch/parallel/100",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "parallel_batch/parallel/1000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "parallel_batch/parallel/10000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "seq_vs_parallel/sequential/100",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "seq_vs_parallel/parallel/100",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "seq_vs_parallel/sequential/1000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "seq_vs_parallel/parallel/1000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "seq_vs_parallel/sequential/10000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "seq_vs_parallel/parallel/10000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "complexity/linear/1",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "complexity/linear/10",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "complexity/linear/20",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "complexity/linear/30",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "complexity/branches/0",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "complexity/branches/1",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "complexity/branches/2",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "complexity/branches/4",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "complexity/branches/6",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "parallel_scaling/parallel/100",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "parallel_scaling/parallel/1000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "parallel_scaling/parallel/10000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "large_molecules/linear_chain/100",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "large_molecules/linear_chain/1000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "large_molecules/linear_chain/2000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "large_molecules/linear_chain/50000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "large_molecules/branched/10",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "large_molecules/branched/50",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "large_molecules/star_branches/10",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "large_molecules/star_branches/50",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "large_molecules/comb_polymer/100",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "large_molecules/comb_polymer/500",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "large_molecules/comb_polymer/1000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "memory_usage/parse/methane",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "memory_usage/parse/decane",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "memory_usage/parse/benzene",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "memory_usage/parse/caffeine",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "memory_usage/linear_chain/100",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "memory_usage/linear_chain/1000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "memory_usage/linear_chain/2000",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "memory_usage/star_branches/10",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "memory_usage/star_branches/50",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "memory_usage/comb_polymer/100",
            "value": 0,
            "unit": "ns"
          },
          {
            "name": "memory_usage/comb_polymer/500",
            "value": 0,
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "47952322+Peariforme@users.noreply.github.com",
            "name": "Peariforme",
            "username": "Peariforme"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c19a26874f0d4ebdc7a7722169a0f16c5182d4dc",
          "message": "refine benchmarks (#12)\n\n* refine benchmarks\n\n* update benchmark feature scope",
          "timestamp": "2026-02-05T21:15:48+01:00",
          "tree_id": "2402c62e8b4a0b0a0f156bc27e4948d74a6cd5d9",
          "url": "https://github.com/Peariforme/bigsmiles-rs/commit/c19a26874f0d4ebdc7a7722169a0f16c5182d4dc"
        },
        "date": 1770322842590,
        "tool": "cargo",
        "benches": [
          {
            "name": "reference/ethanol",
            "value": 251,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "reference/cyclohexane",
            "value": 551,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "reference/benzene",
            "value": 560,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "reference/ibuprofen",
            "value": 1574,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "reference/caffeine",
            "value": 1415,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10",
            "value": 6877,
            "range": "± 277",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10",
            "value": 16070,
            "range": "± 4471",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/100",
            "value": 73977,
            "range": "± 297",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/100",
            "value": 57709,
            "range": "± 1343",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/1000",
            "value": 762110,
            "range": "± 2970",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/1000",
            "value": 412381,
            "range": "± 16724",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/5000",
            "value": 3783252,
            "range": "± 128466",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/5000",
            "value": 1890290,
            "range": "± 90177",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10000",
            "value": 7507084,
            "range": "± 24339",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10000",
            "value": 3664838,
            "range": "± 55688",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/50000",
            "value": 37093844,
            "range": "± 467334",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/50000",
            "value": 17709415,
            "range": "± 264646",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/100",
            "value": 18572,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/1000",
            "value": 173493,
            "range": "± 556",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/5000",
            "value": 894250,
            "range": "± 4865",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/100",
            "value": 28898,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/500",
            "value": 142747,
            "range": "± 389",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/1000",
            "value": 274133,
            "range": "± 1554",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}