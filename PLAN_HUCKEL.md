# Plan d'implementation : Validation de la regle de Huckel (v2)

## Vue d'ensemble

Ajout de la validation de la regle de Huckel (4n+2 electrons pi) pour
controler l'aromaticite des cycles dans le parseur SMILES. L'implementation
se decompose en 4 phases.

Changements majeurs par rapport a la v1 :
- **Phase 1** : configuration electronique complete (sous-couches s, p, d, f)
  au lieu d'un simple `valence_electrons: u8`. Pas d'approximation pour les
  isotopes : un isotope a le meme Z que l'element standard, donc la meme
  configuration electronique.
- **Phase 2** : detection des cycles **au moment du parsing** (a la fermeture
  du ring bond) au lieu d'un algorithme SSSR post-hoc. Les noeuds du cycle
  sont recuperes en remontant le graphe entre source et target de la bond
  de fermeture.

---

## Phase 1 : Configuration electronique (sous-couches s, p, d, f)

### Objectif

Remplacer le concept de `valence_electrons: u8` par une configuration
electronique reelle. Pour chaque element, on calcule la repartition des
electrons sur les sous-couches selon le principe d'Aufbau (regle de
Klechkowski), puis on en deduit le nombre d'electrons sur la couche la
plus eloignee.

### Pourquoi pas de simplification pour les isotopes

Un isotope a le meme nombre de protons (Z) que l'element standard.
Seul le nombre de neutrons change. La configuration electronique depend
uniquement de Z, pas du nombre de masse. Donc `[13C]` a exactement la meme
configuration electronique que `[C]` : `[He] 2s2 2p2`. Il n'y a aucune
approximation a faire.

### Fichiers

| Action   | Fichier                                              |
|----------|------------------------------------------------------|
| Creer    | `crates/smiles-core/src/ast/electron_config.rs`      |
| Modifier | `crates/smiles-core/src/ast/atom.rs`                 |
| Modifier | `crates/smiles-core/src/ast/mod.rs`                  |

### Structures de donnees

```rust
/// Sous-couche electronique identifiee par ses nombres quantiques.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Subshell {
    pub n: u8,  // nombre quantique principal (1, 2, 3, ...)
    pub l: u8,  // nombre quantique azimutal (0=s, 1=p, 2=d, 3=f)
}

/// Configuration electronique complete d'un atome neutre.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElectronConfiguration {
    /// Repartition des electrons : Vec de (sous-couche, nombre d'electrons).
    /// Ordonnee par ordre de remplissage (Aufbau).
    pub subshells: Vec<(Subshell, u8)>,
}
```

### Constantes de remplissage

Ordre d'Aufbau (regle de Klechkowski, n+l croissant, puis n croissant) :

```
1s  2s  2p  3s  3p  4s  3d  4p  5s  4d  5p  6s  4f  5d  6p  7s  5f  6d  7p
```

Capacite maximale par sous-couche : `2(2l + 1)`

| l | Nom | Capacite |
|---|-----|----------|
| 0 | s   | 2        |
| 1 | p   | 6        |
| 2 | d   | 10       |
| 3 | f   | 14       |

### Exceptions au remplissage standard (Aufbau)

Certains elements ont des configurations anormales (stabilisation par
demi-remplissage ou remplissage complet des sous-couches d et f).
Ces exceptions doivent etre codees en dur :

| Z  | Element | Config attendue (Aufbau)  | Config reelle             |
|----|---------|---------------------------|---------------------------|
| 24 | Cr      | [Ar] 3d4 4s2              | [Ar] 3d5 4s1              |
| 29 | Cu      | [Ar] 3d9 4s2              | [Ar] 3d10 4s1             |
| 41 | Nb      | [Kr] 4d3 5s2              | [Kr] 4d4 5s1              |
| 42 | Mo      | [Kr] 4d4 5s2              | [Kr] 4d5 5s1              |
| 44 | Ru      | [Kr] 4d6 5s2              | [Kr] 4d7 5s1              |
| 45 | Rh      | [Kr] 4d7 5s2              | [Kr] 4d8 5s1              |
| 46 | Pd      | [Kr] 4d8 5s2              | [Kr] 4d10 5s0             |
| 47 | Ag      | [Kr] 4d9 5s2              | [Kr] 4d10 5s1             |
| 78 | Pt      | [Xe] 4f14 5d8 6s2         | [Xe] 4f14 5d9 6s1         |
| 79 | Au      | [Xe] 4f14 5d9 6s2         | [Xe] 4f14 5d10 6s1        |

