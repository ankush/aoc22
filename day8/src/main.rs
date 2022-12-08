fn main() {
    let input = std::fs::read_to_string("./src/input.in").unwrap();
    println!("{}", part1(&input));
}

type Grid = Vec<Vec<i32>>;

fn part1(input: &str) -> i32 {
    let grid = parse_input(input);

    let y = grid.len();
    let x = grid[0].len();

    let perimeter_trees = (x + y) as i32 * 2 - 4 ;
    dbg!(perimeter_trees);

    let mut visible_interior_trees = 0;

    for i in 1..y-1 {
        for j in 1..x-1 {
            if is_visible(&grid, i, j) {
                visible_interior_trees += 1;
            }
        }
    }

    perimeter_trees + visible_interior_trees
}


fn is_visible(grid: &Grid, i: usize, j: usize) -> bool {
    let y = grid.len();
    let x = grid[0].len();

    let tree_height = grid[j][i];

    // bottom - same x, y decreasing

    let top = grid[0..j].iter().all(|v| v[i] < tree_height);
    let bottom = grid[j+1..y].iter().all(|v| v[i] < tree_height);

    let left = grid[j][0..i].iter().all(|v| *v < tree_height);
    let right = grid[j][i+1..x].iter().all(|v| *v < tree_height);

    top || bottom || left || right
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

}
