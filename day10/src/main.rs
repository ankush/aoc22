fn main() {
    let input = std::fs::read_to_string("./src/input.in").unwrap();
    println!("{}", part1(&input));
    part2(&input);
}

const IMPORTANT_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];

fn part1(input: &str) -> i32 {
    let instructions = parse_input(input);

    let history = execute_instructions(&instructions);

    IMPORTANT_CYCLES
        .map(|cycle| history[cycle as usize - 1] * cycle)
        .iter()
        .sum()
}

fn part2(input: &str) {
    let instructions = parse_input(input);
    let history = execute_instructions(&instructions);

    let mut crt = vec!['.'; 240];

    for (cycle, x) in history.iter().enumerate() {
        let x_crt = (cycle as i32) % 40;
        if (x-1..=x+1).contains(&x_crt) {
            crt[cycle] = '#';
        }
    }

    let output: Vec<String> = crt
        .chunks(40)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    dbg!(output);  // ¯\_(ツ)_/¯
}

fn execute_instructions(instructions: &Vec<Instruction>) -> Vec<i32> {
    let mut x = 1;
    let mut history = vec![];

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                history.push(x);
            }
            Instruction::Add { value } => {
                // Add takes two cycles.
                history.push(x);
                history.push(x);
                x += value;
            }
        }
    }

    history
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            match (parts.next(), parts.next()) {
                (Some("noop"), _) => Instruction::Noop,
                (Some("addx"), Some(count)) => Instruction::Add {
                    value: count.parse().unwrap(),
                },
                _ => panic!("Unknown instruction"),
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Add { value: i32 },
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 13140);
    }
}