(Liste non exhaustive — d'autres exceptions existent pour les lanthanides
et actinides. On les ajoutera au fur et a mesure si necessaire, mais pour
les elements aromatiques courants B, C, N, O, P, S, Se, As, Te, l'Aufbau
standard suffit.)

### Table const pre-calculee

Pour eviter de recalculer la configuration electronique a chaque appel,
les resultats sont stockes dans un `const` array indexe par Z (0..=118).
Le calcul se fait a la compilation, pas au runtime :

```rust
/// Donnees pre-calculees pour chaque element.
/// Index 0 = Wildcard, index 1..=118 = elements.
struct ElementElectronData {
    outermost_shell: u8,      // n max
    outermost_electrons: u8,  // e- sur la couche n max
    outermost_p_electrons: u8, // e- p sur la couche n max
}

const ELECTRON_DATA: [ElementElectronData; 119] = precompute_all();
```

Cela donne un acces O(1) par `ELECTRON_DATA[z as usize]` sans aucune
allocation ni cache runtime.

### Methodes

```rust
impl ElectronConfiguration {
    /// Construit la configuration pour un element de numero atomique Z.
    pub fn from_atomic_number(z: u8) -> Self;

    /// Numero de la couche la plus eloignee (n max parmi les sous-couches occupees).
    pub fn outermost_shell(&self) -> u8;

    /// Nombre total d'electrons sur la couche la plus eloignee.
    /// C'est la somme des electrons dans toutes les sous-couches ou n == n_max.
    pub fn outermost_electrons(&self) -> u8;

    /// Nombre d'electrons dans une sous-couche specifique (0 si non occupee).
    pub fn electrons_in(&self, n: u8, l: u8) -> u8;

    /// Nombre d'electrons p sur la couche la plus eloignee.
    /// Utile pour determiner la contribution pi.
    pub fn outermost_p_electrons(&self) -> u8;
}
```

### Methodes ajoutees sur AtomSymbol

```rust
impl AtomSymbol {
    /// Numero atomique Z (1..=118, 0 pour Wildcard).
    pub fn atomic_number(&self) -> u8;

    /// Configuration electronique de l'element neutre.
    /// Pour la config complete (utile pour debug/inspection).
    pub fn electron_configuration(&self) -> ElectronConfiguration;

    /// Nombre d'electrons sur la couche la plus eloignee.
    /// Lecture directe dans ELECTRON_DATA — O(1), pas de calcul.
    pub fn outermost_electrons(&self) -> u8;

    /// Masse atomique standard en Da.
    pub fn standard_mass(&self) -> f64;
}
```

### Exemples (elements aromatiques)

| Element | Z  | Configuration                  | Couche ext. (n) | e- couche ext. |
|---------|----|--------------------------------|-----------------|----------------|
| B       | 5  | [He] 2s2 2p1                   | 2               | 3 (2s+1p)      |
| C       | 6  | [He] 2s2 2p2                   | 2               | 4 (2s+2p)      |
| N       | 7  | [He] 2s2 2p3                   | 2               | 5 (2s+3p)      |
| O       | 8  | [He] 2s2 2p4                   | 2               | 6 (2s+4p)      |
| P       | 15 | [Ne] 3s2 3p3                   | 3               | 5 (2s+3p)      |
| S       | 16 | [Ne] 3s2 3p4                   | 3               | 6 (2s+4p)      |
| As      | 33 | [Ar] 3d10 4s2 4p3              | 4               | 5 (2s+3p)      |
| Se      | 34 | [Ar] 3d10 4s2 4p4              | 4               | 6 (2s+4p)      |
| Te      | 52 | [Kr] 4d10 5s2 5p4              | 5               | 6 (2s+4p)      |

### Tests

- Configuration de H (Z=1) : 1s1, couche ext. = 1, e- ext. = 1
- Configuration de C (Z=6) : couche ext. = 2, e- ext. = 4
- Configuration de Fe (Z=26) : [Ar] 3d6 4s2, couche ext. = 4, e- ext. = 2
- Configuration de Cr (Z=24, exception) : [Ar] 3d5 4s1, couche ext. = 4, e- ext. = 1
- Configuration de Cu (Z=29, exception) : [Ar] 3d10 4s1, couche ext. = 4, e- ext. = 1
- Verifier que les 9 elements aromatiques ont les bonnes valeurs
- Verifier que Wildcard retourne 0
- Verifier l'unicite des numeros atomiques
- Verifier que les isotopes (`[13C]`, `[2H]`) ont la meme config que l'element standard

---

## Phase 2 : Detection des cycles au moment du parsing

