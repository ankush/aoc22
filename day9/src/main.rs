use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("./src/input.in").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let movements = parse_input(input);

    let mut head = Point(0, 0);
    let mut tail = Point(0, 0);

    let mut visited: HashSet<Point> = HashSet::new();

    for m in movements {
        let vector = m.direction.unit_vector();
        for _ in 0..m.magnitude {
            head.displace(&vector);
            tail.follow(&head);
            visited.insert(tail.clone());
        }
    }

    visited.len() as i32
}

fn part2(input: &str) -> i32 {
    let movements = parse_input(input);

    let mut rope = [Point(0, 0); 10];
    let mut visited: HashSet<Point> = HashSet::new();

    for m in movements {
        let vector = m.direction.unit_vector();
        for _ in 0..m.magnitude {

            let head = rope.first_mut().unwrap();
            head.displace(&vector);

            for i in 1..rope.len() {
                let leader = rope.get(i - 1).unwrap().clone(); // fought so hard but BC won.
                let follower = rope.get_mut(i).unwrap();
                follower.follow(&leader);
            }

            visited.insert(rope.last().unwrap().clone());
        }
    }

    visited.len() as i32
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// x-y point
#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
struct Point(i32, i32);

// Allow adding a point to a point.
impl Point {
    fn displace(&mut self, vector: &Point) {
        self.0 += vector.0;
        self.1 += vector.1;
    }

    fn touching(&self, other: &Point) -> bool {
        // imagine a 3x3 square wrapping the point.
        // if both x and y are in +/- 1 range
        // then we are touching another point
        (self.0 - 1..=self.0 + 1).contains(&other.0) && (self.1 - 1..=self.1 + 1).contains(&other.1)
    }

    // Implements tail's follow behaviour
    fn follow(&mut self, point: &Point) {
        if self.touching(&point) {
            return;
        }
        // We can make move like queen, 1 step in any direction or diagonal.
        // direction vector has to be computed from comparing destination
        let x_delta = point.0 - self.0;
        let y_delta = point.1 - self.1;

        let vector = Point(limit(x_delta), limit(y_delta));
        self.displace(&vector);
    }
}

// limit number to 1 while preserving sign
fn limit(num: i32) -> i32 {
    match num {
        0 => 0,
        num if num > 0 => std::cmp::min(1, num),
        _ => std::cmp::max(-1, num),
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "R" => Self::Right,
            "L" => Self::Left,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => panic!("Unkown direction {s}"),
        }
    }
}

impl Direction {
    fn unit_vector(&self) -> Point {
        match self {
            Self::Right => Point(1, 0),
            Self::Left => Point(-1, 0),
            Self::Up => Point(0, 1),
            Self::Down => Point(0, -1),
        }
    }
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    magnitude: i32,
}

impl From<&str> for Movement {
    fn from(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        if let (Some(direction), Some(magnitude)) = (parts.next(), parts.next()) {
            return Movement {
                direction: Direction::from(direction),
                magnitude: magnitude.parse().unwrap(),
            };
        }
        panic!("Unknown directions {s}");
    }
}

fn parse_input(input: &str) -> Vec<Movement> {
    input.lines().map(|line| Movement::from(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 1);
    }
}
