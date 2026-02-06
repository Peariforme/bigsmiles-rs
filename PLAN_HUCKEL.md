# Plan d'implementation : Validation de la regle de Huckel

## Vue d'ensemble

Ajout de la validation de la regle de Huckel (4n+2 electrons pi) pour
controler l'aromaticite des cycles dans le parseur SMILES. L'implementation
se decompose en 4 phases independantes et mergeables.

---

## Phase 1 : Donnees elementaires (numero atomique, masse, electrons de valence)

### Objectif

`AtomSymbol` ne porte actuellement aucune donnee numerique. On doit ajouter
pour chaque element : le numero atomique (Z), la masse atomique standard,
et le nombre d'electrons de valence.

### Fichiers

| Action   | Fichier                                              |
|----------|------------------------------------------------------|
| Creer    | `crates/smiles-core/src/ast/element_data.rs`         |
| Modifier | `crates/smiles-core/src/ast/atom.rs`                 |
| Modifier | `crates/smiles-core/src/ast/mod.rs`                  |

### Structure de donnees

```rust
/// Donnees chimiques statiques d'un element.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElementData {
    pub atomic_number: u8,       // 1..=118, 0 pour Wildcard
    pub standard_mass: f64,      // masse atomique standard en Da
    pub valence_electrons: u8,   // electrons de valence (groupes principaux)
}
```

Methodes ajoutees sur `AtomSymbol` :

```rust
impl AtomSymbol {
    pub fn element_data(&self) -> ElementData { /* match exhaustif */ }
    pub fn atomic_number(&self) -> u8 { self.element_data().atomic_number }
    pub fn standard_mass(&self) -> f64 { self.element_data().standard_mass }
    pub fn valence_electrons(&self) -> u8 { self.element_data().valence_electrons }
}
```

### Valeurs cles (elements aromatiques)

| Element | Z  | Masse (Da) | e- valence |
|---------|----|------------|------------|
| B       | 5  | 10.81      | 3          |
| C       | 6  | 12.011     | 4          |
| N       | 7  | 14.007     | 5          |
| O       | 8  | 15.999     | 6          |
| P       | 15 | 30.974     | 5          |
| S       | 16 | 32.06      | 6          |
| As      | 33 | 74.922     | 5          |
| Se      | 34 | 78.971     | 6          |
| Te      | 52 | 127.60     | 6          |

### Masse isotopique

`Atom` possede deja `isotope: Option<u16>` (nombre de masse). On ajoute
une methode pour calculer la masse reelle :

```rust
impl Atom {
    /// Retourne la masse : isotopique si specifiee, standard sinon.
    pub fn mass(&self) -> f64 {
        match self.isotope {
            Some(mass_number) => isotope_mass(self.element(), mass_number),
            None => self.element().standard_mass(),
        }
    }
}
```

Pour les masses isotopiques, on utilise une table const des isotopes les
plus courants (H, D, T, 13C, 14C, 15N, 18O, etc.). Pour les isotopes
absents de la table, on retourne le nombre de masse comme approximation.

### Tests

- Verifier Z et e- de valence pour tous les `OrganicAtom`
- Verifier les 9 elements aromatiques specifiquement
- Verifier Wildcard retourne 0
- Verifier unicite des numeros atomiques
- Verifier masse standard vs masse isotopique

---

## Phase 2 : Liste d'adjacence et detection des cycles

### Objectif

Le `Molecule` stocke les liaisons dans un `Vec<Bond>` plat. Pour detecter
les cycles, il faut construire une liste d'adjacence et implementer
l'algorithme SSSR (Smallest Set of Smallest Rings).

### Fichiers

| Action   | Fichier                                          |
|----------|--------------------------------------------------|
| Creer    | `crates/smiles-core/src/ast/graph.rs`            |
| Modifier | `crates/smiles-core/src/ast/mod.rs`              |
| Modifier | `crates/smiles-core/src/ast/molecule.rs`         |

### Structures de donnees

```rust
/// Liste d'adjacence construite a partir des liaisons d'une molecule.
pub struct AdjacencyList {
    adj: Vec<Vec<(u16, usize)>>,  // (indice_voisin, indice_liaison)
}

/// Un cycle represente par la sequence ordonnee des indices de noeuds.
#[derive(Debug, Clone, PartialEq)]
pub struct Ring {
    pub nodes: Vec<u16>,
}
```

