use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let matrix = read_input();
    let row_len = matrix[0].len();

    let mut pos = (0, 0);

    let mut number_of_trees = 0;

    while pos.0 < matrix.len() {
        let row = &matrix[pos.0];
        let col = pos.1;
        let square = row.clone().chars().nth(col).unwrap();
        if square == '#' {
            number_of_trees += 1;
        }
        pos.0 += 1;
        pos.1 += 3;

        if pos.1 >= row_len {
            pos.1 %= row_len;
        }
    }

    println!("{}", number_of_trees);
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
