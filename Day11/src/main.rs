use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut board = read_input();

    println!("{:?}", part_2_solution(&mut board));
}

fn part_1_solution(board: &mut Vec<String>) -> i32 {
    let neigh = [
        [1, 0],
        [0, 1],
        [-1, 0],
        [0, -1],
        [-1, -1],
        [1, -1],
        [-1, 1],
        [1, 1],
    ];

    let mut boardCopy = vec![];
    let mut res = 0;
    let mut change = false;

    for i in 0..board.len() {
        let mut row = "".to_string();
        for j in 0..board[0].len() {
            let mut countOccupied = 0;
            for k in 0..neigh.len() {
                //println!("{:?}", board[i].as_bytes()[j] as char);
                let new_i = i as isize + neigh[k][0];
                let new_j = j as isize + neigh[k][1];
                if new_i < 0
                    || new_j < 0
                    || new_i >= board.len() as isize
                    || new_j >= board[0].len() as isize
                {
                    continue;
                }
                let new_i = new_i as usize;
                let new_j = new_j as usize;
                if board[new_i].as_bytes()[new_j] as char == '#' {
                    countOccupied += 1;
                }
            }

            let c = board[i].as_bytes()[j] as char;

            if c == '#' && countOccupied >= 4 {
                row.push('L');
                res += 1;
                change = true;
            } else if c == 'L' && countOccupied == 0 {
                row.push('#');
                change = true;
            } else {
                if c == '#' {
                    res += 1;
                }
                row.push(c);
            }
        }
        boardCopy.push(row.clone());
        row.clear();
    }

    if change {
        return part_1_solution(&mut boardCopy);
    }

    res
}

fn part_2_solution(board: &mut Vec<String>) -> i32 {
    let neigh = [
        [1, 0],
        [0, 1],
        [-1, 0],
        [0, -1],
        [-1, -1],
        [1, -1],
        [-1, 1],
        [1, 1],
    ];

    let mut boardCopy = vec![];
    let mut res = 0;
    let mut change = false;

    for i in 0..board.len() {
        let mut row = "".to_string();
        for j in 0..board[0].len() {
            let mut countOccupied = 0;
            for k in 0..neigh.len() {
                //println!("{:?}", board[i].as_bytes()[j] as char);
                let mut new_i = i as isize + neigh[k][0];
                let mut new_j = j as isize + neigh[k][1];

                while new_i >= 0
                    && new_j >= 0
                    && new_i < board.len() as isize
                    && new_j < board[0].len() as isize
                {
                    let c = board[new_i as usize].as_bytes()[new_j as usize] as char;
                    if c == '#' || c == 'L' {
                        if c == '#' {
                            countOccupied += 1;
                        }
                        break;
                    }
                    new_i += neigh[k][0];
                    new_j += neigh[k][1];
                }
            }

            let c = board[i].as_bytes()[j] as char;

            if c == '#' && countOccupied >= 5 {
                row.push('L');
                res += 1;
                change = true;
            } else if c == 'L' && countOccupied == 0 {
                row.push('#');
                change = true;
            } else {
                if c == '#' {
                    res += 1;
                }
                row.push(c);
            }
        }
        boardCopy.push(row.clone());
        row.clear();
    }

    if change {
        return part_2_solution(&mut boardCopy);
    }

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
