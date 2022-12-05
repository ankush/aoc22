#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./src/input.in").unwrap();
    println!("{}", part1(&input));
}

fn part1(input: &str) -> String {
    let (mut arrangement, moves) = parse_input(input);

    for m in moves {
        for _ in 0..m.count {
            let from_stack = arrangement.get_mut(&m.from).unwrap();
            let crate_to_move = from_stack.pop().unwrap();
            let to_stack = arrangement.get_mut(&m.to).unwrap();
            to_stack.push(crate_to_move);
        }
    }

    let mut keys: Vec<&i32> = arrangement.keys().collect();
    keys.sort();

    keys.iter()
        .map(|k| arrangement.get(k).unwrap())
        .map(|v| v.last().unwrap())
        .into_iter()
        .collect()
}

struct Move {
    count: i32,
    from: i32,
    to: i32,
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<char>>, Vec<Move>) {
    let instructions_start = input.find("move").unwrap();
    let crates_diagram = &input[..instructions_start];

    let instructions = &input[instructions_start..];

    let mut arrangement: HashMap<i32, Vec<char>> = HashMap::new();

    let mut crates: Vec<&str> = crates_diagram.trim_end().lines().collect();
    let stack_numbers = crates.pop().unwrap();

    for (i, char) in stack_numbers.char_indices() {
        if char == ' ' {
            continue;
        }

        let idx: i32 = char.to_string().parse().unwrap();

        for line in crates.iter().rev() {
            for (j, crate_char) in line.char_indices() {
                if i != j || "[ ]".contains(crate_char) {
                    continue;
                }
                let stack = arrangement.entry(idx).or_insert(vec![]);
                stack.push(crate_char)
            }
        }
    }

    let mut moves = vec![];
    for instruction in instructions.lines() {
        let parts: Vec<&str> = instruction.split_whitespace().collect();

        moves.push(Move {
            count: parts[1].parse().unwrap(),
            from: parts[3].parse().unwrap(),
            to: parts[5].parse().unwrap(),
        })
    }
    (arrangement, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), "CMZ");
    }
}
