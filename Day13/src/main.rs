use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub const MAX: i32 = i32::MAX;

fn main() {
    let mut ins = read_input();
    println!("{:?}", part_1_solution(&mut ins));
}

fn part_1_solution(instructions: &Vec<String>) -> i32 {
    let num = instructions[0].parse::<i32>().unwrap();
    let buses: Vec<&str> = instructions[1].split(",").collect();
    let mut min_remainder = MAX;
    let mut final_bus = -1;
    for bus in buses {
        if bus == "x" {
            continue;
        }
        let bus_num = bus.parse::<i32>().unwrap();

        let remainder = bus_num - num % bus_num;

        if remainder < min_remainder {
            min_remainder = remainder;
            final_bus = bus_num;
        }
    }

    final_bus * min_remainder
}

fn rotate(direct: &mut [i32; 2], count: i32, mult: i32) {
    match count {
        90 => {
            let p1 = mult * direct[1];
            let p2 = -mult * direct[0];
            direct[0] = p1;
            direct[1] = p2;
        }
        270 => {
            let p1 = -mult * direct[1];
            let p2 = mult * direct[0];
            direct[0] = p1;
            direct[1] = p2;
        }
        180 => {
            let p1 = -direct[0];
            let p2 = -direct[1];
            direct[0] = p1;
            direct[1] = p2;
        }
        _ => {}
    }
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
