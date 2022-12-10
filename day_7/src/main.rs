// this can be solved as a depth first search, see ThePrimeagen's solution for that
//
// However, as an excersize, it would be interesting to try and solve this by first parsing the
// filesystem and generating a tree

use std::{collections::BTreeMap, str::FromStr};

use petgraph::{
    stable_graph::{NodeIndex, StableGraph},
    Direction,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct File {
    name: String,
    size: Size,
}

impl FromStr for File {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.split(' ');

        let err = format!("Could not parse {s}");
        let size = input.next().ok_or(&err)?.parse().map_err(|_| err.clone())?;
        let name = input.next().ok_or(&err)?.parse().map_err(|_| err)?;

        Ok(Self { name, size })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Size {
    #[default]
    Unknown,
    Known(usize),
}
impl Size {
    fn into_inner(self) -> usize {
        match self {
            Size::Unknown => 0,
            Size::Known(size) => size,
        }
    }
}

impl FromStr for Size {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size: usize = s
            .parse()
            .map_err(|_| format!("Could not parse size: {s}"))?;

        Ok(Self::Known(size))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Dir {
    name: String,
    size: Size,
    nodes: BTreeMap<String, Node>,
}

impl FromStr for Dir {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.split(' ');

        let err = format!("Could not parse {s}");
        let name = input.nth(1).ok_or(&err)?.parse().map_err(|_| err)?;

        Ok(Self {
            name,
            ..Default::default()
        })
    }
}

impl Dir {
    fn calculate_size(&mut self) {
        let mut dir_size = 0;

        for node in self.nodes.values_mut() {
            let size = match node {
                Node::Dir(dir) => {
                    if let Size::Unknown = dir.size {
                        dir.calculate_size();
                    }

                    dir.size
                }
                Node::File(file) => file.size,
            };

            if let Size::Known(size) = size {
                dir_size += size;
            }
        }

        self.size = Size::Known(dir_size);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Node {
    Dir(Dir),
    File(File),
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("dir") {
            Dir::from_str(s).map(|dir| dir.into())
        } else {
            File::from_str(s).map(|file| file.into())
        }
    }
}

impl Node {
    fn name(&self) -> &str {
        match self {
            Node::Dir(dir) => &dir.name,
            Node::File(file) => &file.name,
        }
    }

    fn as_dir_mut(&mut self) -> &mut Dir {
        match self {
            Node::Dir(dir) => dir,
            _ => panic!("Can't return File as Dir!"),
        }
    }

    fn calculate_size(&mut self) {
        if let Self::Dir(dir) = self {
            // size for files is always known
            dir.calculate_size();
        }
    }

    fn dir(&self) -> Option<&Dir> {
        match self {
            Node::Dir(dir) => Some(dir),
            Node::File(_) => None,
        }
    }

    fn size(&self) -> usize {
        let size = match self {
            Node::Dir(dir) => dir.size,
            Node::File(file) => file.size,
        };

        match size {
            Size::Unknown => 0,
            Size::Known(size) => size,
        }
    }
}

impl From<Dir> for Node {
    fn from(dir: Dir) -> Self {
        Self::Dir(dir)
    }
}

impl From<File> for Node {
    fn from(file: File) -> Self {
        Self::File(file)
    }
}

#[derive(Debug, Clone)]
struct Filesystem {
    dirs: StableGraph<Node, ()>,
}

impl Filesystem {
    fn from_input(input: &str) -> Self {
        let mut tree: StableGraph<Node, ()> = StableGraph::new();
        let mut curr = None;

        for line in input.lines() {
            if line.starts_with("$ cd") {
                if line == "$ cd .." {
                    let curr_node = tree.node_weight(curr.unwrap()).cloned();

                    curr = tree
                        .neighbors_directed(curr.unwrap(), Direction::Incoming)
                        .next();

                    if let Some(curr) = curr {
                        tree.node_weight_mut(curr)
                            .unwrap()
                            .as_dir_mut()
                            .nodes
                            .insert(
                                curr_node.as_ref().unwrap().name().into(),
                                curr_node.unwrap(),
                            );
                    }
                    continue;
                }

                // if we change to dir, add that dir as node to the graph
                let name = line.split(' ').last().unwrap().into();
                let dir = Dir {
                    name,
                    size: Size::Unknown,
                    nodes: Default::default(),
                };

                let new = tree.add_node(dir.clone().into());

                if let Some(curr) = curr {
                    tree.add_edge(curr, new, ());
                }

                curr = Some(new);
            } else if !line.starts_with("$ ls") {
                let new_node: Node = line.parse().unwrap();
                if let Node::Dir(_) = new_node {
                    // we only care about directories we visit -> cd dir_name
                    continue;
                }

                let new = if let Some(node_idx) =
                    tree.node_indices().find(|idx| tree[*idx] == new_node)
                {
                    node_idx
                } else {
                    tree.add_node(new_node.clone())
                };

                if let Some(node) = curr {
                    let parent = tree.node_weight_mut(node).unwrap();

                    if let Node::Dir(ref mut parent) = parent {
                        parent.nodes.insert(new_node.name().into(), new_node);
                        tree.add_edge(node, new, ());
                    }
                }
            }
        }

        // might be that we never got back from the last directory...
        while let Some(idx) = curr {
            match tree.node_weight(idx) {
                Some(node) if node.name() == "/" => break,
                _ => {
                    let curr_node = tree.node_weight(curr.unwrap()).cloned();

                    curr = tree
                        .neighbors_directed(curr.unwrap(), Direction::Incoming)
                        .next();

                    if let Some(curr) = curr {
                        tree.node_weight_mut(curr)
                            .unwrap()
                            .as_dir_mut()
                            .nodes
                            .insert(
                                curr_node.as_ref().unwrap().name().into(),
                                curr_node.unwrap(),
                            );
                    }
                }
            }
        }

        Self { dirs: tree }
    }

    fn calculate_sizes(&mut self) {
        for node in self.dirs.node_weights_mut() {
            node.calculate_size();
        }
    }

    fn iter(&self) -> Iter {
        Iter {
            filesystem: self,
            index: 0,
            indices: Vec::default(),
        }
    }
}

struct Iter<'a> {
    filesystem: &'a Filesystem,
    index: usize,
    indices: Vec<NodeIndex>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.indices.is_empty() {
            self.indices = self.filesystem.dirs.node_indices().collect();
        }

        let res = self
            .filesystem
            .dirs
            .node_weight(*self.indices.get(self.index)?);

        self.index += 1;

        res
    }
}

// invariants:
// 1. Directed graph
// 2. Each node has exactly one parent, or none for the root
// 3. Each dir node can have 0 or many children
// 4. File nodes don't have any children

fn main() -> anyhow::Result<()> {
    let input = utils::read_file()?;

    let mut fs = Filesystem::from_input(&input);
    fs.calculate_sizes();

    let total_space = 70_000_000;
    let min_needed_space = 30_000_000;
    let used_space = fs.iter().map(|node| node.size()).max().unwrap();
    let free_space = total_space - used_space;
    let needed_space = min_needed_space - free_space;

    let res: usize = fs
        .iter()
        .filter_map(|node| node.dir())
        .map(|dir| dir.size.into_inner())
        .filter(|size| *size >= needed_space)
        .min()
        .unwrap();

    println!("Delete directory with size = {res}");

    Ok(())
}
