use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut input_vec = read_input();
    let n = input_vec.len();

    input_vec.sort();

    for i in 0..n {
        let mut j = i + 1;
        let mut k = n - 1;

        while j < k {
            let sum = input_vec[i] + input_vec[j] + input_vec[k];
            if sum == 2020 {
                println!("{}", input_vec[i] * input_vec[j] * input_vec[k]);
                break;
            } else if sum > 2020 {
                k -= 1;
            } else {
                j += 1;
            }
        }
    }
}

fn read_input() -> Vec<i32> {
    let mut res = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(number) = line {
                res.push(number.parse::<i32>().unwrap());
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