### Objectif

Eliminer le besoin d'un algorithme SSSR post-hoc. Quand le parseur ferme
un cycle (ring closure bond), il connait deja les indices source et target.
On remonte le graphe des bonds existantes pour identifier les noeuds
participants du cycle a ce moment-la.

### Principe

Lors du parsing SMILES, un cycle est cree quand on rencontre un digit de
fermeture (ex: le second `1` dans `c1ccccc1`). A cet instant :
- `target` = indice global du noeud ou le cycle a ete ouvert
- `source` = indice global du noeud courant (ou le cycle se ferme)
- La bond de fermeture relie `source` a `target`

Les noeuds du cycle sont le **chemin** de `target` a `source` dans le graphe
des bonds deja construites (sans utiliser la bond de fermeture elle-meme).
Ce chemin existe forcement puisque tous les atomes entre l'ouverture et
la fermeture sont connectes par une chaine de bonds.

### Algorithme de recuperation du cycle

```
find_cycle_path(bonds, target, source, node_count) -> Vec<u16>:
    // BFS/DFS dans le graphe des bonds existantes (sans la bond de fermeture)
    // pour trouver le chemin de target a source.

    1. Construire une liste d'adjacence temporaire a partir de bonds
    2. BFS depuis target, en cherchant source
    3. Reconstruire le chemin via le tableau des parents
    4. Retourner le chemin [target, ..., source]
```

### Cas des cycles imbriques/fusionnes

Pour le naphthalene `c1ccc2ccccc2c1` :
- Premier cycle ferme : digit `2`, noeuds {3,4,5,6,7,8} (indices 3-8)
- Second cycle ferme : digit `1`, noeuds {0,1,2,3,8,9} (indices 0-2 + 3 + 8 + 9)

Le BFS retrouve naturellement le bon chemin pour chaque fermeture, car
il utilise l'etat du graphe **au moment de la fermeture**. Pour le
second cycle, le BFS trouve le chemin le plus court de 0 a 9 dans le
graphe complet (incluant les bonds du premier cycle et la bond de
fermeture `2`), ce qui donne le cycle de taille 6 correct.

Pour les systemes spiro (`C1CC1C2CC2`) :
- Chaque fermeture produit son propre cycle independant
- Le BFS ne melange pas les cycles car l'atome spiro est le seul point
  de contact

### Fichiers

| Action   | Fichier                                          |
|----------|--------------------------------------------------|
| Modifier | `crates/smiles-core/src/parser.rs`               |
| Modifier | `crates/smiles-core/src/ast/molecule.rs`         |

### Modifications au Parser

```rust
struct Parser<'a> {
    // ... champs existants ...
    detected_rings: Vec<Ring>,  // NOUVEAU : cycles detectes pendant le parsing
}

// Dans la section de fermeture de cycle (ring closure) :
// Apres avoir cree la bond de fermeture, on lance le BFS pour trouver
// le chemin de target a source, et on stocke le cycle detecte.
```

### Structure Ring

```rust
/// Un cycle detecte lors du parsing, represente par la sequence
/// ordonnee des indices globaux de noeuds.
#[derive(Debug, Clone, PartialEq)]
pub struct Ring {
    pub nodes: Vec<u16>,
}
```

### Methode de commodite sur Molecule

```rust
impl Molecule {
    /// Retourne les cycles detectes pendant le parsing.
    /// Seuls les cycles aromatiques (tous les noeuds aromatic == true)
    /// sont inclus si `aromatic_only` est true.
    pub fn rings(&self) -> &[Ring];
    pub fn aromatic_rings(&self) -> Vec<&Ring>;
}
```

### Tests

| Test                      | Entree SMILES         | Cycles attendus                    |
|---------------------------|-----------------------|------------------------------------|
| benzene                   | `c1ccccc1`            | 1 cycle de taille 6               |
| naphthalene               | `c1ccc2ccccc2c1`      | 2 cycles de taille 6              |
| biphenyl                  | `c1ccccc1c1ccccc1`    | 2 cycles separes de taille 6      |
| cyclohexane               | `C1CCCCC1`            | 1 cycle de taille 6 (non aromat.) |
| indole                    | `c1ccc2[nH]ccc2c1`    | 2 cycles (5 + 6)                  |
| toluene                   | `Cc1ccccc1`           | 1 cycle de taille 6               |
| spiro                     | `C1CCC1C2CCC2`        | 2 cycles independants (4 + 4)     |
| cubane                    | `C12C3C4C1C5C3C4C25`  | plusieurs cycles de taille 4      |
| cycle dans branche        | `C(C1CCC1)CC`         | 1 cycle de taille 4               |
| cycle cross-branche       | `C1CC(CC1)CC`         | 1 cycle de taille 5               |

