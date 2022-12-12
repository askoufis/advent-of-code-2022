use std::collections::HashMap;

use nom::{
    character::complete::{char, none_of},
    multi::{many1, separated_list1},
    IResult,
};

fn find_start(locations: &[Vec<Location>]) -> Point {
    let len = locations[0].len();

    for y in 0..len {
        for x in 0..len {
            let loc = &locations[y][x];
            if let Location::Start(_) = loc {
                return Point { x, y };
            }
        }
    }

    unreachable!("Couldn't find the start")
}

fn find_end(locations: &[Vec<Location>]) -> Point {
    let y_max = locations.len();
    let x_max = locations[0].len();

    for y in 0..y_max {
        for x in 0..x_max {
            let loc = &locations[y][x];
            if let Location::End(_) = loc {
                return Point { x, y };
            }
        }
    }

    unreachable!("Couldn't find the end")
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Location {
    Start(char),
    End(char),
    // Locations that are neither start nor end
    Pos(char),
}

impl Location {
    fn new(c: char) -> Self {
        match c {
            'S' => Self::Start('a'),
            'E' => Self::End('z'),
            c => Self::Pos(c),
        }
    }

    fn char(&self) -> char {
        match self {
            Location::Start(c) => *c,
            Location::End(c) => *c,
            Location::Pos(c) => *c,
        }
    }
}

struct Map {
    map: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    start: Point,
    end: Point,
    distance_from_start: HashMap<Point, usize>,
}

impl Map {
    fn new(input: &Vec<Vec<Location>>, ascend: bool) -> Self {
        let width = input[0].len();
        let height = input.len();
        let start = find_start(&input);
        let end = find_end(&input);
        let map = input
            .iter()
            .map(|inner| {
                inner
                    .iter()
                    .map(|l| l.char() as usize - 'a' as usize)
                    .collect()
            })
            .collect();

        let mut distance_from_start = HashMap::new();
        if ascend {
            distance_from_start.insert(start.clone(), 0);
        } else {
            distance_from_start.insert(end.clone(), 0);
        }

        Self {
            map,
            width,
            height,
            start,
            end,
            distance_from_start,
        }
    }

    fn traverse(&mut self, points: &[Point], ascend: bool) {
        let mut new_points: Vec<Point> = Vec::new();

        for point in points {
            let distance = self
                .distance_from_start
                .get(point)
                .expect("No distance found")
                + 1;

            let directions = self.spread(point, ascend);
            for dir in directions {
                if !self.distance_from_start.contains_key(&dir) {
                    self.distance_from_start.insert(dir.clone(), distance);
                    new_points.push(dir);
                }
            }
        }
        if new_points.len() != 0 {
            self.traverse(&new_points, ascend)
        }
    }

    fn height_at(&self, x: usize, y: usize) -> usize {
        self.map[y][x]
    }

    fn can_ascend(&self, current_point: &Point, to: &Point) -> bool {
        let height = self.height_at(current_point.x, current_point.y);
        let height_to = self.height_at(to.x, to.y);

        height + 1 == height_to || height >= height_to
    }

    fn can_descend(&self, current_point: &Point, to: &Point) -> bool {
        let height = self.height_at(current_point.x, current_point.y);
        let height_to = self.height_at(to.x, to.y);

        let can = height == height_to || height_to > height || height - 1 == height_to;
        // println!("{can}");
        can
    }

    fn spread(&self, current_point: &Point, ascend: bool) -> Vec<Point> {
        let up = self.up(current_point, ascend);
        let down = self.down(current_point, ascend);
        let left = self.left(current_point, ascend);
        let right = self.right(current_point, ascend);

        let directions = vec![up, down, left, right];
        directions.into_iter().filter_map(|x| x).collect()
    }

    fn up(&self, current_point: &Point, ascend: bool) -> Option<Point> {
        if current_point.y == 0 {
            return None;
        }

        let to = Point {
            x: current_point.x,
            y: current_point.y - 1,
        };

        if ascend {
            if self.can_ascend(current_point, &to) {
                Some(to)
            } else {
                None
            }
        } else {
            if self.can_descend(current_point, &to) {
                Some(to)
            } else {
                None
            }
        }
    }

    fn down(&self, current_point: &Point, ascend: bool) -> Option<Point> {
        if current_point.y >= self.height - 1 {
            return None;
        }
        let to = Point {
            x: current_point.x,
            y: current_point.y + 1,
        };

        if ascend {
            if self.can_ascend(current_point, &to) {
                Some(to)
            } else {
                None
            }
        } else {
            if self.can_descend(current_point, &to) {
                Some(to)
            } else {
                None
            }
        }
    }

    fn left(&self, current_point: &Point, ascend: bool) -> Option<Point> {
        if current_point.x == 0 {
            return None;
        }

        let to = Point {
            x: current_point.x - 1,
            y: current_point.y,
        };

        if ascend {
            if self.can_ascend(current_point, &to) {
                Some(to)
            } else {
                None
            }
        } else {
            if self.can_descend(current_point, &to) {
                Some(to)
            } else {
                None
            }
        }
    }

    fn right(&self, current_point: &Point, ascend: bool) -> Option<Point> {
        if current_point.x >= self.width - 1 {
            return None;
        }

        let to = Point {
            x: current_point.x + 1,
            y: current_point.y,
        };

        if ascend {
            if self.can_ascend(current_point, &to) {
                Some(to)
            } else {
                None
            }
        } else {
            if self.can_descend(current_point, &to) {
                Some(to)
            } else {
                None
            }
        }
    }
}

fn parse_location(input: &str) -> IResult<&str, Location> {
    let (input, result) = none_of("\n")(input)?;
    let location = Location::new(result);

    IResult::Ok((input, location))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Location>> {
    many1(parse_location)(input)
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Vec<Vec<Location>> {
    separated_list1(char('\n'), parse_line)(input)
        .ok()
        .unwrap()
        .1
}

#[aoc(day12, part1)]
fn part1(input: &Vec<Vec<Location>>) -> usize {
    let ascend = true;
    let mut map = Map::new(input, ascend);
    map.traverse(&[map.start.clone()], ascend);

    *map.distance_from_start
        .get(&map.end)
        .expect("No end distance")
}

#[aoc(day12, part2)]
fn part2(input: &Vec<Vec<Location>>) -> usize {
    let ascend = false;
    let mut map = Map::new(input, ascend);
    map.traverse(&[map.end.clone()], ascend);

    let mut a_points: Vec<Point> = vec![];

    for y in 0..map.height {
        for x in 0..map.width {
            if map.height_at(x, y) == 0 {
                a_points.push(Point { x, y })
            }
        }
    }

    *a_points
        .iter()
        .filter_map(|p| map.distance_from_start.get(p))
        .min()
        .expect("No min")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn part2_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part2(&input), 29);
    }
}
