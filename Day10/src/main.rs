use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut instructions = read_input();

    println!("{:?}", part_2_solution(&mut instructions));
}

fn part_1_solution(instructions: &mut Vec<i32>) -> i32 {
    let mut diff_one_count = 0;
    let mut diff_three_count = 0;
    let mut prev = 0;

    instructions.sort();

    for num in instructions {
        if num.clone() - prev == 1 {
            diff_one_count += 1;
        }
        if num.clone() - prev == 3 {
            diff_three_count += 1;
        }

        prev = num.clone();
    }

    diff_three_count += 1;

    diff_one_count * diff_three_count
}

fn part_2_solution(instructions: &mut Vec<i32>) -> i64 {
    let mut dp = [0; 300];

    instructions.sort();
    instructions.push(instructions[instructions.len() - 1] + 3);
    instructions.insert(0, 0);

    println!("{:?}", instructions);

    dp[0] = 1;
    dp[1] = 1;
    dp[2] = 2;

    for index in 3..instructions.len() {
        for j in index - 3..index {
            let diff = instructions[index].clone() - instructions[j].clone();
            if diff <= 3 {
                dp[index] += dp[j] as i64;
            }
        }
    }

    dp[instructions.len() - 1]
}

fn read_input() -> Vec<i32> {
    let mut res = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                res.push(l.trim().parse::<i32>().unwrap());
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