---

## Phase 3 : Comptage des electrons pi et validation de Huckel

### Objectif

Pour chaque cycle aromatique detecte (Phase 2), calculer la contribution
en electrons pi de chaque atome en utilisant la configuration electronique
(Phase 1), puis verifier que le total satisfait 4n+2.

### Fichiers

| Action   | Fichier                                             |
|----------|-----------------------------------------------------|
| Creer    | `crates/smiles-core/src/ast/aromaticity.rs`         |
| Creer    | `crates/smiles-core/tests/huckel.rs`                |
| Modifier | `crates/smiles-core/src/ast/mod.rs`                 |
| Modifier | `crates/smiles-core/src/error/molecule.rs`          |

### Determination de la contribution pi a partir de la configuration electronique

Pour un atome dans un cycle aromatique, on utilise la configuration
electronique pour determiner combien d'electrons il peut fournir au
systeme pi conjugue.

#### Algorithme

```
pi_contribution(node, molecule, ring) -> Option<u8>:
    element = node.atom().element()
    if element == Wildcard:
        return None  // indetermine

    Z = element.atomic_number()
    config = ElectronConfiguration::from_atomic_number(Z)
    charge = node.atom().charge()

    // Electrons disponibles sur la couche la plus eloignee, ajustes par la charge.
    // charge > 0 => electrons retires, charge < 0 => electrons ajoutes
    valence_e = config.outermost_electrons() as i8 - charge

    // Nombre de liaisons sigma formees par cet atome
    // = liaisons dans le cycle (toujours 2) + H explicites + substituants hors cycle
    sigma_bonds = count_sigma_bonds(node, molecule)

    // Paires libres non-liantes = electrons restants apres liaisons sigma
    // Chaque liaison sigma consomme 1 electron de l'atome
    remaining = valence_e - sigma_bonds as i8

    // En hybridation sp2 (systeme aromatique plan) :
    // - Les liaisons sigma et une paire libre (si dispo) occupent les 3 orbitales sp2
    // - L'orbitale p perpendiculaire au plan contient la contribution pi
    //
    // Si remaining >= 2 et que l'atome n'a que 2 bonds dans le cycle (pas de double bond) :
    //   => 2 electrons dans l'orbitale p (paire libre donneuse, type pyrrole/furane)
    // Si remaining == 1 :
    //   => 1 electron dans l'orbitale p (type benzene/pyridine)
    // Si remaining <= 0 :
    //   => 0 electrons (orbitale p vide, type borane)

    if remaining <= 0: return Some(0)
    if remaining == 1: return Some(1)

    // remaining >= 2 : il faut distinguer le cas ou l'atome a une
    // double liaison dans le cycle (contribue 1) vs paire libre (contribue 2)
    ring_bond_types = get_ring_bond_types(node_index, ring, molecule)
    has_double_in_ring = ring_bond_types.contains(BondType::Double)

    if has_double_in_ring:
        return Some(1)  // electron engage dans la double liaison
    else:
        return Some(2)  // paire libre disponible pour le pi
```

#### Verification

Ce raisonnement produit les memes resultats que la table empirique de la v1,
mais il est derive de la physique :

| Atome         | Config ext.   | charge | valence_e | sigma | remaining | pi |
|---------------|---------------|--------|-----------|-------|-----------|----|
| C (benzene)   | 2s2 2p2       | 0      | 4         | 3     | 1         | 1  |
| C- (Cp anion) | 2s2 2p2       | -1     | 5         | 3     | 2         | 2  |
| C+ (tropylium)| 2s2 2p2       | +1     | 3         | 3     | 0         | 0  |
| N (pyridine)  | 2s2 2p3       | 0      | 5         | 3     | 2         | 1* |
| N (pyrrole)   | 2s2 2p3       | 0      | 5         | 3     | 2         | 2  |
| O (furane)    | 2s2 2p4       | 0      | 6         | 2     | 4         | 2  |
| S (thiophene) | 3s2 3p4       | 0      | 6         | 2     | 4         | 2  |
| B (borazine)  | 2s2 2p1       | 0      | 3         | 3     | 0         | 0  |
| Se            | 4s2 4p4       | 0      | 6         | 2     | 4         | 2  |

(*) N pyridine : remaining=2 mais a une double liaison dans le cycle => 1 pi.
N pyrrole : remaining=2, pas de double liaison, [nH] => 2 pi (paire libre).

