use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::{
    ast::{
        atom::{AtomSymbol, OrganicAtom},
        bond::{Bond, BondType},
        chirality::Chirality,
        node::{Node, NodeBuilder},
    },
    AtomError, MoleculeError, NodeError,
};

type SpanningTreeResult = (Vec<Vec<(u16, BondType)>>, Vec<Vec<u8>>);

#[derive(Debug, Clone, PartialEq)]
pub struct Molecule {
    nodes: Vec<Node>,
    bonds: Vec<Bond>,
}

struct DfsState<'a> {
    neighbour_list: &'a [Vec<(u16, BondType)>],
    tree_children: &'a mut Vec<Vec<(u16, BondType)>>,
    ring_pair_ids: &'a [Vec<u8>],
    visited: &'a mut Vec<bool>,
    output: &'a mut Vec<String>,
    nodes_to_output_positions: &'a mut Vec<usize>,
    pair_to_rnum: &'a mut HashMap<u8, u8>,
    next_rnum: &'a mut u8,
    bridges: &'a [(u16, u16)],
    virtual_h: &'a [u8],
    effective_aromatic: &'a [bool],
    aromatic_bonds: &'a HashSet<(u16, u16)>,
}

struct BridgeDfsState<'a> {
    neighbour_list: &'a [Vec<(u16, BondType)>],
    visited: &'a mut Vec<bool>,
    disc: &'a mut Vec<u32>,
    low: &'a mut Vec<u32>,
    timer: &'a mut u32,
    bridges: &'a mut Vec<(u16, u16)>,
}

struct SpanningTreeState<'a> {
    visited: &'a mut Vec<bool>,
    on_stack: &'a mut Vec<bool>,
    ring_counter: &'a mut u8,
    neighbour_list: &'a [Vec<(u16, BondType)>],
    tree_children: &'a mut Vec<Vec<(u16, BondType)>>,
    ring_digits: &'a mut Vec<Vec<u8>>,
}

