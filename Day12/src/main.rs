use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut ins = read_input();
    println!("{:?}", part_1_solution(&mut ins));
}

fn part_1_solution(instructions: &Vec<String>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut direct = [1, 0]; // [0, -1] [-1, 0] [0, 1]
    for ins in instructions {
        let command = ins.chars().nth(0).unwrap();
        let count = &ins[1..].parse::<i32>().unwrap();

        match command {
            'F' => {
                x += direct[0] * count;
                y += direct[1] * count;
            }
            'N' => y += count,
            'S' => y -= count,
            'W' => x -= count,
            'E' => x += count,
            'R' => {
                rotate(&mut direct, count.clone(), 1);
            }
            'L' => {
                rotate(&mut direct, count.clone(), -1);
            }
            _ => {}
        }
    }
    i32::abs(x) + i32::abs(y)
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
