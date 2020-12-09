use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut instructions = read_input();

    println!("{}", part_2_solution(&mut instructions));
}

fn check_and_reverse_jmp(line_items: &mut Vec<String>) {
    if line_items[0] == "jmp" {
        line_items[0] = "nop".to_string();
    } else if line_items[0] == "nop" {
        line_items[0] = "jmp".to_string();
    }
}

fn part_2_solution(instructions: &mut Vec<String>) -> i32 {
    for i in 0..instructions.len() {
        let mut line_items: Vec<String> = instructions[i].split(' ').map(String::from).collect();

        check_and_reverse_jmp(&mut line_items);

        instructions[i] = line_items.join(" ");

        let result = run(&instructions);

        check_and_reverse_jmp(&mut line_items);

        instructions[i] = line_items.join(" ");

        if result.0 == true {
            return result.1;
        }
    }
    0
}

fn run(instructions: &Vec<String>) -> (bool, i32) {
    let mut visited: Vec<bool> = vec![false; instructions.len() as usize];

    let mut index = 0 as usize;

    let mut res = 0;

    loop {
        if index == instructions.len() {
            return (true, res);
        }

        if visited[index] == true {
            return (false, res);
        }

        let line = instructions.get(index).unwrap();
        let line_items: Vec<String> = line.split(' ').map(String::from).collect();
        let instruction = line_items[0].clone();
        let offset = line_items[1].trim().clone().parse::<i32>().unwrap();

        visited[index] = true;

        if instruction == "acc" {
            res += offset;
            index += 1;
        } else if instruction == "jmp" {
            index = (index as i32 + offset) as usize;
        } else {
            index += 1;
        }
    }
}

fn part_1_solution(instructions: Vec<String>) -> i32 {
    run(&instructions).1
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
