window.BENCHMARK_DATA = {
  "lastUpdate": 1772016582159,
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
          "id": "9286e5a6a81f7e38610123a05af42b552b4760c6",
          "message": "fix: fix comparison charts x-axis labels, align benchmark sizes, remove empty graph (#14)\n\n- Fix Chart.js bar charts showing tick indices (0,1,2…) instead of\n  actual batch size / repeat unit labels on the x-axis. The root cause\n  was setting `callback: undefined` on category axis ticks, which\n  overrides Chart.js's default label renderer. Conditionally set the\n  callback property only when a real function is needed.\n- Align PEG and Teflon benchmark repeat unit sizes to [100, 500, 1000,\n  5000] so the scaling comparison is meaningful (previously PEG used\n  [100, 1000, 5000] and Teflon used [100, 500, 1000]).\n- Remove the empty \"Single Molecule Parse Time\" horizontal bar chart\n  from the Reference Molecules section (table is kept).\n\nhttps://claude.ai/code/session_01LEufEiHBZPJmyiA7d5o66x\n\nCo-authored-by: Claude <noreply@anthropic.com>",
          "timestamp": "2026-02-05T23:21:04+01:00",
          "tree_id": "b33133f86da9678695750ae6896d71210aef52a3",
          "url": "https://github.com/Peariforme/bigsmiles-rs/commit/9286e5a6a81f7e38610123a05af42b552b4760c6"
        },
        "date": 1770330368232,
        "tool": "cargo",
        "benches": [
          {
            "name": "reference/ethanol",
            "value": 222,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "reference/cyclohexane",
            "value": 579,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "reference/benzene",
            "value": 580,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "reference/ibuprofen",
            "value": 1620,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "reference/caffeine",
            "value": 1450,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10",
            "value": 7047,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10",
            "value": 15300,
            "range": "± 295",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/100",
            "value": 71982,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/100",
            "value": 74902,
            "range": "± 5662",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/1000",
            "value": 746961,
            "range": "± 3184",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/1000",
            "value": 487611,
            "range": "± 8693",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/5000",
            "value": 3669509,
            "range": "± 8599",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/5000",
            "value": 2228214,
            "range": "± 31035",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10000",
            "value": 7402637,
            "range": "± 16401",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10000",
            "value": 4425517,
            "range": "± 41003",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/50000",
            "value": 37422848,
            "range": "± 544024",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/50000",
            "value": 22052376,
            "range": "± 290663",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/100",
            "value": 16763,
            "range": "± 118",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/500",
            "value": 77638,
            "range": "± 521",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/1000",
            "value": 154553,
            "range": "± 2271",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/5000",
            "value": 760327,
            "range": "± 5959",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/100",
            "value": 25698,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/500",
            "value": 120695,
            "range": "± 1022",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/1000",
            "value": 236170,
            "range": "± 560",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/5000",
            "value": 1171755,
            "range": "± 25986",
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
          "id": "26059f6791e359a0b4cd832ee5bff9f38e8c08ae",
          "message": "Fix nav.js duplicate injection in benchmark workflow (#15)\n\nAdd grep guard to check if nav.js is already present in index.html\nbefore injecting the script tag, preventing duplication on repeated runs.\n\nhttps://claude.ai/code/session_01Y4XK6sxqwtP5JXeScH9Bi5\n\nCo-authored-by: Claude <noreply@anthropic.com>",
          "timestamp": "2026-02-05T23:43:38+01:00",
          "tree_id": "ed3c32daeec818d01395b3536e94ef61fbeb58ab",
          "url": "https://github.com/Peariforme/bigsmiles-rs/commit/26059f6791e359a0b4cd832ee5bff9f38e8c08ae"
        },
        "date": 1770331738233,
        "tool": "cargo",
        "benches": [
          {
            "name": "reference/ethanol",
            "value": 249,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "reference/cyclohexane",
            "value": 588,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "reference/benzene",
            "value": 568,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "reference/ibuprofen",
            "value": 1579,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "reference/caffeine",
            "value": 1394,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10",
            "value": 6855,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10",
            "value": 16001,
            "range": "± 354",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/100",
            "value": 72921,
            "range": "± 184",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/100",
            "value": 57809,
            "range": "± 2324",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/1000",
            "value": 761662,
            "range": "± 4859",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/1000",
            "value": 410551,
            "range": "± 25745",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/5000",
            "value": 3742751,
            "range": "± 61198",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/5000",
            "value": 1911921,
            "range": "± 85550",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10000",
            "value": 7479706,
            "range": "± 47022",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10000",
            "value": 3718875,
            "range": "± 48005",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/50000",
            "value": 36765858,
            "range": "± 725523",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/50000",
            "value": 17895953,
            "range": "± 192444",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/100",
            "value": 18609,
            "range": "± 113",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/500",
            "value": 89445,
            "range": "± 1017",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/1000",
            "value": 176699,
            "range": "± 508",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/5000",
            "value": 876325,
            "range": "± 1468",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/100",
            "value": 28297,
            "range": "± 184",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/500",
            "value": 136496,
            "range": "± 577",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/1000",
            "value": 269953,
            "range": "± 2262",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/5000",
            "value": 1342726,
            "range": "± 18297",
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
          "id": "7d861d4e89b2a552accd4216058f63ac82cef183",
          "message": "feat: add stereochemistry parsing (chirality + directional bonds) (#16)\n\n* feat: add stereochemistry parsing (chirality + directional bonds)\n\nImplement full OpenSMILES stereochemistry support:\n\n- Chirality: @/@@ (tetrahedral), @TH, @AL, @SP, @TB1-20, @OH1-30\n- Directional bonds: / (Up) and \\ (Down) for cis/trans geometry\n- Chirality enum uses #[repr(u8)] for 1-byte Option<Chirality>\n- 39 new tests covering all chirality classes, bond geometry, and errors\n\nhttps://claude.ai/code/session_012vAwPmWEub96F9WWPhzFGb\n\n* fix: resolve fmt and clippy warnings\n\n- Use (1..=N).contains() instead of manual range checks (clippy)\n- Suppress too_many_arguments on add_atom (inherited pattern)\n- Apply rustfmt formatting fixes\n\nhttps://claude.ai/code/session_012vAwPmWEub96F9WWPhzFGb\n\n* fix: add position to chirality error messages\n\nInvalidChiralityClass and InvalidChiralitySpec now include the\ncharacter position where the error occurred, consistent with\nUnexpectedCharacter.\n\nhttps://claude.ai/code/session_012vAwPmWEub96F9WWPhzFGb\n\n---------\n\nCo-authored-by: Claude <noreply@anthropic.com>",
          "timestamp": "2026-02-06T09:18:39+01:00",
          "tree_id": "1da7621416b50663c20b08b6dbc5cebe15438eee",
          "url": "https://github.com/Peariforme/bigsmiles-rs/commit/7d861d4e89b2a552accd4216058f63ac82cef183"
        },
        "date": 1770366250713,
        "tool": "cargo",
        "benches": [
          {
            "name": "reference/ethanol",
            "value": 281,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "reference/cyclohexane",
            "value": 600,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "reference/benzene",
            "value": 597,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "reference/ibuprofen",
            "value": 1824,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "reference/caffeine",
            "value": 1551,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10",
            "value": 7716,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10",
            "value": 16409,
            "range": "± 251",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/100",
            "value": 81677,
            "range": "± 448",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/100",
            "value": 60252,
            "range": "± 1693",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/1000",
            "value": 827118,
            "range": "± 3413",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/1000",
            "value": 440619,
            "range": "± 19342",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/5000",
            "value": 4053555,
            "range": "± 21083",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/5000",
            "value": 1971905,
            "range": "± 28896",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10000",
            "value": 8093848,
            "range": "± 34172",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10000",
            "value": 3824526,
            "range": "± 31714",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/50000",
            "value": 41028084,
            "range": "± 380685",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/50000",
            "value": 19452040,
            "range": "± 401578",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/100",
            "value": 20740,
            "range": "± 195",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/500",
            "value": 96016,
            "range": "± 1466",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/1000",
            "value": 197664,
            "range": "± 2167",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/5000",
            "value": 953742,
            "range": "± 13580",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/100",
            "value": 34277,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/500",
            "value": 167397,
            "range": "± 13943",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/1000",
            "value": 332856,
            "range": "± 761",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/5000",
            "value": 1652685,
            "range": "± 15052",
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
          "id": "3e0130e2e102c80002f49cafc34b513dbcedfe7e",
          "message": "feat: OpenSMILES compliance audit and fixes (#17)\n\n* feat: OpenSMILES compliance audit and fixes\n\nAudit the parser against the OpenSMILES specification and fix all\nremaining non-compliance issues (6 fixes, 48/49 spec features now pass):\n\n- Parse aromatic two-letter bracket symbols [se], [as], [te]\n- Reject [HH1] — hydrogen atom cannot have a hydrogen count\n- Reject self-bonds (C11) — atom cannot be bonded to itself\n- Reject duplicate bonds (C12CCCCC12) — no two bonds between same pair\n- Whitespace terminator — SPACE/TAB/LF/CR stops parsing per spec\n- Fix disconnected structures — dot no longer creates a bond in the graph\n\nAlso un-ignores all 8 stoichiometry tests (now passing), adds 32 new\ncompliance tests, and updates README with detailed compliance tables.\n\nThe only remaining gap is Hückel's rule aromaticity validation.\n\nhttps://claude.ai/code/session_01A5NN91Pu2YdBZdb7EmeNS4\n\n* style: apply cargo fmt formatting\n\nhttps://claude.ai/code/session_01A5NN91Pu2YdBZdb7EmeNS4\n\n* refactor: remove redundant uppercase conversions in parser\n\nAtomSymbol::from_str already normalizes to uppercase internally, so\nthe manual capitalization before calling it was unnecessary.\n\nhttps://claude.ai/code/session_01A5NN91Pu2YdBZdb7EmeNS4\n\n---------\n\nCo-authored-by: Claude <noreply@anthropic.com>",
          "timestamp": "2026-02-06T12:39:31+01:00",
          "tree_id": "a0740ad938868373110936c052db239cc7289c80",
          "url": "https://github.com/Peariforme/bigsmiles-rs/commit/3e0130e2e102c80002f49cafc34b513dbcedfe7e"
        },
        "date": 1770378316103,
        "tool": "cargo",
        "benches": [
          {
            "name": "reference/ethanol",
            "value": 253,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "reference/cyclohexane",
            "value": 595,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "reference/benzene",
            "value": 580,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "reference/ibuprofen",
            "value": 1751,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "reference/caffeine",
            "value": 1590,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10",
            "value": 7208,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10",
            "value": 16124,
            "range": "± 324",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/100",
            "value": 76069,
            "range": "± 8498",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/100",
            "value": 59032,
            "range": "± 637",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/1000",
            "value": 789087,
            "range": "± 14394",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/1000",
            "value": 428228,
            "range": "± 16639",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/5000",
            "value": 3874692,
            "range": "± 14999",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/5000",
            "value": 1919002,
            "range": "± 37439",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10000",
            "value": 7713919,
            "range": "± 65907",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10000",
            "value": 3731310,
            "range": "± 52031",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/50000",
            "value": 39516787,
            "range": "± 1340924",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/50000",
            "value": 19537973,
            "range": "± 900762",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/100",
            "value": 18868,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/500",
            "value": 89829,
            "range": "± 268",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/1000",
            "value": 176832,
            "range": "± 3017",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/5000",
            "value": 902858,
            "range": "± 17996",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/100",
            "value": 32453,
            "range": "± 125",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/500",
            "value": 155811,
            "range": "± 845",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/1000",
            "value": 309210,
            "range": "± 6966",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/5000",
            "value": 1540139,
            "range": "± 17290",
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
          "id": "babebc6d9d0d5be46ff6037327e18b50c7a80b98",
          "message": "fix: simplify CI by merging feature-powerset testing into single job\n\nRemove redundant feature-tests job — cargo hack --feature-powerset\nalready covers all individual and combined feature configurations.\nReplace cargo test and cargo clippy with their cargo hack equivalents\nin the existing build job. (#19)\n\n* docs: add implementation plan for Hückel's rule aromaticity validation\n\nFour-phase plan covering element data (Z, mass, valence electrons),\nring detection (SSSR algorithm), pi electron counting, and integration\ninto the parse pipeline via feature flag.\n\nhttps://claude.ai/code/session_01NkMRmpmH7HmsJHuViYWrXw\n\n* feat: implement Hückel's rule aromaticity validation\n\nFour-phase implementation following the plan in PLAN_HUCKEL.md:\n\n- Phase 1: Add element data (atomic number, standard mass, valence electrons)\n  for all 118 elements via AtomSymbol::element_data(), plus isotope mass lookup\n  and Atom::mass() convenience method.\n\n- Phase 2: Ring detection using shortest-cycle-per-edge algorithm. For each\n  aromatic bond, removes the edge and BFS-finds the shortest alternative path\n  to identify minimal rings. Handles fused systems (naphthalene, indole)\n  correctly via deduplication.\n\n- Phase 3: Pi electron counting and Hückel validation (4n+2 rule).\n  Determines pi contribution per atom based on element type, charge, and\n  hydrogen count (pyrrole-type N/P/As with H → 2e⁻, pyridine-type → 1e⁻,\n  O/S/Se/Te → 2e⁻, B → 0e⁻, C±charge variants).\n\n- Phase 4: Integration into parse() behind `huckel-validation` feature flag.\n  Public API (validate_aromaticity, require_valid_aromaticity) available\n  regardless of feature flag for explicit use.\n\nhttps://claude.ai/code/session_01Cf5QyYdYkk7jXNeZApkjDr\n\n* feat: add Hückel benchmarks, CI feature testing, and update docs\n\n- Benchmarks: add `huckel` benchmark group comparing parse-only vs\n  parse+validate for aromatic molecules (benzene, naphthalene, etc.)\n  to measure Hückel validation overhead.\n\n- CI: add `feature-tests` job using cargo-hack to test all feature flag\n  combinations (--feature-powerset for check/clippy, --each-feature\n  for tests, --all-features for full test run).\n\n- Compare page: add \"Hückel Validation Overhead\" section with side-by-side\n  time chart and overhead % chart, plus detailed results table.\n\n- README: mark Hückel compliance as done (49/49), add feature flags\n  table, document `huckel-validation` feature with usage example,\n  update aromaticity compliance row and roadmap.\n\n- Format: apply cargo fmt to new files.\n\nhttps://claude.ai/code/session_01Cf5QyYdYkk7jXNeZApkjDr\n\n* fix: simplify CI by merging feature-powerset testing into single job\n\nRemove redundant feature-tests job — cargo hack --feature-powerset\nalready covers all individual and combined feature configurations.\nReplace cargo test and cargo clippy with their cargo hack equivalents\nin the existing build job.\n\nhttps://claude.ai/code/session_01Cf5QyYdYkk7jXNeZApkjDr\n\n* fix: conditional assertion for selenium ring with huckel-validation\n\nc1cc[se]cc1 is a 6-membered ring with Se giving 7 pi electrons (not\n4n+2). Without huckel-validation, test asserts successful parse (valid\nsyntax). With huckel-validation, test asserts parse error (chemically\ninvalid aromaticity).\n\nhttps://claude.ai/code/session_01Cf5QyYdYkk7jXNeZApkjDr\n\n* style: fix rustfmt formatting in selenium test assertion\n\nhttps://claude.ai/code/session_01Cf5QyYdYkk7jXNeZApkjDr\n\n* Delete PLAN_HUCKEL.md\n\n* perf: make element_data() and related methods const fn\n\nAll element data (atomic number, mass, valence electrons) is purely\nstatic. Marking these functions as const fn allows the compiler to\nevaluate them at compile time when the element variant is known.\n\nhttps://claude.ai/code/session_01Cf5QyYdYkk7jXNeZApkjDr\n\n* fix pi electron calculation\n\n---------\n\nCo-authored-by: Claude <noreply@anthropic.com>",
          "timestamp": "2026-02-11T15:24:59+01:00",
          "tree_id": "9aceb7e1e8631104eefd3dd72dee04cf2798f96f",
          "url": "https://github.com/Peariforme/bigsmiles-rs/commit/babebc6d9d0d5be46ff6037327e18b50c7a80b98"
        },
        "date": 1770820339370,
        "tool": "cargo",
        "benches": [
          {
            "name": "reference/ethanol",
            "value": 228,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "reference/cyclohexane",
            "value": 549,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "reference/benzene",
            "value": 554,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "reference/ibuprofen",
            "value": 1522,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "reference/caffeine",
            "value": 1465,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10",
            "value": 6653,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10",
            "value": 16029,
            "range": "± 1582",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/100",
            "value": 72521,
            "range": "± 251",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/100",
            "value": 56078,
            "range": "± 2828",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/1000",
            "value": 734157,
            "range": "± 1621",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/1000",
            "value": 394374,
            "range": "± 12524",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/5000",
            "value": 3563040,
            "range": "± 11897",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/5000",
            "value": 1749198,
            "range": "± 50252",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10000",
            "value": 7110814,
            "range": "± 105475",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10000",
            "value": 3394286,
            "range": "± 35697",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/50000",
            "value": 35435788,
            "range": "± 393092",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/50000",
            "value": 17192668,
            "range": "± 296746",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/100",
            "value": 15617,
            "range": "± 215",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/500",
            "value": 73522,
            "range": "± 1704",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/1000",
            "value": 145369,
            "range": "± 1131",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/5000",
            "value": 713006,
            "range": "± 1276",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/100",
            "value": 30835,
            "range": "± 147",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/500",
            "value": 147762,
            "range": "± 707",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/1000",
            "value": 293005,
            "range": "± 704",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/5000",
            "value": 1450970,
            "range": "± 20862",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/ethanol",
            "value": 234,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/ethanol",
            "value": 299,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/benzene",
            "value": 550,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/benzene",
            "value": 2523,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/naphthalene",
            "value": 903,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/naphthalene",
            "value": 5101,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/ibuprofen",
            "value": 1490,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/ibuprofen",
            "value": 3583,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/caffeine",
            "value": 1401,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/caffeine",
            "value": 1498,
            "range": "± 7",
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
          "id": "f3358c665214b1726222bc7ac8a1fc2ac52c8b09",
          "message": "Display molecule (#20)\n\n* simple molecule display + standard cycle\n\n* fix bracket parsing with no property order\n\n* normalize removable hydrogens\n\n* Standardize kekule rings\n\n* cargo fmt\n\n* standardize removable chirality\n\n* cargo fmt",
          "timestamp": "2026-02-25T10:25:33+01:00",
          "tree_id": "78472e4240ccd5f4abd69ba4e0a5625fcf87dde6",
          "url": "https://github.com/Peariforme/bigsmiles-rs/commit/f3358c665214b1726222bc7ac8a1fc2ac52c8b09"
        },
        "date": 1772011979989,
        "tool": "cargo",
        "benches": [
          {
            "name": "reference/ethanol",
            "value": 204,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "reference/cyclohexane",
            "value": 573,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "reference/benzene",
            "value": 566,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "reference/ibuprofen",
            "value": 1597,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "reference/caffeine",
            "value": 1444,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10",
            "value": 6700,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10",
            "value": 14970,
            "range": "± 179",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/100",
            "value": 69709,
            "range": "± 414",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/100",
            "value": 70962,
            "range": "± 5063",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/1000",
            "value": 720138,
            "range": "± 12193",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/1000",
            "value": 460707,
            "range": "± 9329",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/5000",
            "value": 3507977,
            "range": "± 5401",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/5000",
            "value": 2131333,
            "range": "± 27376",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10000",
            "value": 7102587,
            "range": "± 27205",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10000",
            "value": 4196885,
            "range": "± 25400",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/50000",
            "value": 36479004,
            "range": "± 605078",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/50000",
            "value": 21098555,
            "range": "± 654481",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/100",
            "value": 14557,
            "range": "± 115",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/500",
            "value": 67080,
            "range": "± 1329",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/1000",
            "value": 131518,
            "range": "± 3473",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/5000",
            "value": 652383,
            "range": "± 2005",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/100",
            "value": 25788,
            "range": "± 125",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/500",
            "value": 120682,
            "range": "± 1502",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/1000",
            "value": 238674,
            "range": "± 1345",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/5000",
            "value": 1183894,
            "range": "± 30541",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/ethanol",
            "value": 236,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/ethanol",
            "value": 317,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/benzene",
            "value": 563,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/benzene",
            "value": 2636,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/naphthalene",
            "value": 971,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/naphthalene",
            "value": 5178,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/ibuprofen",
            "value": 1548,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/ibuprofen",
            "value": 3714,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/caffeine",
            "value": 1409,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/caffeine",
            "value": 1542,
            "range": "± 2",
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
          "id": "d8a066fbf1fca98b7a4c1f680463eb9ac54865e7",
          "message": "Prepare to publish (#21)\n\n* feat: rename smiles-core to opensmiles and prepare for crates.io\n\n- Rename crate from smiles-core to opensmiles (Cargo.toml, imports, tests)\n- Clean public API: MoleculeBuilder, NodeBuilder, bond internals → pub(crate)\n- Remove dead code: unused NodeBuilder methods, build_spanning_tree()\n- Add /// doc comments on all public types and functions\n- Add README.md for the opensmiles crate with usage examples, feature table,\n  benchmark results and links, and compliance summary\n- Add documentation field and readme field in Cargo.toml\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>\n\n* ci: add release-plz for automated crates.io publishing\n\n- Add .github/workflows/release-plz.yml: on push to master, release-plz\n  creates a release PR with bumped version and CHANGELOG; merging the PR\n  triggers automatic publish to crates.io\n- Add release-plz.toml: configure opensmiles as publishable,\n  bigsmiles-core as skipped (not ready)\n- Add .githooks/commit-msg: enforces Conventional Commits format\n  (feat/fix/docs/... with optional scope and breaking change marker)\n  Install with: git config core.hooksPath .githooks\n- Fix ci.yml: update cargo-hack references from smiles-core to opensmiles\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>\n\n* ci: add pre-commit and pre-push git hooks\n\n- pre-commit: runs cargo fmt --check and cargo hack clippy (all feature\n  combinations) to catch formatting and lint issues before commit\n- pre-push: runs cargo hack test (all feature combinations) before push\n- fix: apply cargo fmt to molecule.rs (add_branch signature line length)\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>\n\n* fix: update benchmark workflow to use opensmiles crate name\n\n* fix: benchmark reference to opensmiles\n\n---------\n\nCo-authored-by: Claude Sonnet 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-02-25T11:34:12+01:00",
          "tree_id": "355c104da03f830704689f2db5234f67981ab43c",
          "url": "https://github.com/Peariforme/bigsmiles-rs/commit/d8a066fbf1fca98b7a4c1f680463eb9ac54865e7"
        },
        "date": 1772016098087,
        "tool": "cargo",
        "benches": [
          {
            "name": "reference/ethanol",
            "value": 231,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "reference/cyclohexane",
            "value": 544,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "reference/benzene",
            "value": 544,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "reference/ibuprofen",
            "value": 1530,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "reference/caffeine",
            "value": 1433,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10",
            "value": 6830,
            "range": "± 129",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10",
            "value": 15858,
            "range": "± 3447",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/100",
            "value": 72232,
            "range": "± 301",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/100",
            "value": 56292,
            "range": "± 1612",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/1000",
            "value": 741736,
            "range": "± 14574",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/1000",
            "value": 401185,
            "range": "± 14243",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/5000",
            "value": 3608827,
            "range": "± 185212",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/5000",
            "value": 1750743,
            "range": "± 40234",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10000",
            "value": 7201607,
            "range": "± 334137",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10000",
            "value": 3422551,
            "range": "± 73564",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/50000",
            "value": 36645330,
            "range": "± 688478",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/50000",
            "value": 17213791,
            "range": "± 709090",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/100",
            "value": 15558,
            "range": "± 234",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/500",
            "value": 74111,
            "range": "± 1198",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/1000",
            "value": 144989,
            "range": "± 815",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/5000",
            "value": 720489,
            "range": "± 2633",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/100",
            "value": 29871,
            "range": "± 723",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/500",
            "value": 143573,
            "range": "± 3878",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/1000",
            "value": 285359,
            "range": "± 7781",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/5000",
            "value": 1415012,
            "range": "± 12474",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/ethanol",
            "value": 241,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/ethanol",
            "value": 306,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/benzene",
            "value": 548,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/benzene",
            "value": 2454,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/naphthalene",
            "value": 909,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/naphthalene",
            "value": 4917,
            "range": "± 278",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/ibuprofen",
            "value": 1483,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/ibuprofen",
            "value": 3526,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/caffeine",
            "value": 1404,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/caffeine",
            "value": 1512,
            "range": "± 7",
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
            "email": "47952322+Peariforme@users.noreply.github.com",
            "name": "Peariforme",
            "username": "Peariforme"
          },
          "distinct": true,
          "id": "a0318676b045f061bbc3c4c6cdb8230682a762e8",
          "message": "fix: remove invalid dependencies_update field from release-plz.toml",
          "timestamp": "2026-02-25T11:36:07+01:00",
          "tree_id": "0dd7b0dd10edb3caa5b1ab79db053713917cffe0",
          "url": "https://github.com/Peariforme/bigsmiles-rs/commit/a0318676b045f061bbc3c4c6cdb8230682a762e8"
        },
        "date": 1772016241154,
        "tool": "cargo",
        "benches": [
          {
            "name": "reference/ethanol",
            "value": 228,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "reference/cyclohexane",
            "value": 558,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "reference/benzene",
            "value": 544,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "reference/ibuprofen",
            "value": 1512,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "reference/caffeine",
            "value": 1446,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10",
            "value": 6764,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10",
            "value": 15969,
            "range": "± 1836",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/100",
            "value": 71369,
            "range": "± 270",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/100",
            "value": 56081,
            "range": "± 6737",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/1000",
            "value": 711617,
            "range": "± 39842",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/1000",
            "value": 393281,
            "range": "± 20895",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/5000",
            "value": 3632635,
            "range": "± 26419",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/5000",
            "value": 1738618,
            "range": "± 43252",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10000",
            "value": 7234651,
            "range": "± 40477",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10000",
            "value": 3410644,
            "range": "± 53367",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/50000",
            "value": 36226402,
            "range": "± 524216",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/50000",
            "value": 16885483,
            "range": "± 425025",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/100",
            "value": 16776,
            "range": "± 440",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/500",
            "value": 75945,
            "range": "± 1179",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/1000",
            "value": 144556,
            "range": "± 1243",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/5000",
            "value": 720402,
            "range": "± 14691",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/100",
            "value": 30207,
            "range": "± 363",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/500",
            "value": 143544,
            "range": "± 677",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/1000",
            "value": 285348,
            "range": "± 1208",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/5000",
            "value": 1421897,
            "range": "± 30118",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/ethanol",
            "value": 237,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/ethanol",
            "value": 303,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/benzene",
            "value": 538,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/benzene",
            "value": 2460,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/naphthalene",
            "value": 904,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/naphthalene",
            "value": 4909,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/ibuprofen",
            "value": 1506,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/ibuprofen",
            "value": 3489,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/caffeine",
            "value": 1372,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/caffeine",
            "value": 1507,
            "range": "± 6",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9775a3943450992977429896222398ffdd2d0ce6",
          "message": "chore: release v0.1.0 (#22)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2026-02-25T11:42:14+01:00",
          "tree_id": "5721be9c1f13094d2fc633f8b4df0c009b0228d9",
          "url": "https://github.com/Peariforme/bigsmiles-rs/commit/9775a3943450992977429896222398ffdd2d0ce6"
        },
        "date": 1772016581800,
        "tool": "cargo",
        "benches": [
          {
            "name": "reference/ethanol",
            "value": 234,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "reference/cyclohexane",
            "value": 555,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "reference/benzene",
            "value": 573,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "reference/ibuprofen",
            "value": 1526,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "reference/caffeine",
            "value": 1440,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10",
            "value": 6804,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10",
            "value": 16782,
            "range": "± 1176",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/100",
            "value": 70321,
            "range": "± 167",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/100",
            "value": 57348,
            "range": "± 2092",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/1000",
            "value": 735864,
            "range": "± 1413",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/1000",
            "value": 399584,
            "range": "± 40081",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/5000",
            "value": 3624556,
            "range": "± 73548",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/5000",
            "value": 1759284,
            "range": "± 54515",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/10000",
            "value": 7252770,
            "range": "± 39911",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/10000",
            "value": 3389858,
            "range": "± 68266",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/sequential/50000",
            "value": 36568126,
            "range": "± 204433",
            "unit": "ns/iter"
          },
          {
            "name": "seq_vs_parallel/parallel/50000",
            "value": 17220092,
            "range": "± 176577",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/100",
            "value": 15136,
            "range": "± 1146",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/500",
            "value": 73099,
            "range": "± 141",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/1000",
            "value": 119305,
            "range": "± 6860",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/peg/5000",
            "value": 662218,
            "range": "± 8754",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/100",
            "value": 30865,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/500",
            "value": 151367,
            "range": "± 236",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/1000",
            "value": 301802,
            "range": "± 1449",
            "unit": "ns/iter"
          },
          {
            "name": "scaling/teflon/5000",
            "value": 1493330,
            "range": "± 12867",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/ethanol",
            "value": 243,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/ethanol",
            "value": 315,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/benzene",
            "value": 586,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/benzene",
            "value": 2482,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/naphthalene",
            "value": 951,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/naphthalene",
            "value": 5066,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/ibuprofen",
            "value": 1499,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/ibuprofen",
            "value": 3580,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_only/caffeine",
            "value": 1400,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "huckel/parse_and_validate/caffeine",
            "value": 1518,
            "range": "± 3",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}