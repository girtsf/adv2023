use std::{
    cmp::max,
    collections::{BTreeSet, HashMap, HashSet},
};

use adv2023::Pos;

type NodeIdx = usize;

#[derive(Debug, Clone)]
struct Edge {
    length: usize,
    connects_to: NodeIdx,
}

#[derive(Clone, Debug, Default)]
struct Node {
    pos: Pos,
    outgoing: Vec<Edge>,
    incoming: HashSet<NodeIdx>,
    longest_distance: usize,
}

#[derive(Clone, Default, Debug)]
struct Graph {
    nodes: Vec<Node>,
    pos_to_node: HashMap<Pos, NodeIdx>,
}

impl Graph {
    fn insert_or_get_node(&mut self, pos: &Pos) -> usize {
        if let Some(idx) = self.pos_to_node.get(&pos) {
            return *idx;
        }
        let node = Node {
            pos: pos.clone(),
            ..Default::default()
        };
        let idx = self.nodes.len();
        self.nodes.push(node);
        self.pos_to_node.insert(pos.clone(), idx);
        idx
    }

    fn add_edge(&mut self, from: NodeIdx, connects_to: NodeIdx, length: usize) {
        self.nodes[from].outgoing.push(Edge {
            length,
            connects_to,
        });
        assert!(self.nodes[connects_to].incoming.insert(from));
    }

    fn dump_nodes(&self) {
        println!("digraph G {{");
        for (i, node) in self.nodes.iter().enumerate() {
            print!("  N{i} -> {{");
            for edge in &node.outgoing {
                print!("N{} ", edge.connects_to);
            }
            println!("}}");
        }
        println!("}}");
    }

    fn make_bidirectional(&mut self) {
        for (i, node) in self.nodes.clone().iter().enumerate() {
            for edge in &node.outgoing {
                self.add_edge(edge.connects_to, i, edge.length);
            }
        }
    }

    fn recurse(&self, node_idx: NodeIdx, mut visited_mask: u64, length: usize) -> Option<usize> {
        if node_idx == (self.nodes.len() - 1) {
            // assumes end is last idx
            return Some(length);
        }
        if ((1 << node_idx) & visited_mask) != 0 {
            // seen already
            return None;
        }
        let mut out: Option<usize> = None;
        visited_mask |= 1 << node_idx;
        for edge in &self.nodes[node_idx].outgoing {
            if let Some(result_length) =
                self.recurse(edge.connects_to, visited_mask, length + edge.length)
            {
                out = Some(max(out.unwrap_or_default(), result_length));
            }
        }
        out
    }

    fn brute_force_longest_path(&self) -> usize {
        // assumes going from 0 to last idx.
        self.recurse(0, 0, 0).unwrap()
    }
}

#[derive(Default, Debug, Clone)]
struct Problem {
    map: Vec<Vec<char>>,
    map_size: Pos,
    start: Pos,
    end: Pos,
    graph: Graph,
}

impl Problem {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();
        let map_size = Pos::new(map.len(), map[0].len());
        let start = Pos::new(0, 1);
        let end = map_size + Pos::new(-1, -2);
        Self {
            map,
            map_size,
            start,
            end,
            ..Default::default()
        }
    }

    fn get_tile(&self, pos: &Pos) -> char {
        self.map[pos.y as usize][pos.x as usize]
    }

    // Walks the edge, returns next node id and path length, unless dead-ends.
    fn walk_edge(&mut self, mut pos: Pos, mut dir: Pos) -> Option<(NodeIdx, usize)> {
        // println!("walk_edge, pos: {pos:?} dir: {dir:?}");
        let mut length = 1;
        loop {
            if let Some(node_idx) = self.graph.pos_to_node.get(&pos) {
                // println!("ended at node {node_idx}, length {length}");
                return Some((*node_idx, length));
            }

            let mut possible_dirs = match self.get_tile(&pos) {
                '.' => vec![Pos::left(), Pos::right(), Pos::up(), Pos::down()],
                '>' => vec![Pos::right()],
                '<' => vec![Pos::left()],
                'v' => vec![Pos::down()],
                '^' => vec![Pos::up()],
                _ => panic!(),
            };
            // dbg!(&possible_dirs);
            // Remove directions that would take us back to where we just came or go into walls.
            possible_dirs.retain(|new_dir| {
                new_dir != &dir.opposite() && self.get_tile(&(&pos + new_dir)) != '#'
            });
            // dbg!(&possible_dirs);
            if possible_dirs.is_empty() {
                // println!("ended at a dead-end");
                return None;
            }
            assert!(possible_dirs.len() == 1);
            dir = possible_dirs[0];
            pos += &dir;
            length += 1;
        }
    }

    fn find_nodes(&mut self) {
        self.graph.insert_or_get_node(&self.start);
        for y in 1..(self.map_size.y as usize - 1) {
            for x in 1..(self.map_size.x as usize - 1) {
                let pos = Pos::new(y, x);
                if self.get_tile(&pos) == '#' {
                    continue;
                }
                let walls = pos
                    .orthogonal_neighbors()
                    .iter()
                    .filter(|pos| self.get_tile(pos) == '#')
                    .count();
                if walls < 2 {
                    // println!("node at {pos:?}");
                    self.graph.insert_or_get_node(&pos);
                }
            }
        }
        self.graph.insert_or_get_node(&self.end);
    }

    fn find_edges(&mut self) {
        // For each node, consider each outgoing path.
        for node_idx in 0..self.graph.nodes.len() {
            let node_pos = self.graph.nodes[node_idx].pos;
            for dir in [Pos::up(), Pos::left(), Pos::down(), Pos::right()] {
                let n_pos = node_pos + dir;
                if !n_pos.check_bounds(&self.map_size) {
                    continue;
                }
                if self.get_tile(&n_pos) == '#' {
                    continue;
                }
                // println!("node_idx: {node_idx}, will look at edge starting at {n_pos:?}");
                if let Some((connects_to, length)) = self.walk_edge(n_pos, dir) {
                    self.graph.add_edge(node_idx, connects_to, length);
                }
            }
        }
    }

    fn build(&mut self) {
        self.find_nodes();
        self.find_edges();
    }

    fn find_longest_directional_path(&mut self) -> usize {
        let mut queue = BTreeSet::<NodeIdx>::from([0]);
        while let Some(node_idx) = queue.pop_first() {
            assert!(self.graph.nodes[node_idx].incoming.is_empty());
            let distance = self.graph.nodes[node_idx].longest_distance;
            let outgoing_edges = self.graph.nodes[node_idx].outgoing.clone();
            for Edge {
                length,
                connects_to,
            } in outgoing_edges
            {
                let other_node = &mut self.graph.nodes[connects_to];

                let new_distance = distance + length;
                if new_distance > other_node.longest_distance {
                    other_node.longest_distance = new_distance;
                }
                assert!(other_node.incoming.remove(&node_idx));
                if other_node.incoming.is_empty() {
                    queue.insert(connects_to);
                }
            }
        }
        self.graph.nodes.last().unwrap().longest_distance
    }
}

fn main() {
    let input = adv2023::read_input();
    let mut problem = Problem::new(&input);
    problem.build();
    problem.graph.dump_nodes();

    let mut graph2 = problem.graph.clone();
    let part1 = problem.find_longest_directional_path();
    dbg!(&part1);

    graph2.make_bidirectional();
    graph2.dump_nodes();
    let part2 = graph2.brute_force_longest_path();
    dbg!(&part2);
}
