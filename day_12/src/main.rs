use std::{collections::BTreeMap, str::FromStr};

use anyhow::Result;
use petgraph::{
    algo,
    dot::{Config, Dot},
    graph::NodeIndex,
    Directed, Graph,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Height {
    Start(usize),
    End(usize),
    OnPath(usize),
}

impl Height {
    fn into_inner(self) -> usize {
        match self {
            Height::Start(val) | Height::End(val) | Height::OnPath(val) => val,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    height: Height,
    pos: Position,
}

impl Node {
    fn reaches(&self, other: &Self) -> bool {
        let this_height = self.height.into_inner();
        let other_height = other.height.into_inner();

        this_height > other_height || matches!(this_height.abs_diff(other_height), 0..=1)
    }
}

trait HeightFromStr {
    fn from_str(s: &str) -> Result<Self>
    where
        Self: Sized;
}

impl HeightFromStr for Height {
    fn from_str(s: &str) -> Result<Self> {
        let ch = char::from_str(s)?;

        let val = match ch {
            'S' => 'a',
            'E' => 'z',
            ch => ch,
        };

        let val = val as usize - 'a' as usize;

        let height = match ch {
            'S' => Height::Start(val),
            'E' => Height::End(val),
            _ => Height::OnPath(val),
        };

        Ok(height)
    }
}

fn reachable_positions(nodes: &[Vec<Node>], Position { x, y }: Position) -> Vec<Position> {
    let mut positions = Vec::with_capacity(4);
    let start_y = y.saturating_sub(1);
    let start_x = x.saturating_sub(1);

    for other_y in start_y..=y + 1 {
        for other_x in start_x..=x + 1 {
            if other_x == x && other_y == y || other_x != x && other_y != y {
                // current node OR diagonal
                continue;
            }

            if let Some(node) = nodes.get(other_y).and_then(|row| row.get(other_x)) {
                if nodes[y][x].reaches(node) {
                    positions.push(node.pos);
                }
            }
        }
    }

    positions
}

fn create_graph(nodes: Vec<Vec<Node>>) -> Graph<Node, usize, Directed> {
    let capacity = nodes.len() * nodes[0].len();
    let mut graph = Graph::with_capacity(capacity, capacity * 4); // each node has max 4 edges

    let mut pairs = BTreeMap::default();

    for row_nodes in nodes.iter() {
        for node in row_nodes.iter().cloned() {
            let pos = node.pos;
            let idx = graph.add_node(node);

            pairs.insert(idx, reachable_positions(&nodes, pos));
        }
    }

    for node_idx in graph.node_indices() {
        let reachable_indices: Vec<_> = graph
            .node_indices()
            .filter(|idx| {
                let reachable_positions = pairs.get(&node_idx).unwrap();
                graph
                    .node_weight(*idx)
                    .map(|node| reachable_positions.contains(&node.pos))
                    .unwrap_or(false)
            })
            .collect();

        for reachable in reachable_indices {
            graph.add_edge(node_idx, reachable, 1);
        }
    }

    graph
}

trait StartEnd {
    fn start(&self) -> Option<NodeIndex>;
    fn end(&self) -> Option<NodeIndex>;
}

impl StartEnd for Graph<Node, usize, Directed> {
    fn start(&self) -> Option<NodeIndex> {
        self.node_indices().find(|idx| {
            self.node_weight(*idx)
                .map(|node| matches!(node.height, Height::Start(_)))
                .unwrap_or(false)
        })
    }

    fn end(&self) -> Option<NodeIndex> {
        self.node_indices().find(|idx| {
            self.node_weight(*idx)
                .map(|node| matches!(node.height, Height::End(_)))
                .unwrap_or(false)
        })
    }
}

fn main() -> Result<()> {
    let input = utils::read_file()?;

    let nodes = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.split("")
                .filter(|s| !s.is_empty())
                .enumerate()
                .map(move |(x, s)| {
                    Ok::<_, anyhow::Error>(Node {
                        pos: Position { x, y },
                        height: HeightFromStr::from_str(s)?,
                    })
                })
                .collect()
        })
        .collect::<Result<Vec<Vec<Node>>, _>>()?;

    let graph = create_graph(nodes);

    // part 1 - from S to E
    if let (Some(start), Some(end)) = (graph.start(), graph.end()) {
        let map = petgraph::algo::dijkstra(&graph, start, None, |_| 1);

        if let Some(cost) = map.get(&end) {
            println!("Part 1 result = {cost}");
        }
    }

    // part 2 - from any a to E, choose shortest one
    let a_nodes = graph.node_indices().filter(|idx| {
        graph
            .node_weight(*idx)
            .map(|node| node.height.into_inner() == 0)
            .unwrap_or(false)
    });

    if let Some(end) = graph.end() {
        let mut results = Vec::default();
        for start_node in a_nodes {
            let res = algo::astar(&graph, start_node, |node| node == end, |_| 1, |_| 0);

            if let Some((cost, _)) = res {
                results.push(cost);
            }
        }

        let min = results.iter().min();
        if let Some(min) = min {
            println!("Part 2 result = {min}");
        }
    }

    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel]);

    println!("{dot:?}");

    Ok(())
}
