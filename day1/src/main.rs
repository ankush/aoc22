fn main() {
    let input = std::fs::read_to_string("./src/input.in").unwrap();
    println!("Max food {}", part1(&input));
    println!("Sum of top 3 food {}", part2(&input));
}

fn get_elf_foods(inventory: &str) -> Vec<i32> {
    let mut elf_foods = vec![];
    let mut current_food = 0;

    for food in inventory.lines() {
        if food == "" {
            elf_foods.push(current_food);
            current_food = 0;
        } else {
            current_food += food.trim().parse::<i32>().unwrap();
        }
    }
    elf_foods
}

fn part1(inventory: &str) -> i32 {
    let elf_foods = get_elf_foods(inventory);

    elf_foods.iter().max().unwrap_or(&0).to_owned()
}


fn part2(inventory: &str) -> i32 {
    let mut elf_foods = get_elf_foods(inventory);
    elf_foods.sort();
    elf_foods.reverse();

    let top3_sum = &elf_foods[0..=2];
    top3_sum.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1(""), 0);

        const TEST_INPUT : &str = "1
            41

            40
            1";
        assert_eq!(part1(TEST_INPUT), 42);
    }

    #[test]
    fn part2_works() {
        const TEST_INPUT : &str = "1
            20

            10
            5

            3
            3

            1
            1
            1";
        assert_eq!(part2(TEST_INPUT), 42);
    }
}
