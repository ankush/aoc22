use take_until::TakeUntilExt;

fn main() {
    let input = std::fs::read_to_string("./src/input.in").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

type Grid = Vec<Vec<i32>>;

fn part1(input: &str) -> i32 {
    let grid = parse_input(input);

    let y_max = grid.len();
    let x_max = grid[0].len();

    let perimeter_trees = (x_max + y_max) as i32 * 2 - 4;

    let mut visible_interior_trees = 0;

    for y in 1..y_max - 1 {
        for x in 1..x_max - 1 {
            if is_visible(&grid, x, y) {
                visible_interior_trees += 1;
            }
        }
    }

    perimeter_trees + visible_interior_trees
}

fn part2(input: &str) -> i32 {
    let grid = parse_input(input);

    let y_max = grid.len();
    let x_max = grid[0].len();

    let mut max_score = 0;
    for y in 1..y_max - 1 {
        for x in 1..x_max - 1 {
            max_score = std::cmp::max(max_score, scenic_score(&grid, x, y));
        }
    }
    max_score
}

fn is_visible(grid: &Grid, x: usize, y: usize) -> bool {
    let y_max = grid.len();
    let x_max = grid[0].len();

    let tree_height = grid[y][x];

    let top = grid[0..y].iter().all(|v| v[x] < tree_height);
    let bottom = grid[y + 1..y_max].iter().all(|v| v[x] < tree_height);

    let left = grid[y][0..x].iter().all(|v| *v < tree_height);
    let right = grid[y][x + 1..x_max].iter().all(|v| *v < tree_height);

    top || bottom || left || right
}

fn scenic_score(grid: &Grid, x: usize, y: usize) -> i32 {
    let y_max = grid.len();
    let x_max = grid[0].len();

    let tree_height = grid[y][x];

    let top = grid[0..y]
        .iter()
        .rev()
        .take_until(|v| v[x] >= tree_height)
        .count();
    let bottom = grid[y + 1..y_max]
        .iter()
        .take_until(|v| v[x] >= tree_height)
        .count();

    let left = grid[y][0..x]
        .iter()
        .rev()
        .take_until(|v| **v >= tree_height)
        .count();
    let right = grid[y][x + 1..x_max]
        .iter()
        .take_until(|v| **v >= tree_height)
        .count();

    (top * bottom * left * right) as i32
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 21);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 8);
    }
}
