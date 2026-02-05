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