use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

#[derive(Clone, Debug)]
enum Action {
    Accept,
    Reject,
    Send(String),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Action::Accept,
            "R" => Action::Reject,
            _ => Action::Send(s.to_string()),
        })
    }
}

#[derive(Debug)]
struct Rule {
    xmas: char,
    comparison: char,
    against: isize,
    action: Action,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (m, action_str) = s.split_once(':').unwrap();
        let xmas = m.chars().nth(0).unwrap();
        let comparison = m.chars().nth(1).unwrap();
        let against = m[2..].parse().unwrap();
        let action = action_str.parse::<Action>().unwrap();
        Ok(Self {
            xmas,
            comparison,
            against,
            action,
        })
    }
}

impl Rule {
    fn matches(&self, part: &Part) -> bool {
        let part_value = part.0.get(&self.xmas).unwrap();
        match self.comparison {
            '<' => *part_value < self.against,
            '>' => *part_value > self.against,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    default: Action,
}

impl Workflow {
    fn decide(&self, part: &Part) -> Action {
        for r in &self.rules {
            if r.matches(part) {
                return r.action.clone();
            }
        }
        self.default.clone()
    }
}

#[derive(Debug)]
struct Workflows(HashMap<String, Workflow>);

fn parse_workflow_line(line: &str) -> (String, Workflow) {
    let (name, rest) = line.split_once('{').unwrap();
    let mut items: Vec<_> = rest.trim_end_matches('}').split(',').collect();
    let default = items.pop().unwrap().parse().unwrap();
    let rules = items.iter().map(|i| i.parse().unwrap()).collect();

    (name.to_string(), Workflow { rules, default })
}

impl FromStr for Workflows {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(parse_workflow_line).collect()))
    }
}

impl Workflows {
    fn should_accept(&self, part: &Part) -> bool {
        let mut workflow = "in".to_string();
        loop {
            match self.0.get(&workflow).unwrap().decide(part) {
                Action::Accept => return true,
                Action::Reject => return false,
                Action::Send(new_workflow) => {
                    workflow = new_workflow;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Part(HashMap<char, isize>);

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(HashMap::from_iter(
            s.trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .map(|rating| {
                    let xmas = rating.chars().next().unwrap();
                    let value = rating[2..].parse().unwrap();
                    (xmas, value)
                }),
        )))
    }
}

impl Part {
    fn sum(&self) -> isize {
        self.0.values().sum()
    }
}

fn main() {
    let input = adv2023::read_input();
    let (workflows_str, parts_str) = input.split("\n\n").collect_tuple().unwrap();
    let workflows: Workflows = workflows_str.parse().unwrap();
    let parts: Vec<Part> = parts_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    dbg!(&workflows);
    dbg!(&parts);
    let part1: isize = parts
        .iter()
        .map(|part| {
            if workflows.should_accept(part) {
                part.sum()
            } else {
                0
            }
        })
        .sum();
    dbg!(&part1);
}