impl Molecule {
    pub fn new(nodes: Vec<Node>, bonds: Vec<Bond>) -> Molecule {
        Molecule { nodes, bonds }
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn bonds(&self) -> &[Bond] {
        &self.bonds
    }

    fn dfs(&self, current: u16, state: &mut DfsState) -> Result<(), AtomError> {
        let pos = state.output.len();
        state.nodes_to_output_positions[current as usize] = pos;

        // Compute effective bond order sum (Kekulé double bonds treated as aromatic when in overlay)
        let bond_order_sum = state.neighbour_list[current as usize]
            .iter()
            .map(|(v, k)| {
                let edge = (current.min(*v), current.max(*v));
                if state.aromatic_bonds.contains(&edge) {
                    BondType::Aromatic.bond_order_x2_for_implicit_h()
                } else {
                    k.bond_order_x2_for_implicit_h()
                }
            })
            .sum::<u8>()
            / 2;

        state.output.push(self.format_atom(
            current as usize,
            bond_order_sum,
            state.virtual_h[current as usize],
            state.effective_aromatic[current as usize],
        )?);

        for &pair_id in &state.ring_pair_ids[current as usize] {
            let rnum = *state.pair_to_rnum.entry(pair_id).or_insert_with(|| {
                let n = *state.next_rnum;
                *state.next_rnum += 1;
                n
            });
            if rnum >= 10 {
                state.output.push(format!("%{:02}", rnum));
            } else {
                state.output.push(rnum.to_string());
            }
        }

        state.visited[current as usize] = true;

        let mut children = state.tree_children[current as usize].clone();
        children.sort_by_key(|(n, _)| Self::subtree_size(*n, state.tree_children));
        if let Some((main, branches)) = children.split_last() {
            for branch in branches {
                state.output.push("(".to_string());
                let edge = (current.min(branch.0), current.max(branch.0));
                let effective_bond = if state.aromatic_bonds.contains(&edge) {
                    BondType::Aromatic
                } else {
                    branch.1
                };
                if let Some(s) = Self::bond_symbol(
                    effective_bond,
                    state.effective_aromatic[current as usize],
                    state.effective_aromatic[branch.0 as usize],
                    state.bridges.contains(&edge),
                ) {
                    state.output.push(s.to_string());
                }
                self.dfs(branch.0, state)?;
                state.output.push(")".to_string());
            }
            let edge = (current.min(main.0), current.max(main.0));
            let effective_bond = if state.aromatic_bonds.contains(&edge) {
                BondType::Aromatic
            } else {
                main.1
            };
            if let Some(s) = Self::bond_symbol(
                effective_bond,
                state.effective_aromatic[current as usize],
                state.effective_aromatic[main.0 as usize],
                state.bridges.contains(&edge),
            ) {
                state.output.push(s.to_string());
            }
            self.dfs(main.0, state)?;
        }

        Ok(())
    }

    fn subtree_size(start: u16, tree_children: &[Vec<(u16, BondType)>]) -> usize {
        1 + tree_children[start as usize]
            .iter()
            .map(|(child, _)| Self::subtree_size(*child, tree_children))
            .sum::<usize>()
    }

    fn removable_hydrogens(&self, neighbour_list: &[Vec<(u16, BondType)>]) -> Vec<bool> {
        let mut removable = vec![false; self.nodes.len()];
        for (i, node) in self.nodes.iter().enumerate() {
            if *node.atom().element() != AtomSymbol::H {
                continue;
            }
            if node.atom().charge() != 0
                || node.atom().isotope().is_some()
                || node.class().is_some()
                || node.chirality().is_some()
            {
                continue;
            }
            let neighbors = &neighbour_list[i];
            if neighbors.len() != 1 {
                continue;
            }
            let (neighbor_idx, _) = neighbors[0];
            if *self.nodes[neighbor_idx as usize].atom().element() == AtomSymbol::H {
                continue;
            }
            removable[i] = true;
        }
        removable
    }

    fn best_starting_atom(
        &self,
        neighbour_list: &[Vec<(u16, BondType)>],
        removable_h: &[bool],
    ) -> u16 {
        let terminals: Vec<u16> = (0..self.nodes.len() as u16)
            .filter(|&i| !removable_h[i as usize] && neighbour_list[i as usize].len() == 1)
            .collect();

        if terminals.is_empty() {
            // Pas de terminaux : préférer les atomes de degré minimal pour éviter de
            // commencer sur un atome de jonction (spiro, pont) qui accumulerait
            // plusieurs ring closures sur le même atome dans la sortie canonique.
            let min_degree = (0..self.nodes.len() as u16)
                .filter(|&i| !removable_h[i as usize])
                .map(|i| neighbour_list[i as usize].len())
                .min()
                .unwrap_or(0);
            let candidates: Vec<u16> = (0..self.nodes.len() as u16)
                .filter(|&i| {
                    !removable_h[i as usize] && neighbour_list[i as usize].len() == min_degree
                })
                .collect();
            for &c in &candidates {
                if *self.nodes[c as usize].atom().element() != AtomSymbol::Organic(OrganicAtom::C) {
                    return c;
                }
            }
            return candidates[0];
        }

        for &t in &terminals {
            if *self.nodes[t as usize].atom().element() != AtomSymbol::Organic(OrganicAtom::C) {
                return t;
            }
        }

        terminals[0]
    }

    fn find_bridges(n: usize, neighbour_list: &[Vec<(u16, BondType)>]) -> Vec<(u16, u16)> {
        let mut visited = vec![false; n];
        let mut disc = vec![0u32; n];
        let mut low = vec![0u32; n];
        let mut timer = 0u32;
        let mut bridges = Vec::new();

        let mut state = BridgeDfsState {
            neighbour_list,
            visited: &mut visited,
            disc: &mut disc,
            low: &mut low,
            timer: &mut timer,
            bridges: &mut bridges,
        };
        for start in 0..n as u16 {
            if !state.visited[start as usize] {
                Self::bridge_dfs(start, u16::MAX, &mut state);
            }
        }
        bridges
    }

    fn bridge_dfs(u: u16, parent: u16, state: &mut BridgeDfsState) {
        state.visited[u as usize] = true;
        state.disc[u as usize] = *state.timer;
        state.low[u as usize] = *state.timer;
        *state.timer += 1;

        for v_idx in 0..state.neighbour_list[u as usize].len() {
            let (v, _) = state.neighbour_list[u as usize][v_idx];
            if v == parent {
                continue;
            }
            if !state.visited[v as usize] {
                Self::bridge_dfs(v, u, state);
                state.low[u as usize] = state.low[u as usize].min(state.low[v as usize]);
                if state.low[v as usize] > state.disc[u as usize] {
                    state.bridges.push((u.min(v), u.max(v)));
                }
            } else {
                state.low[u as usize] = state.low[u as usize].min(state.disc[v as usize]);
            }
        }
    }

    fn build_spanning_tree_inner(
        &self,
        current: u16,
        parent: Option<u16>,
        state: &mut SpanningTreeState,
    ) {
        state.visited[current as usize] = true;
        state.on_stack[current as usize] = true;

        // Trier les voisins (hors parent) par priorité de liaison décroissante.
        // Les liaisons doubles/triples deviennent ainsi des arêtes de l'arbre couvrant
        // (chain bonds) plutôt que des back edges (ring closures), ce qui évite d'avoir
        // une double liaison sur un ring closure dans la sortie canonique.
        let mut sorted_neighbors: Vec<(u16, BondType)> = state.neighbour_list[current as usize]
            .iter()
            .copied()
            .filter(|&(v, _)| Some(v) != parent)
            .collect();
        sorted_neighbors.sort_by(|a, b| b.1.bond_order_priority().cmp(&a.1.bond_order_priority()));

        for (voisin, bond_type) in sorted_neighbors {
            if state.visited[voisin as usize] {
                // On ne traite l'arête de retour que si le voisin est encore
                // sur la pile DFS (ancêtre), pour éviter de compter l'arête
                // une deuxième fois depuis l'autre extrémité.
                if state.on_stack[voisin as usize] {
                    *state.ring_counter += 1;
                    state.ring_digits[current as usize].push(*state.ring_counter);
                    state.ring_digits[voisin as usize].push(*state.ring_counter);
                }
            } else {
                state.tree_children[current as usize].push((voisin, bond_type));
                self.build_spanning_tree_inner(voisin, Some(current), state);
            }
        }

        state.on_stack[current as usize] = false;
    }

    pub fn build_spanning_tree(
        &self,
        neighbour_list: &[Vec<(u16, BondType)>],
    ) -> SpanningTreeResult {
        self.build_spanning_tree_from(0, neighbour_list)
    }

    fn build_spanning_tree_from(
        &self,
        start: u16,
        neighbour_list: &[Vec<(u16, BondType)>],
    ) -> SpanningTreeResult {
        let mut tree_children: Vec<Vec<(u16, BondType)>> = vec![Vec::new(); self.nodes.len()];
        let mut ring_pair_ids: Vec<Vec<u8>> = vec![Vec::new(); self.nodes.len()];
        let mut visited = vec![false; self.nodes.len()];
        let mut on_stack = vec![false; self.nodes.len()];
        let mut ring_counter = 0;

        let mut state = SpanningTreeState {
            visited: &mut visited,
            on_stack: &mut on_stack,
            ring_counter: &mut ring_counter,
            neighbour_list,
            tree_children: &mut tree_children,
            ring_digits: &mut ring_pair_ids,
        };
        self.build_spanning_tree_inner(start, None, &mut state);

        (tree_children, ring_pair_ids)
    }

    fn bond_symbol(
        kind: BondType,
        source_aromatic: bool,
        target_aromatic: bool,
        is_bridge: bool,
    ) -> Option<&'static str> {
        match kind {
            BondType::Simple => {
                if source_aromatic && target_aromatic {
                    Some("-")
                } else {
                    None
                }
            }
            BondType::Aromatic => {
                if is_bridge {
                    Some("-")
                } else {
                    None
                }
            }
            BondType::Double => Some("="),
            BondType::Triple => Some("#"),
            BondType::Quadruple => Some("$"),
            BondType::Disconnected => Some("."),
            BondType::Down => Some("\\"),
            BondType::Up => Some("/"),
        }
    }

