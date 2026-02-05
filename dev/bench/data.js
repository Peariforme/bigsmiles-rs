window.BENCHMARK_DATA = {
  "lastUpdate": 1770318239271,
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
      }
    ]
  }
}