type Trees = Vec<Vec<usize>>;

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Trees {
    input
        .split("\n")
        .map(|row| {
            row.chars()
                .map(|char| (char.to_digit(10).expect("Char was not a digit") as usize))
                .collect()
        })
        .collect()
}

struct TreeGrid {
    trees: Trees,
    grid_size: usize,
    max_grid_index: usize,
}

impl TreeGrid {
    fn new(trees: Trees) -> Self {
        let grid_size = trees[0].len();
        let max_grid_index = grid_size - 1;
        Self {
            trees,
            grid_size,
            max_grid_index,
        }
    }

    fn height_at(&self, x: usize, y: usize) -> usize {
        if x >= self.grid_size || y >= self.grid_size {
            panic!(
                "x or y is greater than grid size {}\nx: {} y: {}",
                self.grid_size, x, y
            )
        }

        self.trees[y][x]
    }

    fn is_edge_tree(&self, x: usize, y: usize) -> bool {
        if x == 0 || x == self.max_grid_index {
            return true;
        }

        if y == 0 || y == self.max_grid_index {
            return true;
        }

        false
    }

    fn visible_from_left(&self, initial_x: usize, initial_y: usize) -> (bool, usize) {
        let mut visibility = 0;
        let mut visible = true;

        let is_edge_tree = self.is_edge_tree(initial_x, initial_y);
        if !is_edge_tree {
            let height = self.height_at(initial_x, initial_y);

            let mut x = initial_x - 1;

            loop {
                visibility += 1;
                let left_height = self.height_at(x, initial_y);
                if left_height >= height {
                    visible = false;
                    break;
                }
                if x == 0 {
                    break;
                }
                x -= 1;
            }
        }
        (visible, visibility)
    }

    fn visible_from_right(&self, initial_x: usize, initial_y: usize) -> (bool, usize) {
        let mut visible = true;
        let mut visibility = 0;

        let is_edge_tree = self.is_edge_tree(initial_x, initial_y);
        if !is_edge_tree {
            let height = self.height_at(initial_x, initial_y);

            let mut x = initial_x + 1;

            while x < self.grid_size {
                visibility += 1;
                let right_height = self.height_at(x, initial_y);
                if right_height >= height {
                    visible = false;
                    break;
                }
                x += 1;
            }
        }
        (visible, visibility)
    }

    fn visible_from_top(&self, initial_x: usize, initial_y: usize) -> (bool, usize) {
        let mut visibility = 0;
        let mut visible = true;
        let is_edge_tree = self.is_edge_tree(initial_x, initial_y);
        if !is_edge_tree {
            let height = self.height_at(initial_x, initial_y);

            let mut y = initial_y - 1;

            loop {
                visibility += 1;
                let top_height = self.height_at(initial_x, y);
                if top_height >= height {
                    visible = false;
                    break;
                }
                if y == 0 {
                    break;
                }
                y -= 1;
            }
        }
        (visible, visibility)
    }

    fn visible_from_bottom(&self, initial_x: usize, initial_y: usize) -> (bool, usize) {
        let mut visibility = 0;
        let mut visible = true;
        let is_edge_tree = self.is_edge_tree(initial_x, initial_y);
        if !is_edge_tree {
            let height = self.height_at(initial_x, initial_y);

            let mut y = initial_y + 1;

            while y < self.grid_size {
                visibility += 1;
                let top_height = self.height_at(initial_x, y);
                if top_height >= height {
                    visible = false;
                    break;
                }
                y += 1;
            }
        }
        (visible, visibility)
    }

    fn tree_is_visible(&self, x: usize, y: usize) -> bool {
        let (left, _) = self.visible_from_left(x, y);
        let (right, _) = self.visible_from_right(x, y);
        let (top, _) = self.visible_from_top(x, y);
        let (bottom, _) = self.visible_from_bottom(x, y);
        let visible = left || right || top || bottom;
        visible
    }

    fn visibility_score(&self, x: usize, y: usize) -> usize {
        let height = self.height_at(x, y);
        let (_, left) = self.visible_from_left(x, y);
        let (_, right) = self.visible_from_right(x, y);
        let (_, top) = self.visible_from_top(x, y);
        let (_, bottom) = self.visible_from_bottom(x, y);
        println!(
            "x: {}, y: {}, height: {}\nleft: {}, right: {}, top: {}, bottom: {}",
            x, y, height, left, right, top, bottom
        );

        left * right * top * bottom
    }
}

#[aoc(day8, part1)]
fn part1(input: &Trees) -> usize {
    let tree_grid = TreeGrid::new(input.clone());

    let mut visible_count = 0;
    for x in 0..tree_grid.grid_size {
        for y in 0..tree_grid.grid_size {
            if tree_grid.tree_is_visible(x, y) {
                visible_count += 1;
            }
        }
    }
    visible_count
}

#[aoc(day8, part2)]
fn part2(input: &Trees) -> usize {
    let tree_grid = TreeGrid::new(input.clone());
    let mut max = 0;

    for x in 0..tree_grid.grid_size {
        for y in 0..tree_grid.grid_size {
            let score = tree_grid.visibility_score(x, y);
            println!("score: {}", score);
            if score > max {
                max = score;
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = input_generator(
            r"30373
25512
65332
33549
35390
",
        );
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn part2_test() {
        let input = input_generator(
            r"30373
25512
65332
33549
35390
",
        );
        assert_eq!(part2(&input), 8);
    }
}