    fn format_atom(
        &self,
        node_idx: usize,
        bond_order_sum: u8,
        extra_h: u8,
        effective_aromatic: bool,
    ) -> Result<String, AtomError> {
        let node = &self.nodes[node_idx];
        let total_h = node.hydrogens() + extra_h;

        if node.atom().is_organic()
            && node.atom().charge() == 0
            && node.atom().isotope().is_none()
            && node.chirality().is_none()
            && node.class().is_none()
            && total_h
                == node
                    .atom()
                    .implicit_hydrogens(Some(bond_order_sum), effective_aromatic)?
        {
            if effective_aromatic {
                Ok(node.atom().element().to_string().to_ascii_lowercase())
            } else {
                Ok(node.atom().element().to_string())
            }
        } else {
            let mut output = String::new();
            output.push('[');
            if let Some(i) = node.atom().isotope() {
                output.push_str(&i.to_string());
            }
            let element_str = node.atom().element().to_string();
            if effective_aromatic && node.atom().element().can_be_aromatic() {
                output.push_str(&element_str.to_ascii_lowercase());
            } else {
                output.push_str(&element_str);
            }
            if let Some(c) = node.chirality() {
                output.push_str(&c.to_string());
            }

            match total_h {
                0 => {}
                1 => output.push('H'),
                n => {
                    output.push('H');
                    output.push_str(&n.to_string());
                }
            }

            match node.atom().charge() {
                0 => (),
                1 => output.push('+'),
                -1 => output.push('-'),
                n => {
                    if n < 0 {
                        output.push('-');
                    } else {
                        output.push('+');
                    }
                    output.push_str(&n.abs().to_string());
                }
            }

            if let Some(c) = node.class() {
                output.push(':');
                output.push_str(&c.to_string());
            }
            output.push(']');
            Ok(output)
        }
    }