### Algorithme

```
find_aromatic_rings(molecule) -> Vec<Ring>:
  1. Construire le sous-graphe aromatique :
     - Garder seulement les noeuds ou aromatic == true
     - Garder seulement les liaisons BondType::Aromatic
       entre deux noeuds aromatiques

  2. Trouver les composantes connexes (BFS/DFS)

  3. Pour chaque composante connexe :
     a. Construire un arbre couvrant (BFS)
     b. Identifier les aretes hors-arbre (back edges)
     c. Pour chaque arete hors-arbre (u, v) :
        - Remonter de u et v vers l'ancetre commun (LCA)
        - Le cycle fondamental = chemin(u -> LCA) + chemin(v -> LCA) inverse
     d. Nombre de cycles = |E| - |V| + 1

  4. Retourner tous les cycles collectes
```

### Methode de commodite sur Molecule

```rust
impl Molecule {
    pub fn aromatic_rings(&self) -> Vec<Ring> {
        find_aromatic_rings(self)
    }
}
```

### Tests

| Test                       | Entree SMILES       | Resultat attendu                 |
|----------------------------|----------------------|----------------------------------|
| benzene                    | `c1ccccc1`           | 1 cycle de taille 6              |
| naphthalene                | `c1ccc2ccccc2c1`     | 2 cycles de taille 6             |
| biphenyl                   | `c1ccccc1c1ccccc1`   | 2 cycles separes de taille 6     |
| cyclohexane (non aromat.)  | `C1CCCCC1`           | 0 cycles (pas de noeuds aromat.) |
| indole                     | `c1ccc2[nH]ccc2c1`   | 2 cycles (5 + 6)                 |
| toluene                    | `Cc1ccccc1`          | 1 cycle (methyle exclu)          |

---

## Phase 3 : Comptage des electrons pi et validation de Huckel

### Objectif

Pour chaque cycle aromatique detecte, compter la contribution en electrons
pi de chaque atome, puis verifier que le total satisfait 4n+2.

### Fichiers

| Action   | Fichier                                             |
|----------|-----------------------------------------------------|
| Creer    | `crates/smiles-core/src/ast/aromaticity.rs`         |
| Creer    | `crates/smiles-core/tests/huckel.rs`                |
| Modifier | `crates/smiles-core/src/ast/mod.rs`                 |
| Modifier | `crates/smiles-core/src/error/molecule.rs`          |

### Regles de contribution pi

La contribution depend de l'element, de la charge, et du contexte de liaison :

| Element | Contribution | Condition                                         |
|---------|-------------|---------------------------------------------------|
| C       | 1           | cas standard                                      |
| C-      | 2           | anion (cyclopentadienyle)                         |
| C+      | 0           | cation (tropylium)                                |
| N       | 1           | type pyridine (=N- dans le cycle, pas de H)       |
| N       | 2           | type pyrrole ([nH], paire libre donnee)           |
| O       | 2           | toujours donneur de paire libre (furane)          |
| S       | 2           | toujours donneur de paire libre (thiophene)       |
| B       | 0           | orbitale p vide                                   |
| P       | 1 ou 2      | analogue a N (depend du contexte)                 |
| Se      | 2           | analogue a S                                      |
| As      | 1 ou 2      | analogue a P                                      |
| Te      | 2           | analogue a Se                                     |

### Algorithme de determination

```
pi_electron_contribution(node, ring_bonds_count, total_bonds) -> Option<u8>:
  element = node.atom().element()
  charge  = node.atom().charge()
  has_h   = node.hydrogens() > 0

  match element:
    C  => match charge { -1 => 2, +1 => 0, _ => 1 }
    N  => if has_h || (ring_bonds_count == 2 && total_bonds == 2) { 2 } else { 1 }
    O  => 2
    S  => 2
    B  => 0
    P  => if has_h || (ring_bonds_count == 2 && total_bonds == 2) { 2 } else { 1 }
    Se => 2
    As => if has_h || (ring_bonds_count == 2 && total_bonds == 2) { 2 } else { 1 }
    Te => 2
    Wildcard => None   // indetermine, skip validation
    _  => None
```

### Verification de Huckel

```rust
fn satisfies_huckel(pi_electrons: u8) -> bool {
    pi_electrons >= 2 && (pi_electrons - 2) % 4 == 0
}
// Valeurs valides : 2, 6, 10, 14, 18, 22...
```

