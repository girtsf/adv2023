use std::{
    collections::{HashMap, VecDeque},
    process::Output,
    str::FromStr,
};

#[derive(Debug, Clone)]
enum NodeType {
    Broadcast,
    Output,
    // Current state.
    FlipFlop(bool),
    // Most recent pulse per incoming node.
    Conjunction(HashMap<String, bool>),
}

impl NodeType {
    fn from_char(c: char) -> Self {
        match c {
            '%' => Self::FlipFlop(false),
            '&' => Self::Conjunction(HashMap::new()),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    destinations: Vec<String>,
    node_type: NodeType,
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (type_and_name, dests) = s.split_once(" -> ").unwrap();
        let (name, node_type) = if type_and_name == "broadcaster" {
            (type_and_name.to_string(), NodeType::Broadcast)
        } else {
            (
                type_and_name[1..].to_string(),
                NodeType::from_char(type_and_name.chars().next().unwrap()),
            )
        };
        let destinations = dests.split(", ").map(|s| s.to_string()).collect();
        Ok(Node {
            name,
            destinations,
            node_type,
        })
    }
}

#[derive(Debug, Clone)]
struct Modules(HashMap<String, Node>);

impl FromStr for Modules {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: HashMap<String, Node> = s
            .lines()
            .map(|line| {
                let node: Node = line.parse().unwrap();
                (node.name.clone(), node)
            })
            .collect();
        nodes.insert(
            "output".to_string(),
            Node {
                name: "output".to_string(),
                destinations: vec![],
                node_type: NodeType::Output,
            },
        );
        let nodes_copy = nodes.clone();
        for source in nodes_copy.values() {
            for dest in &source.destinations {
                dbg!(dest);
                if nodes.contains_key(dest) {
                    if let NodeType::Conjunction(c) = &mut nodes.get_mut(dest).unwrap().node_type {
                        c.insert(source.name.clone(), false);
                    }
                }
            }
        }

        Ok(Self(nodes))
    }
}

#[derive(Debug, Default)]
struct Propagation {
    // (src, dest, high/low)
    queue: VecDeque<(String, String, bool)>,
    low_sent: usize,
    high_sent: usize,
}

impl Propagation {
    fn add_to_queue(&mut self, src: &str, dest: &str, pulse: bool) {
        self.queue
            .push_back((src.to_string(), dest.to_string(), pulse));
        if pulse {
            self.high_sent += 1;
        } else {
            self.low_sent += 1;
        }
    }

    fn add_to_queue_many(&mut self, src: &str, dests: &[String], pulse: bool) {
        for d in dests {
            self.add_to_queue(src, d, pulse);
        }
    }

    fn push_button(&mut self, modules: &mut Modules) {
        self.add_to_queue("button", "broadcaster", false);
        self.process(modules)
    }

    fn process(&mut self, modules: &mut Modules) {
        while let Some((src, dest, pulse)) = self.queue.pop_front() {
            if !modules.0.contains_key(&dest) {
                continue;
            }
            let node = &mut modules.0.get_mut(&dest).unwrap();
            match &mut node.node_type {
                NodeType::Broadcast => {
                    self.add_to_queue_many(&dest, &node.destinations, pulse);
                }
                NodeType::Output => {}
                NodeType::FlipFlop(state) => {
                    if !pulse {
                        *state = !*state;
                        self.add_to_queue_many(&dest, &node.destinations, *state);
                    }
                }
                NodeType::Conjunction(inputs) => {
                    *inputs.get_mut(&src).unwrap() = pulse;
                    if inputs.values().all(|t| *t) {
                        self.add_to_queue_many(&dest, &node.destinations, false);
                    } else {
                        self.add_to_queue_many(&dest, &node.destinations, true);
                    }
                }
            }
        }
    }
}

fn main() {
    let input = adv2023::read_input();
    let mut modules: Modules = input.parse().unwrap();
    dbg!(&modules);
    let mut prop = Propagation::default();
    for _ in 0..1000 {
        prop.push_button(&mut modules);
    }
    dbg!(&prop.high_sent);
    dbg!(&prop.low_sent);
    dbg!(prop.high_sent * prop.low_sent);
}