    /// Trouve les cycles minimaux pour chaque arête du sous-graphe Kekulé
    /// (liaisons Simple et Double uniquement).
    fn find_kekule_rings(
        n: usize,
        neighbour_list: &[Vec<(u16, BondType)>],
    ) -> Vec<super::graph::Ring> {
        let mut adj: Vec<Vec<u16>> = vec![Vec::new(); n];
        let mut edges: HashSet<(u16, u16)> = HashSet::new();

        for u in 0..n as u16 {
            for &(v, bond) in &neighbour_list[u as usize] {
                if matches!(bond, BondType::Simple | BondType::Double) {
                    adj[u as usize].push(v);
                    edges.insert((u.min(v), u.max(v)));
                }
            }
        }

        super::graph::find_rings_in_subgraph(&adj, &edges, n)
    }

    /// Vérifie si un cycle (avec liaisons Kekulé) est aromatique selon Hückel.
    /// Retourne Some(pi_electrons) si aromatique, None sinon.
    fn kekule_pi_electrons(
        &self,
        cycle: &[u16],
        neighbour_list: &[Vec<(u16, BondType)>],
    ) -> Option<u8> {
        let n = cycle.len();
        if n < 3 {
            return None;
        }

        // Tous les atomes doivent pouvoir être aromatiques et ne pas l'être déjà
        for &node_idx in cycle {
            let node = &self.nodes[node_idx as usize];
            if !node.atom().element().can_be_aromatic() {
                return None;
            }
            if node.aromatic() {
                return None;
            }
        }

        let mut pi_electrons: i32 = 0;
        let mut has_double_bond = vec![false; n];

        for i in 0..n {
            let a = cycle[i];
            let b = cycle[(i + 1) % n];

            let bond_type = neighbour_list[a as usize]
                .iter()
                .find(|&&(v, _)| v == b)
                .map(|&(_, t)| t)?;

            match bond_type {
                BondType::Double => {
                    pi_electrons += 2;
                    has_double_bond[i] = true;
                    has_double_bond[(i + 1) % n] = true;
                }
                BondType::Simple => {}
                _ => return None,
            }
        }

        // Pour les atomes sans double liaison dans le cycle, vérifier la paire libre
        for i in 0..n {
            if !has_double_bond[i] {
                let node = &self.nodes[cycle[i] as usize];
                let element = *node.atom().element();
                let charge = node.atom().charge();
                let hydrogens = node.hydrogens();

                let contribution: i32 = match element {
                    AtomSymbol::Organic(OrganicAtom::C) => {
                        if charge < 0 {
                            2
                        } else {
                            return None;
                        }
                    }
                    AtomSymbol::Organic(OrganicAtom::N) => {
                        if hydrogens > 0 || charge < 0 {
                            2
                        } else {
                            return None;
                        }
                    }
                    AtomSymbol::Organic(OrganicAtom::O) => 2,
                    AtomSymbol::Organic(OrganicAtom::S) => 2,
                    AtomSymbol::Organic(OrganicAtom::P) => {
                        if hydrogens > 0 || charge < 0 {
                            2
                        } else {
                            return None;
                        }
                    }
                    AtomSymbol::Organic(OrganicAtom::B) => 0,
                    AtomSymbol::Se | AtomSymbol::As | AtomSymbol::Te => 2,
                    _ => return None,
                };

                pi_electrons += contribution;
            }
        }

        if pi_electrons < 0 {
            return None;
        }
        let pi = pi_electrons as u8;
        if super::aromaticity::satisfies_huckel(pi) {
            Some(pi)
        } else {
            None
        }
    }

