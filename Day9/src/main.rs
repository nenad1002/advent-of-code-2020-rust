use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let instructions = read_input();

    println!("{:?}", part_2_solution(instructions, 22406676));
}

fn part_1_solution(instructions: Vec<String>, preample_length: usize) -> i32 {
    let mut map = HashMap::new();

    let mut index = 0 as usize;
    for line in &instructions {
        let num = line.trim().parse::<i32>().unwrap();

        if index >= preample_length {
            let mut is_found = false;
            for j in index - preample_length..index {
                let diff_two_numbers = num - instructions[j].clone().trim().parse::<i32>().unwrap();

                let part_other_num = map.get(&diff_two_numbers) as Option<&usize>;

                if part_other_num != None {
                    if diff_two_numbers == num - diff_two_numbers {
                        continue;
                    }
                    let other_num_index = part_other_num.unwrap().clone();

                    if other_num_index < index - preample_length {
                        continue;
                    }
                    is_found = true;
                    break;
                }
            }

            if !is_found {
                return num;
            }
        }

        map.insert(num, index.clone());

        index += 1;
    }
    0
}

fn part_2_solution(instructions: Vec<String>, target: i32) -> i32 {
    let mut j = 0 as usize;
    let mut i = 0 as usize;

    let mut sum = 0;
    while j < instructions.len() {
        sum += instructions[j].clone().trim().parse::<i32>().unwrap();

        while sum > target {
            sum -= instructions[i].clone().trim().parse::<i32>().unwrap();
            i += 1;
        }

        if sum == target {
            break;
        }

        j += 1;
    }

    let mut min = 999999999;
    let mut max = 0;

    for index in i..j + 1 {
        let n = instructions[index].clone().trim().parse::<i32>().unwrap();
        min = std::cmp::min(min, n);
        max = std::cmp::max(max, n);
    }
    return min + max;
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
