use std::str::FromStr;

fn main() {
    color_eyre::install().unwrap();

    let input = std::fs::read_to_string("./src/input.in").unwrap();
    println!("Fully overlapping pairs {}", part1(&input));
    println!("Any overlapping pairs {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let assignments = parse_assignments(input);

    assignments
        .iter()
        .filter(|a| a.first.contains(&a.second) || a.second.contains(&a.first))
        .count() as i32
}

fn part2(input: &str) -> i32 {
    let assignments = parse_assignments(input);

    assignments
        .iter()
        .filter(|a| a.first.overlap(&a.second))
        .count() as i32
}

struct Range {
    start: i32,
    end: i32,
}

struct Assignment {
    first: Range,
    second: Range,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    fn overlap(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

impl FromStr for Range {
    type Err = color_eyre::Report;

    // Construct range from strings like `12-20`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s.split('-').collect();

        Ok(Self {
            start: values[0].parse::<i32>()?,
            end: values[1].parse::<i32>()?,
        })
    }
}

impl FromStr for Assignment {
    type Err = color_eyre::Report;

    // Construct assingment pair from strings like `12-20,1-2`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: Vec<&str> = s.split(',').collect();

        Ok(Self {
            first: ranges[0].parse().unwrap(),
            second: ranges[1].parse().unwrap(),
        })
    }
}

fn parse_assignments(input: &str) -> Vec<Assignment> {
    input
        .lines()
        .map(|line| line.parse::<Assignment>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        assert_eq!(part1(""), 0);
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(""), 0);
        assert_eq!(part2(TEST_INPUT), 4);
    }

    #[test]
    fn range_contains() {
        assert!(Range { start: 2, end: 8 }.contains(&Range { start: 3, end: 7 }));
        assert!(!Range { start: 2, end: 8 }.contains(&Range { start: 1, end: 7 }));
    }
}
