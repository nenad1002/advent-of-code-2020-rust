use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let declarations = read_input();

    println!("{}", part_2_solution(declarations));
}

fn part_1_solution(declarations: Vec<String>) -> i32 {
    let mut res: i32 = 0;
    let mut set = HashSet::new();

    for declaration in declarations {
        if declaration != "" {
            for letter in declaration.chars() {
                set.insert(letter);
            }
        } else {
            res += set.len() as i32;
            set.clear();
        }
    }

    res += set.len() as i32;

    res
}

fn part_2_solution(declarations: Vec<String>) -> i32 {
    let mut res: i32 = 0;
    let mut set = HashSet::new();
    let mut new_set = HashSet::new();
    let mut first_in_group = true;

    for declaration in declarations {
        if declaration != "" {
            for letter in declaration.chars() {
                if first_in_group {
                    new_set.insert(letter);
                } else {
                    if set.contains(&letter) {
                        new_set.insert(letter);
                    }
                }
            }

            first_in_group = false;
            set = new_set.clone();
            new_set.clear();
        } else {
            first_in_group = true;
            res += set.len() as i32;
            set.clear();
        }
    }

    res += set.len() as i32;

    res
}

fn read_input() -> Vec<String> {
    let mut res = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                res.push(l);
            }
        }
    }

    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
