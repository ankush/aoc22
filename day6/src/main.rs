fn main() {
    let input = std::fs::read_to_string("./src/input.in").unwrap();
    println!("{}", part1(&input));
}

fn part1(input: &str) -> i32 {
    for i in 4..input.len() {
        let last_section = &input[i - 4..i];
        if unique(last_section) {
            return i as i32;
        }
    }

    panic!("Didn't find any");
}

fn unique(string: &str) -> bool {
    string
        .char_indices()
        .find_map(|(i, c)| string.chars().skip(i + 1).find(|x| *x == c))
        .is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