    /// Calcule l'overlay d'aromaticité pour l'affichage.
    /// Détecte les cycles Kekulé aromatiques et les combine avec l'aromaticité existante.
    /// Retourne (effective_aromatic_par_atome, ensemble_des_liaisons_aromatiques).
    fn compute_kekule_aromatic_overlay(
        &self,
        neighbour_list: &[Vec<(u16, BondType)>],
    ) -> (Vec<bool>, HashSet<(u16, u16)>) {
        let n = self.nodes.len();

        // Initialiser depuis l'aromaticité existante des nœuds
        let mut effective_aromatic: Vec<bool> =
            self.nodes.iter().map(|nd| nd.aromatic()).collect();
        let mut aromatic_bonds: HashSet<(u16, u16)> = HashSet::new();

        for bond in &self.bonds {
            if bond.kind() == BondType::Aromatic {
                let edge = (
                    bond.source().min(bond.target()),
                    bond.source().max(bond.target()),
                );
                aromatic_bonds.insert(edge);
            }
        }

        // Détecter les cycles Kekulé aromatiques et les ajouter à l'overlay
        let cycles = Self::find_kekule_rings(n, neighbour_list);

        for ring in &cycles {
            if self.kekule_pi_electrons(&ring.nodes, neighbour_list).is_some() {
                let len = ring.nodes.len();
                for i in 0..len {
                    let a = ring.nodes[i];
                    let b = ring.nodes[(i + 1) % len];
                    effective_aromatic[a as usize] = true;
                    aromatic_bonds.insert((a.min(b), a.max(b)));
                }
            }
        }

        (effective_aromatic, aromatic_bonds)
    }
}

