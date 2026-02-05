window.BENCHMARK_DATA = {
  "lastUpdate": 1770328192518,
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
          "id": "69c0dec34965959400c21a35222172648af52da6",
          "message": "feat: add benchmark comparison dashboard on GitHub Pages (#13)\n\n* feat: add benchmark comparison dashboard on GitHub Pages\n\nAdd an interactive comparison page (compare.html) that reads the same\ndata.js produced by benchmark-action and generates side-by-side charts:\n\n- Sequential vs Parallel: bar chart + speedup factor by batch size\n- Linear vs Branched: PEG vs Teflon parse time by chain length\n- Reference Molecules: horizontal bar chart sorted by complexity\n\nThe page is auto-deployed to gh-pages on each push to master via\na new workflow step. Dark mode support and responsive layout included.\n\nhttps://claude.ai/code/session_01WsPK2ZhbnX6pQ4Vb4KGRRm\n\n* feat: add shared tab navigation between benchmark pages\n\n- Create nav.js that auto-injects a sticky tab bar (Historical Trends /\n  Comparison) with active state detection and GitHub link\n- Update compare.html to load nav.js instead of hardcoded navigation\n- Update workflow to copy nav.js to gh-pages and inject it into\n  benchmark-action's index.html via sed\n\nThe nav bar works on both pages: natively on compare.html, and injected\ninto index.html during CI deployment.\n\nhttps://claude.ai/code/session_01WsPK2ZhbnX6pQ4Vb4KGRRm\n\n---------\n\nCo-authored-by: Claude <noreply@anthropic.com>",
          "timestamp": "2026-02-05T22:45:07+01:00",
          "tree_id": "24f9099b88ef4e90cdefba52519e153c9a1d3d16",
          "url": "https://github.com/Peariforme/bigsmiles-rs/commit/69c0dec34965959400c21a35222172648af52da6"
        },
        "date": 1770328192203,
        "tool": "cargo",
        "benches": [
          {
            "name": "reference/ethanol",
            "value": 253,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "reference/cyclohexane",
            "value": 568,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "reference/benzene",
            "value": 556,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "reference/ibuprofen",
            "value": 1598,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "reference/caffeine",
            "value": 1408,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10",
            "value": 6907,
            "range": "± 123",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10",
            "value": 16498,
            "range": "± 2728",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/100",
            "value": 73067,
            "range": "± 270",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/100",
            "value": 60562,
            "range": "± 1931",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/1000",
            "value": 759135,
            "range": "± 5214",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/1000",
            "value": 418170,
            "range": "± 15127",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/5000",
            "value": 3756991,
            "range": "± 12096",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/5000",
            "value": 1868789,
            "range": "± 64890",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10000",
            "value": 7561086,
            "range": "± 66582",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10000",
            "value": 3651958,
            "range": "± 68283",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/50000",
            "value": 37296676,
            "range": "± 375831",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/50000",
            "value": 18075291,
            "range": "± 460578",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/100",
            "value": 18591,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/1000",
            "value": 174780,
            "range": "± 707",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/5000",
            "value": 867627,
            "range": "± 4287",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/100",
            "value": 28356,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/500",
            "value": 136143,
            "range": "± 194",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/1000",
            "value": 269100,
            "range": "± 508",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}