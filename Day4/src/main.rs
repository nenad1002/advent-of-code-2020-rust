use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use substring::Substring;

fn main() {
    let passports = read_input();

    println!("{}", part_2_solution(passports));
}

fn check_byr(s: &str) -> bool {
    let num = s.trim().parse::<i32>();

    if num.is_err() {
        return false;
    }

    let num = num.unwrap();

    num >= 1920 && num <= 2002
}

fn check_iyr(s: &str) -> bool {
    let num = s.trim().parse::<i32>();

    if num.is_err() {
        return false;
    }

    let num = num.unwrap();

    num >= 2010 && num <= 2020
}

fn check_eyr(s: &str) -> bool {
    let num = s.trim().parse::<i32>();

    if num.is_err() {
        return false;
    }

    let num = num.unwrap();

    num >= 2020 && num <= 2030
}

fn check_hgt(s: &str) -> bool {
    let metric = s.substring(3, 5);

    if metric == "cm" {
        let num = s.substring(0, 3).trim().parse::<i32>();

        if num.is_err() {
            return false;
        }

        let num = num.unwrap();

        return num >= 150 && num <= 193;
    }

    let metric = s.substring(2, 4);

    if metric == "in" {
        let num = s.substring(0, 2).trim().parse::<i32>();

        if num.is_err() {
            return false;
        }

        let num = num.unwrap();

        return num >= 59 && num <= 76;
    }

    false
}

fn check_hcl(s: &str) -> bool {
    let first_c = s.chars().nth(0).unwrap();

    if first_c != '#' || s.len() != 7 {
        return false;
    }

    for i in 1..7 {
        let c = s.chars().nth(i).unwrap();
        if !(c >= '0' && c <= '9' || c >= 'a' && c <= 'f') {
            return false;
        }
    }

    true
}

fn check_ecl(s: &str) -> bool {
    match s {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn check_pid(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }

    for i in 0..9 {
        let c = s.chars().nth(i).unwrap();
        if !(c >= '0' && c <= '9') {
            return false;
        }
    }

    true
}

fn part_2_solution(passports: Vec<String>) -> i32 {
    let mut info_data = [false; 8];

    let mut count = 0;

    for line in passports {
        if line != "" {
            let line_items: Vec<String> = line.split(' ').map(String::from).collect();
            for item in line_items {
                let info: Vec<String> = item.split(':').map(String::from).collect();
                match &info[0][..] {
                    "byr" => {
                        info_data[0] = check_byr(&info[1]);
                    }
                    "iyr" => {
                        info_data[1] = check_iyr(&info[1]);
                    }
                    "eyr" => {
                        info_data[2] = check_eyr(&info[1]);
                    }
                    "hgt" => {
                        info_data[3] = check_hgt(&info[1]);
                    }
                    "hcl" => {
                        info_data[4] = check_hcl(&info[1]);
                    }
                    "ecl" => {
                        info_data[5] = check_ecl(&info[1]);
                    }
                    "pid" => {
                        info_data[6] = check_pid(&info[1]);
                    }
                    "cid" => {
                        info_data[7] = true;
                    }
                    _ => {}
                }
            }
        } else {
            println!("{:?}", info_data);
            let mut should_be_counted = true;
            for x in 0..7 {
                if info_data[x] == false {
                    should_be_counted = false;
                }
                info_data[x] = false;
            }
            info_data[7] = false;

            if should_be_counted {
                count += 1;
            }
        }
    }

    count
}

fn part_1_solution(passports: Vec<String>) -> i32 {
    let mut info_data = [false; 8];

    let mut count = 0;

    for line in passports {
        if line != "" {
            let line_items: Vec<String> = line.split(' ').map(String::from).collect();
            for item in line_items {
                let info: Vec<String> = item.split(':').map(String::from).collect();
                match &info[0][..] {
                    "byr" => {
                        info_data[0] = true;
                    }
                    "iyr" => {
                        info_data[1] = true;
                    }
                    "eyr" => {
                        info_data[2] = true;
                    }
                    "hgt" => {
                        info_data[3] = true;
                    }
                    "hcl" => {
                        info_data[4] = true;
                    }
                    "ecl" => {
                        info_data[5] = true;
                    }
                    "pid" => {
                        info_data[6] = true;
                    }
                    "cid" => {
                        info_data[7] = true;
                    }
                    _ => {}
                }
            }
        } else {
            println!("{:?}", info_data);
            let mut should_be_counted = true;
            for x in 0..7 {
                if info_data[x] == false {
                    should_be_counted = false;
                }
                info_data[x] = false;
            }
            info_data[7] = false;

            if should_be_counted {
                count += 1;
            }
        }
    }

    count
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
