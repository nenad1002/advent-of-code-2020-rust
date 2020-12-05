use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let matrix = read_input();
    let row_len = matrix[0].len();

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut res: i64 = 1;
    let mut number_of_trees: i64;

    for slope in slopes.iter() {
        number_of_trees = 0;
        let mut pos = (0, 0);
        println!("{:?}", slope.0);

        while pos.0 < matrix.len() {
            pos.0 += slope.1;
            pos.1 += slope.0;

            if pos.1 >= row_len {
                pos.1 %= row_len;
            }

            if pos.0 >= matrix.len() {
                break;
            }

            let row = &matrix[pos.0];
            let col = pos.1;
            let square = row.clone().chars().nth(col).unwrap();
            if square == '#' {
                number_of_trees += 1;
            }
        }

        res *= number_of_trees;
        println!("{}", number_of_trees);
    }

    println!("{}", res);
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
