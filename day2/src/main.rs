fn main() {
    let input = std::fs::read_to_string("./src/input.in").unwrap();
    println!("Total score: {}", part1(&input));
    println!("Total score: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let games = get_games(input);
    games.iter().map(|g| g.score()).sum()
}

fn part2(input: &str) -> i32 {
    let strategies = get_strategies(input);
    strategies
        .iter()
        .map(|s| s.construct_game())
        .map(|g| g.score())
        .sum()
}

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rocks,
    Paper,
    Scissors,
}

impl Move {
    const ALL_MOVES: [Move; 3]= [Move::Rocks, Move::Paper, Move::Scissors];

    fn score(&self) -> i32 {
        match *self {
            Move::Rocks => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    // What this move beats
    fn beats(self) -> Move {
        match self {
            Move::Rocks => Move::Scissors,
            Move::Paper => Move::Rocks,
            Move::Scissors => Move::Paper,
        }
    }

    // copied from https://fasterthanli.me/series/advent-of-code-2022/part-2
    fn winning_move(self) -> Move {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats() == self)
            .unwrap()
    }

    fn losing_move(self) -> Move {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| *m == self.beats())
            .unwrap()
    }

    fn drawing_move(self) -> Move {
        self
    }
    // end copied
}

enum Outcome {
    Loss,
    Win,
    Draw,
}

impl Outcome {
    fn score(&self) -> i32 {
        match *self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

struct Game {
    them: Move,
    us: Move,
}

impl Game {
    fn get_outcome(&self) -> Outcome {
        if self.them == self.us {
            Outcome::Draw
        } else if self.them == self.us.beats() {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }

    fn score(&self) -> i32 {
        self.us.score() + self.get_outcome().score()
    }
}

struct Stratagy {
    expected_move: Move,
    outcome: Outcome,
}

impl Stratagy {
    fn suggest_move(&self) -> Move {
        match &self.outcome {
            Outcome::Loss => self.expected_move.losing_move(),
            Outcome::Win => self.expected_move.winning_move(),
            Outcome::Draw => self.expected_move.drawing_move(),
        }
    }

    fn construct_game(&self) -> Game {
        Game {
            them: self.expected_move,
            us: self.suggest_move(),
        }
    }
}

fn map_code_to_move(code: &str) -> Move {
    match code {
        "A" | "X" => Move::Rocks,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => panic!("Unknown code {}", code),
    }
}

fn map_code_to_outcome(code: &str) -> Outcome {
    match code {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Unknown code {}", code),
    }
}

fn get_games(input: &str) -> Vec<Game> {
    let mut games = vec![];

    for gamestr in input.lines() {
        let moves: Vec<&str> = gamestr.split_whitespace().collect();
        let game = Game {
            them: map_code_to_move(moves[0]),
            us: map_code_to_move(moves[1]),
        };
        games.push(game);
    }

    games
}

fn get_strategies(input: &str) -> Vec<Stratagy> {
    let mut games = vec![];

    for gamestr in input.lines() {
        let moves: Vec<&str> = gamestr.split_whitespace().collect();
        let game = Stratagy {
            expected_move: map_code_to_move(moves[0]),
            outcome: map_code_to_outcome(moves[1]),
        };
        games.push(game);
    }

    games
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        assert_eq!(part1(""), 0);
        assert_eq!(part1(TEST_INPUT), 15);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(""), 0);
        assert_eq!(part2(TEST_INPUT), 12);
    }
}
