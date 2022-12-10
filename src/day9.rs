use std::{collections::HashSet, convert::From};

#[derive(Debug)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Direction {
    fn distance(&self) -> usize {
        match self {
            Direction::Up(distance) => *distance,
            Direction::Down(distance) => *distance,
            Direction::Left(distance) => *distance,
            Direction::Right(distance) => *distance,
        }
    }

    fn unit(&self) -> Self {
        match self {
            Direction::Up(_) => Self::Up(1),
            Direction::Down(_) => Self::Down(1),
            Direction::Left(_) => Self::Left(1),
            Direction::Right(_) => Self::Right(1),
        }
    }
}

impl From<(&str, usize)> for Direction {
    fn from((direction, distance): (&str, usize)) -> Self {
        match direction {
            "U" => Self::Up(distance),
            "D" => Self::Down(distance),
            "L" => Self::Left(distance),
            "R" => Self::Right(distance),
            _ => panic!("Bad direction"),
        }
    }
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Direction> {
    input
        .split("\n")
        .map(|line| {
            let line_split: Vec<_> = line.split(" ").collect();
            let direction = line_split[0];
            let distance: usize = line_split[1].parse().expect("Bad parse");

            Direction::from((direction, distance))
        })
        .collect()
}

struct Grid {
    knot_positions: Vec<(isize, isize)>,
    num_knots: usize,
    t_visited: HashSet<(isize, isize)>,
}

impl Grid {
    fn new(num_knots: usize) -> Self {
        Self {
            knot_positions: vec![(0, 0); num_knots],
            num_knots,
            t_visited: HashSet::from([(0, 0)]),
        }
    }

    fn head_pos(&self) -> &(isize, isize) {
        &self.knot_positions[0]
    }

    fn knot_pos(&self, knot_index: usize) -> &(isize, isize) {
        &self.knot_positions[knot_index]
    }

    fn update_knot(&mut self, knot_index: usize, new_x: isize, new_y: isize) {
        self.knot_positions[knot_index] = (new_x, new_y)
    }

    // Movement for knot x relative to knot x - 1
    fn knot_movement(&self, tail_index: usize) -> (Option<Direction>, Option<Direction>) {
        let (h_x, h_y) = self.knot_pos(tail_index - 1);
        let (t_x, t_y) = self.knot_pos(tail_index);
        let x_deviation = h_x - t_x;
        let x_deviation_abs = x_deviation.abs();
        let y_deviation = h_y - t_y;
        let y_deviation_abs = y_deviation.abs();

        let diagonal_deviation =
            x_deviation_abs > 0 && y_deviation_abs > 0 && x_deviation_abs + y_deviation_abs >= 3;

        let x_direction = if diagonal_deviation || x_deviation_abs > 1 {
            match x_deviation.signum() {
                -1 => Some(Direction::Left(1)),
                1 => Some(Direction::Right(1)),
                _ => panic!("Bad signum"),
            }
        } else {
            None
        };

        let y_direction = if diagonal_deviation || y_deviation_abs > 1 {
            match y_deviation.signum() {
                -1 => Some(Direction::Down(1)),
                1 => Some(Direction::Up(1)),
                _ => panic!("Bad signum"),
            }
        } else {
            None
        };

        // This is the correction value that will move the tail to the right position
        (x_direction, y_direction)
    }

    fn move_knot(&mut self, knot_index: usize, direction: &Direction) {
        let distance = direction.distance();

        let (h_x, h_y) = self.knot_pos(knot_index);

        if distance > 0 {
            match direction {
                Direction::Up(_) => {
                    self.update_knot(knot_index, *h_x, h_y + 1);
                }
                Direction::Down(_) => {
                    self.update_knot(knot_index, *h_x, h_y - 1);
                }
                Direction::Left(_) => {
                    self.update_knot(knot_index, h_x - 1, *h_y);
                }
                Direction::Right(_) => {
                    self.update_knot(knot_index, h_x + 1, *h_y);
                }
            }

            for i in 1..self.num_knots {
                self.adjust_knot(i);
            }

            let distance_left = distance - 1;

            let new_direction = match direction {
                Direction::Up(_) => Direction::Up(distance_left),
                Direction::Down(_) => Direction::Down(distance_left),
                Direction::Left(_) => Direction::Left(distance_left),
                Direction::Right(_) => Direction::Right(distance_left),
            };
            self.move_knot(knot_index, &new_direction)
        };
    }

    fn adjust_knot(&mut self, knot_index: usize) {
        match self.knot_movement(knot_index) {
            (None, None) => {}
            (None, Some(y_direction)) => self.move_tail(&y_direction, knot_index),
            (Some(x_direction), None) => self.move_tail(&x_direction, knot_index),
            (Some(x_direction), Some(y_direction)) => {
                self.move_tail(&x_direction, knot_index);
                self.move_tail(&y_direction, knot_index);
            }
        }

        if knot_index == self.num_knots - 1 {
            self.visit();
        }
    }

    fn move_tail(&mut self, direction: &Direction, knot_index: usize) {
        let (t_x, t_y) = self.knot_pos(knot_index);
        match direction {
            Direction::Right(_) => self.update_knot(knot_index, t_x + 1, *t_y),
            Direction::Left(_) => self.update_knot(knot_index, t_x - 1, *t_y),
            Direction::Up(_) => self.update_knot(knot_index, *t_x, t_y + 1),
            Direction::Down(_) => self.update_knot(knot_index, *t_x, t_y - 1),
        }
    }

    fn visit(&mut self) {
        let (t_x, t_y) = self.knot_pos(self.num_knots - 1);
        self.t_visited.insert((*t_x, *t_y));
    }
}

#[aoc(day9, part1)]
fn part1(input: &[Direction]) -> usize {
    let mut grid = Grid::new(2);
    input
        .iter()
        .for_each(|movement| grid.move_knot(0, movement));

    // visualize the path
    // let x_min_index = grid
    //     .t_visited
    //     .iter()
    //     .min_by(|one, two| one.0.cmp(&two.0))
    //     .unwrap()
    //     .0;
    // let x_max_index = grid
    //     .t_visited
    //     .iter()
    //     .max_by(|one, two| one.0.cmp(&two.0))
    //     .unwrap()
    //     .0;
    // let y_min_index = grid
    //     .t_visited
    //     .iter()
    //     .min_by(|one, two| one.1.cmp(&two.1))
    //     .unwrap()
    //     .1;
    // let y_max_index = grid
    //     .t_visited
    //     .iter()
    //     .max_by(|one, two| one.1.cmp(&two.1))
    //     .unwrap()
    //     .1;
    // for y in (y_min_index..=y_max_index).rev() {
    //     for x in x_min_index..=x_max_index {
    //         if x == 0 && y == 0 {
    //             print!("s");
    //         } else {
    //             if grid.t_visited.contains(&(x as isize, y as isize)) {
    //                 print!("#")
    //             } else {
    //                 print!(".")
    //             }
    //         }
    //     }
    //     print!("\n");
    // }

    grid.t_visited.len()
}

#[aoc(day9, part2)]
fn part2(input: &[Direction]) -> usize {
    let mut grid = Grid::new(10);
    input
        .iter()
        .for_each(|movement| grid.move_knot(0, movement));

    grid.t_visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn part2_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part2(&input), 1);
    }
}
