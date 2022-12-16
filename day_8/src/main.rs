use std::fmt::Display;

use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Tree {
    height: usize,
    visible: bool,
    score: usize,
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{};", self.height))?;
        if self.visible {
            f.write_str("t]")?;
        } else {
            f.write_str("f]")?;
        }

        Ok(())
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tree>> {
    let mut res = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        res.push(
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|tree_num| Tree {
                    height: tree_num.parse().unwrap(),
                    visible: false,
                    score: 0,
                })
                .collect(),
        );
    }

    res
}

fn mark_horizontally(trees: &mut [Vec<Tree>]) {
    let forest_height = trees.len();
    let forest_width = trees[0].len();

    let mut max_height: usize = 0;

    for (y, trees) in trees.iter_mut().enumerate() {
        let is_edge = y == 0 || y == forest_height - 1;

        for (x, tree) in trees.iter_mut().enumerate() {
            let is_edge = is_edge || x == 0 || x == forest_width - 1;
            if is_edge || tree.height > max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }

        max_height = 0;
        for tree in trees.iter_mut().rev() {
            if tree.height > max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }
    }
}

fn mark_vertically(trees: &mut [Vec<Tree>]) {
    let forest_height = trees.len();
    let forest_width = trees[0].len();

    let mut max_height: usize = 0;

    for x in 0..forest_width {
        let is_edge = x == 0 || x == forest_width - 1;

        #[allow(clippy::needless_range_loop)]
        // from top
        for y in 0..forest_height {
            let tree = &mut trees[y][x];
            let is_edge = is_edge || y == 0 || y == forest_height - 1;

            if is_edge || tree.height > max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }

        max_height = 0;
        #[allow(clippy::needless_range_loop)]
        // from bottom
        for y in (0..forest_height).rev() {
            let tree = &mut trees[y][x];
            let is_edge = is_edge || y == 0 || y == forest_height - 1;

            if is_edge || tree.height > max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }
    }
}

fn mark_visibility(trees: &mut [Vec<Tree>]) {
    mark_horizontally(trees);
    mark_vertically(trees);
}

#[allow(clippy::needless_range_loop)]
fn mark_tree_score((x_pos, y_pos): (usize, usize), trees: &mut [Vec<Tree>]) {
    let forest_width = trees[0].len();
    let forest_height = trees.len();

    let height = trees[y_pos][x_pos].height;
    let mut seen: Vec<usize> = Vec::new();
    // to the left
    let mut saw = 0;
    for x in (0..x_pos).rev() {
        saw += 1;

        if trees[y_pos][x].height >= height {
            break;
        }
    }
    seen.push(saw);
    saw = 0;

    // to the right
    for x in (x_pos + 1)..forest_width {
        saw += 1;

        if trees[y_pos][x].height >= height {
            break;
        }
    }
    seen.push(saw);
    saw = 0;

    for y in (0..y_pos).rev() {
        saw += 1;

        if trees[y][x_pos].height >= height {
            break;
        }
    }
    seen.push(saw);
    saw = 0;

    for y in (y_pos + 1)..forest_height {
        saw += 1;

        if trees[y][x_pos].height >= height {
            break;
        }
    }
    seen.push(saw);

    trees[x_pos][y_pos].score = seen.iter().product();
}

fn mark_scores(trees: &mut [Vec<Tree>]) {
    let forest_height = trees.len();
    let forest_width = trees[0].len();

    for y in 0..forest_height {
        for x in 0..forest_width {
            mark_tree_score((x, y), trees);
        }
    }
}

fn main() -> Result<()> {
    let input = utils::read_file()?;

    let mut trees = parse_input(&input);

    mark_visibility(&mut trees);

    let res = trees.iter().flatten().filter(|tree| tree.visible).count();

    println!("total visible trees = {res}");

    mark_scores(&mut trees);

    let score = trees.iter().flatten().map(|tree| tree.score).max().unwrap();

    println!("highest scenic score = {score}");

    Ok(())
}