Note : dans le SMILES aromatique (atomes en minuscules), les liaisons
explicites double/simple ne sont pas specifiees — toutes sont
`BondType::Aromatic`. La distinction pyridine/pyrrole se fait donc
par le contexte : si N a un H (`[nH]`) ou seulement 2 bonds dans le
cycle sans H, c'est un donneur de paire (2 pi). Si N a 3 bonds (y compris
un substituant) et pas de H, c'est type pyridine (1 pi). On raffine
l'algorithme :

```
// Cas special pour les atomes avec remaining >= 2 et BondType::Aromatic
// (pas de double/simple explicite dans le cycle)
if all_ring_bonds_are_aromatic:
    total_bonds_on_atom = count_all_bonds(node, molecule)
    ring_bonds = 2  // toujours 2 dans un cycle simple

    // Type pyrrole : l'atome n'a que 2 bonds dans le cycle + optionnel H
    // => paire libre disponible
    if total_bonds_on_atom == ring_bonds + node.hydrogens():
        if remaining >= 2: return Some(2)
        else: return Some(1)

    // Type pyridine : l'atome a 3+ bonds (substituant, ou participe
    // a la conjugaison via double bond implicite)
    return Some(1)
```

### Verification de Huckel

```rust
fn satisfies_huckel(pi_electrons: u8) -> bool {
    pi_electrons >= 2 && (pi_electrons - 2) % 4 == 0
}
// Valeurs valides : 2, 6, 10, 14, 18, 22...
```

### Structures de resultat

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct AromaticityCheck {
    pub ring: Ring,
    pub pi_electrons: Option<u8>,  // None si indetermine (Wildcard present)
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

| Test                        | SMILES              | e- pi | Valide |
|-----------------------------|----------------------|-------|--------|
| benzene                     | `c1ccccc1`           | 6     | oui    |
| pyridine                    | `c1ccncc1`           | 6     | oui    |
| pyrrole                     | `c1cc[nH]c1`         | 6     | oui    |
| furane                      | `c1ccoc1`            | 6     | oui    |
| thiophene                   | `c1ccsc1`            | 6     | oui    |
| imidazole                   | `c1cnc[nH]1`         | 6     | oui    |
| cyclopentadienyle anion     | `[c-]1cccc1`         | 6     | oui    |
| tropylium cation            | `[cH+]1cccccc1`      | 6     | oui    |
| naphthalene (chaque cycle)  | `c1ccc2ccccc2c1`     | 6     | oui    |
| borazine                    | `b1nbnbn1`           | 6     | oui    |

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
Phase 1: electron_config.rs ──────┐
     (independant)                 ├──> Phase 3: aromaticity.rs ──> Phase 4: Integration
Phase 2: cycles au parsing ───────┘
     (independant)
```

Les phases 1 et 2 sont independantes et peuvent etre developpees en parallele.

---

## Cas limites

1. **Wildcard dans un cycle** : skip la validation (indetermine)
2. **Atomes charges** : ajustement des electrons de couche ext. par la charge
3. **Systemes fusionnes** : chaque fermeture de cycle produit un cycle,
   le BFS au moment de la fermeture recupere le bon chemin
4. **Doubles liaisons exocycliques** : comptees dans sigma_bonds, reduisent
   les electrons restants pour le pi
5. **Cycles partiellement aromatiques** : pas de validation Huckel
   (le cycle n'est pas dans `aromatic_rings()`)
6. **Borazine** : syntaxiquement valide (6 e- pi), meme si pas
   physiquement aromatique — correct pour un validateur syntaxique
7. **Fragments disconnectes** : chaque composante traitee independamment
8. **Cycles dans les branches** : le BFS fonctionne car les bonds de
   branche sont deja integrees au graphe au moment de la fermeture
9. **Cycles cross-branche** : les deferred_ring_bonds sont resolues
   apres la branche, le cycle est detecte a ce moment

---

## Resume des fichiers

### Nouveaux fichiers (3)
- `crates/smiles-core/src/ast/electron_config.rs`
- `crates/smiles-core/src/ast/aromaticity.rs`
- `crates/smiles-core/tests/huckel.rs`

### Fichiers modifies (5)
- `crates/smiles-core/src/ast/atom.rs`
- `crates/smiles-core/src/ast/mod.rs`
- `crates/smiles-core/src/ast/molecule.rs`
- `crates/smiles-core/src/error/molecule.rs`
- `crates/smiles-core/src/parser.rs`
- `crates/smiles-core/Cargo.toml`