impl fmt::Display for Molecule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut neighbour_list: Vec<Vec<(u16, BondType)>> = vec![Vec::new(); self.nodes.len()];
        for bond in &self.bonds {
            neighbour_list[bond.source() as usize].push((bond.target(), bond.kind()));
            neighbour_list[bond.target() as usize].push((bond.source(), bond.kind()));
        }

        // Identifier les H explicites normaux et les absorber dans le hcount de l'atome lourd.
        let removable_h = self.removable_hydrogens(&neighbour_list);

        let mut virtual_h: Vec<u8> = vec![0; self.nodes.len()];
        for (i, &removable) in removable_h.iter().enumerate() {
            if removable {
                let (neighbor_idx, _) = neighbour_list[i][0];
                virtual_h[neighbor_idx as usize] += 1;
            }
        }

        let mut neighbour_list_heavy: Vec<Vec<(u16, BondType)>> =
            vec![Vec::new(); self.nodes.len()];
        for bond in &self.bonds {
            let s = bond.source() as usize;
            let t = bond.target() as usize;
            if !removable_h[s] && !removable_h[t] {
                neighbour_list_heavy[s].push((bond.target(), bond.kind()));
                neighbour_list_heavy[t].push((bond.source(), bond.kind()));
            }
        }

        // Calculer l'overlay d'aromaticité Kekulé avant la construction de l'arbre couvrant
        let (effective_aromatic, aromatic_bonds) =
            self.compute_kekule_aromatic_overlay(&neighbour_list_heavy);

        let start = self.best_starting_atom(&neighbour_list_heavy, &removable_h);
        let bridges = Self::find_bridges(self.nodes.len(), &neighbour_list_heavy);
        let (mut tree_children, ring_pair_ids) =
            self.build_spanning_tree_from(start, &neighbour_list_heavy);

        let mut output: Vec<String> = Vec::new();
        let mut visited: Vec<bool> = vec![false; self.nodes.len()];
        let mut nodes_to_output_positions: Vec<usize> = vec![0; self.nodes.len()];
        let mut pair_to_rnum: HashMap<u8, u8> = HashMap::new();
        let mut next_rnum: u8 = 1;

        {
            let mut state = DfsState {
                neighbour_list: &neighbour_list_heavy,
                tree_children: &mut tree_children,
                ring_pair_ids: &ring_pair_ids,
                visited: &mut visited,
                output: &mut output,
                nodes_to_output_positions: &mut nodes_to_output_positions,
                pair_to_rnum: &mut pair_to_rnum,
                next_rnum: &mut next_rnum,
                bridges: &bridges,
                virtual_h: &virtual_h,
                effective_aromatic: &effective_aromatic,
                aromatic_bonds: &aromatic_bonds,
            };
            self.dfs(start, &mut state).map_err(|_| fmt::Error)?;
        }

        let final_output = output.join("");
        write!(f, "{final_output}")
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MoleculeBuilder {
    nodes: Vec<NodeBuilder>,
    bonds: Vec<Bond>,
}

impl MoleculeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn nodes(&self) -> &[NodeBuilder] {
        &self.nodes
    }

    pub fn bonds(&self) -> &[Bond] {
        &self.bonds
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_atom(
        &mut self,
        element: AtomSymbol,
        charge: i8,
        isotope: Option<u16>,
        aromatic: Option<bool>,
        hydrogens: Option<u8>,
        class: Option<u16>,
        chirality: Option<Chirality>,
    ) -> Result<usize, NodeError> {
        self.nodes.push(NodeBuilder::new(
            element, charge, isotope, aromatic, hydrogens, class, chirality,
        )?);
        Ok(self.nodes.len() - 1)
    }

    pub fn add_branch(&mut self, m: MoleculeBuilder, bond_type: BondType, source: Option<u16>) {
        let node_count = self.nodes.len() as u16;
        if let Some(src) = source {
            self.add_bond(src, node_count, bond_type);
        }
        self.nodes.extend(m.nodes);
        for bond in m.bonds {
            // Translate branch-local indices to main molecule indices
            // bond.source() and bond.target() are relative to the branch (0, 1, 2...)
            // After extend, they start at node_count in the main molecule
            self.add_bond(
                node_count + bond.source(),
                node_count + bond.target(),
                bond.kind(),
            );
        }
    }

    pub fn add_bond(&mut self, source: u16, target: u16, kind: BondType) {
        self.bonds.push(Bond::new(kind, source, target));
    }

    // À la fin
    pub fn build(self) -> Result<Molecule, MoleculeError> {
        let mut nodes: Vec<Node> = Vec::new();
        let mut bond_orders_x2 = vec![0u8; self.nodes.len()];

        // Calculate bond order sum for implicit hydrogen calculation
        // According to OpenSMILES, aromatic bonds count as 1.0 (not 1.5) for this purpose
        for bond in &self.bonds {
            bond_orders_x2[bond.source() as usize] += bond.kind().bond_order_x2_for_implicit_h();
            bond_orders_x2[bond.target() as usize] += bond.kind().bond_order_x2_for_implicit_h();
        }

        for (index, node) in self.nodes.into_iter().enumerate() {
            nodes.push(node.build(Some(bond_orders_x2[index] / 2))?);
        }

        Ok(Molecule {
            nodes,
            bonds: self.bonds,
        })
    }
}