### Structure de resultat

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct AromaticityCheck {
    pub ring: Ring,
    pub pi_electrons: Option<u8>,  // None si indetermine (Wildcard)
    pub is_valid: bool,
}

pub fn validate_aromaticity(molecule: &Molecule) -> Vec<AromaticityCheck>;
pub fn require_valid_aromaticity(molecule: &Molecule) -> Result<(), MoleculeError>;
```

### Erreur ajoutee

```rust
// dans error/molecule.rs
pub enum MoleculeError {
    // ...existant...
    #[error("aromatic ring {ring:?} has {pi_electrons} pi electrons (Huckel: 4n+2 required)")]
    HuckelViolation {
        ring: Vec<u16>,
        pi_electrons: u8,
    },
}
```

### Tests

| Test                          | SMILES              | e- pi | Valide |
|-------------------------------|----------------------|-------|--------|
| benzene                       | `c1ccccc1`           | 6     | oui    |
| pyridine                      | `c1ccncc1`           | 6     | oui    |
| pyrrole                       | `c1cc[nH]c1`         | 6     | oui    |
| furane                        | `c1ccoc1`            | 6     | oui    |
| thiophene                     | `c1ccsc1`            | 6     | oui    |
| imidazole                     | `c1cnc[nH]1`         | 6     | oui    |
| cyclopentadienyle anion       | `[c-]1cccc1`         | 6     | oui    |
| tropylium cation              | `[cH+]1cccccc1`      | 6     | oui    |
| naphthalene (chaque cycle)    | `c1ccc2ccccc2c1`     | 6     | oui    |
| borazine                      | `b1nbnbn1`           | 6     | oui    |

---

## Phase 4 : Integration dans le pipeline de parsing

### Objectif

Brancher la validation dans `parse()` via un feature flag optionnel.

### Fichiers

| Action   | Fichier                                          |
|----------|--------------------------------------------------|
| Modifier | `crates/smiles-core/Cargo.toml`                  |
| Modifier | `crates/smiles-core/src/parser.rs`               |

### Feature flag

```toml
# Cargo.toml
[features]
default = []
parallel = ["rayon"]
huckel-validation = []
```

### Integration dans parse()

```rust
pub fn parse(input: &str) -> Result<Molecule, ParserError> {
    // ... code existant ...
    let molecule = builder.build()?;

    #[cfg(feature = "huckel-validation")]
    {
        crate::ast::aromaticity::require_valid_aromaticity(&molecule)?;
    }

    Ok(molecule)
}
```

L'API publique `validate_aromaticity()` et `require_valid_aromaticity()`
reste accessible meme sans le feature flag, pour un usage explicite.

---

## Graphe de dependances

```
Phase 1: element_data.rs ─────────┐
     (independant)                 ├──> Phase 3: aromaticity.rs ──> Phase 4: Integration
Phase 2: graph.rs ────────────────┘
     (independant)
```

Les phases 1 et 2 sont independantes et peuvent etre developpees en parallele.

---

## Cas limites

1. **Wildcard dans un cycle** : skip la validation (indetermine)
2. **Atomes charges** : C- = 2e-, C+ = 0e-, N+ depend du contexte
3. **Systemes fusionnes** : le SSSR decompose en cycles fondamentaux,
   chacun valide independamment
4. **Doubles liaisons exocycliques** : changent la contribution pi,
   a detecter via le contexte de liaison total
5. **Cycles partiellement aromatiques** : pas de cycle detecte = pas de validation
6. **Borazine** : syntaxiquement valide (6 e- pi), meme si pas
   physiquement aromatique - correct pour un validateur syntaxique
7. **Fragments disconnectes** : chaque composante traitee independamment

---

## Resume des fichiers

### Nouveaux fichiers (4)
- `crates/smiles-core/src/ast/element_data.rs`
- `crates/smiles-core/src/ast/graph.rs`
- `crates/smiles-core/src/ast/aromaticity.rs`
- `crates/smiles-core/tests/huckel.rs`

### Fichiers modifies (6)
- `crates/smiles-core/src/ast/atom.rs`
- `crates/smiles-core/src/ast/mod.rs`
- `crates/smiles-core/src/ast/molecule.rs`
- `crates/smiles-core/src/error/molecule.rs`
- `crates/smiles-core/src/parser.rs`
- `crates/smiles-core/Cargo.toml`
