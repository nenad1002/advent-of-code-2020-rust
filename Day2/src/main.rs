use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_vec = read_input();

    println!("{}", part1_solution(&input_vec));

    println!("{}", part2_solution(&input_vec));
}

fn part1_solution(input_vec: &Vec<String>) -> i32 {
    let mut valid_passwords_count = 0;

    for line in input_vec {
        let line_items: Vec<String> = line.split(' ').map(String::from).collect();
        let (low, high) = get_range(&line_items[0]);
        let letter = line_items[1].chars().next().unwrap();

        let mut letter_count = 0;

        for c in line_items[2].chars() {
            if c == letter {
                letter_count += 1;
            }
        }

        if letter_count >= low && letter_count <= high {
            valid_passwords_count += 1;
        }
    }

    valid_passwords_count
}

fn part2_solution(input_vec: &Vec<String>) -> i32 {
    let mut valid_passwords_count = 0;

    for line in input_vec {
        let line_items: Vec<String> = line.split(' ').map(String::from).collect();
        let (low, high) = get_range(&line_items[0]);
        let letter = line_items[1].chars().next().unwrap();

        let is_left_valid: bool = is_char_valid(&line_items[2], letter, low as usize - 1);
        let is_right_valid: bool = is_char_valid(&line_items[2], letter, high as usize - 1);

        if is_left_valid != is_right_valid {
            valid_passwords_count += 1;
        }
    }

    valid_passwords_count
}

fn is_char_valid(letters: &String, target_letter: char, index: usize) -> bool {
    let letter_at_index = letters.chars().nth(index);

    if letter_at_index != None && letter_at_index.unwrap() == target_letter {
        return true;
    }
    false
}

fn get_range(item: &String) -> (i32, i32) {
    let range_nums: Vec<String> = item.split("-").map(String::from).collect();
    (
        range_nums[0].trim().clone().parse::<i32>().unwrap(),
        range_nums[1].trim().clone().parse::<i32>().unwrap(),
    )
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
