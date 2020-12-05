use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let binary_codes = read_input();

    println!("{}", part_2_solution(binary_codes));
}

fn get_part_num(s: &String, left: usize, right: usize, comparison_char: char) -> i32 {
    let mut res = 0;
    let mut mult = 1;

    for i in (left..right).rev() {
        let c = s.chars().nth(i).unwrap();

        if c == comparison_char {
            res += mult;
        }

        mult *= 2;
    }

    res
}

fn part_1_solution(binary_codes: Vec<String>) -> i32 {
    let mut res = 0;
    for code in binary_codes {
        let left_num = get_part_num(&code, 0, 7, 'B');
        let right_num = get_part_num(&code, 7, 10, 'R');

        res = cmp::max(res, 8 * left_num + right_num);
    }
    res
}

fn part_2_solution(binary_codes: Vec<String>) -> i32 {
    let mut arr = [false; 1024];
    for code in binary_codes {
        let left_num = get_part_num(&code, 0, 7, 'B');
        let right_num = get_part_num(&code, 7, 10, 'R');

        arr[(8 * left_num + right_num) as usize] = true;
    }

    for i in 100..1024 {
        if arr[i] == false {
            return i as i32;
        }
    }
    -1
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
