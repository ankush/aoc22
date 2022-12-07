#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./src/input.in").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

type DirStack<'a> = Vec<&'a str>;
type DirMap = HashMap<String, i32>;

fn part1(input: &str) -> i32 {
    let lines = parse_input(input);
    let sizes = construct_dir_map(&lines);

    sizes
        .iter()
        .filter(|(_, size)| **size < 100000)
        .map(|(_, size)| size)
        .sum()
}

const DISK_CAPACITY: i32 = 70000000;
const REQUIRED_CAPACITY: i32 = 30000000;

fn part2(input: &str) -> i32 {
    let lines = parse_input(input);
    let sizes = construct_dir_map(&lines);

    let unused_space = DISK_CAPACITY - *sizes.get("/").unwrap();
    let required_cleanup = REQUIRED_CAPACITY - unused_space;

    *sizes
        .iter()
        .filter(|(_, size)| **size > required_cleanup)
        .map(|(_, size)| size)
        .min()
        .unwrap()
}

fn construct_dir_map(lines: &Vec<ShellLine>) -> DirMap {
    let mut pwd: DirStack = vec![];
    let mut sizes: HashMap<String, i32> = HashMap::new();

    for line in lines.into_iter() {
        match line {
            ShellLine::Command {
                cmd: "cd",
                arg: Some(arg),
            } => {
                cd(&mut pwd, arg);
            }
            ShellLine::File {
                is_dir: false,
                size,
                ..
            } => {
                update_sizes(&mut sizes, &pwd, *size);
            }
            _ => continue,
        }
    }
    sizes
}

fn update_sizes(sizes: &mut HashMap<String, i32>, pwd: &DirStack, size: i32) {
    for i in 0..=pwd.len() {
        let dir = format!("/{}", &pwd[..i].join("/"));

        let total_size = sizes.entry(dir).or_insert(0);
        *total_size = *total_size + size;
    }
}

fn cd<'a>(pwd: &mut DirStack<'a>, directory: &'a str) {
    match directory {
        "/" => pwd.clear(),
        ".." => {
            pwd.pop();
        }
        _ => pwd.push(directory),
    };
}

#[derive(Debug)]
enum ShellLine<'a> {
    Command {
        cmd: &'a str,
        arg: Option<&'a str>,
    },
    File {
        is_dir: bool,
        size: i32,
        name: &'a str,
    },
}

fn convert_to_shellline(s: &str) -> ShellLine {
    let mut parts = s.split_whitespace();

    match (parts.next(), parts.next(), parts.next()) {
        (Some("$"), Some(cmd), arg) => ShellLine::Command { cmd, arg },
        (Some("dir"), Some(name), None) => ShellLine::File {
            is_dir: true,
            name,
            size: 0,
        },
        (Some(size), Some(name), None) => ShellLine::File {
            is_dir: false,
            name,
            size: size.parse().unwrap(),
        },
        _ => panic!("Unknown shell output {s}"),
    }
}

fn parse_input(input: &str) -> Vec<ShellLine> {
    input.lines().map(convert_to_shellline).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 95437);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 24933642);
    }
}
