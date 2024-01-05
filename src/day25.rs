use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Node((char, char, char));

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Node({}{}{})",
            self.0 .0, self.0 .1, self.0 .2
        ))
    }
}

impl Node {
    fn from(s: &str) -> Self {
        assert_eq!(s.len(), 3);
        Self(s.chars().collect_tuple().unwrap())
    }
}

#[derive(Debug, Default, Clone)]
struct Graph {
    edges: HashMap<Node, HashSet<Node>>,
}

impl Graph {
    fn add_edge(&mut self, lhs: Node, rhs: Node) {
        self.edges.entry(lhs).or_default().insert(rhs);
        self.edges.entry(rhs).or_default().insert(lhs);
    }

    // https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm

    fn find_path(&self, from: Node, to: Node) -> Option<Vec<Node>> {
        let mut queue = VecDeque::<Node>::from([from]);
        let mut pred = HashMap::<Node, Node>::new();
        'outer: while let Some(cur) = queue.pop_front() {
            for e in &self.edges[&cur] {
                if !pred.contains_key(&e) && *e != from {
                    pred.insert(*e, cur);
                    if *e == to {
                        break 'outer;
                    }
                    queue.push_back(*e);
                }
            }
        }
        if !pred.contains_key(&to) {
            None
        } else {
            let mut out = vec![];
            let mut cur = to;
            while pred.contains_key(&cur) {
                out.push(cur);
                cur = pred[&cur];
            }
            out.reverse();
            Some(out)
        }
    }

    fn find_reachable(&self, from: Node) -> HashSet<Node> {
        let mut queue = VecDeque::<Node>::from([from]);
        let mut reachable = HashSet::<Node>::from([from]);
        while let Some(cur) = queue.pop_front() {
            for e in &self.edges[&cur] {
                if !reachable.contains(e) {
                    reachable.insert(*e);
                    queue.push_back(*e);
                }
            }
        }
        reachable
    }

    fn take_edge(&mut self, from: Node, to: Node) {
        if self.edges[&to].contains(&from) {
            // println!("removing {:?}->{:?}", from, to);
            self.edges.get_mut(&from).unwrap().remove(&to);
        } else {
            // println!("adding {:?}->{:?}", to, from);
            self.edges.get_mut(&to).unwrap().insert(from);
        }
    }

    fn find_all_non_overlapping_paths(&mut self, from: Node, to: Node) -> usize {
        let mut paths = 0;
        loop {
            match self.find_path(from, to) {
                Some(path) => {
                    // dbg!(&path);
                    let mut cur = from;
                    for next in path {
                        self.take_edge(cur, next);
                        cur = next;
                    }
                }
                None => break,
            }
            paths += 1;
        }
        // dbg!(paths);
        paths
    }
}

impl FromStr for Graph {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Graph::default();
        // pzl: lsr hfx nvd
        s.lines().for_each(|line| {
            let (lhs, rhs) = line.split_once(": ").unwrap();
            let a = Node::from(lhs);
            rhs.split_whitespace()
                .for_each(|b| graph.add_edge(a, Node::from(b)));
        });
        Ok(graph)
    }
}

fn main() {
    let input = adv2023::read_input();
    let graph: Graph = input.parse().unwrap();
    dbg!(&graph);

    // Pick an arbitrary starting node.
    let from = *graph.edges.keys().next().unwrap();

    let to_candidates: Vec<Node> = graph.edges.keys().cloned().collect();

    for to in to_candidates {
        if to == from {
            continue;
        }
        let mut tmp_graph = graph.clone();
        let path_count = tmp_graph.find_all_non_overlapping_paths(from, to);
        if path_count > 3 {
            continue;
        }
        assert_eq!(path_count, 3);

        // Now, any nodes that we can still reach, are in one partition.
        let reachable = tmp_graph.find_reachable(from);
        let group1 = reachable.len();
        let group2 = tmp_graph.edges.len() - group1;
        dbg!(group1, group2, group1 * group2);

        break;
    }
}
