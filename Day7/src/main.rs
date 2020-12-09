use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// TODO: Refactor code to be more readable.
fn main() {
    let declarations = read_input();

    println!("{}", part_2_solution(declarations));
}

fn part_1_solution(input_lines: Vec<String>) -> i32 {
    let mut colors_map = HashMap::new();
    let re = Regex::new(r"(\w*\s\w*)(\sbags)?\s(contain)((\w|\s|,)*)").unwrap();

    for line in input_lines {
        for cap in re.captures_iter(&line) {
            let captured_key = cap[1].to_string();
            let captured_value = cap[4].to_string();
            let split_arr = captured_value.split(",");
            let re2 = Regex::new(r"(\d+)\s(\w*\s\w*)\s(bags*)").unwrap();

            let mut colors = Vec::new();
            for item in split_arr {
                for cap2 in re2.captures_iter(item) {
                    colors.push(cap2[2].to_string());
                }
            }
            colors_map.insert(captured_key, colors);
        }
    }

    let mut res = 0;

    for k in colors_map.keys() {
        if rec(&k, &colors_map) {
            res += 1;
        }
    }

    res
}

fn part_2_solution(input_lines: Vec<String>) -> i32 {
    let mut colors_map = HashMap::new();
    let re = Regex::new(r"(\w*\s\w*)(\sbags)?\s(contain)((\w|\s|,)*)").unwrap();

    for line in input_lines {
        for cap in re.captures_iter(&line) {
            let captured_key = cap[1].to_string();
            let captured_value = cap[4].to_string();
            let split_arr = captured_value.split(",");
            let re2 = Regex::new(r"(\d+)\s(\w*\s\w*)\s(bags*)").unwrap();

            let mut colors = Vec::new();
            for item in split_arr {
                for cap2 in re2.captures_iter(item) {
                    let num_of_bags = cap2[1].trim().parse::<i32>().unwrap();
                    colors.push((cap2[2].to_string(), num_of_bags));
                }
            }
            colors_map.insert(captured_key, colors);
        }
    }

    let mut res = 0;

    rec2(&"shiny gold".to_string(), &colors_map, 1)
}

fn rec2(k: &String, colors_map: &HashMap<String, Vec<(String, i32)>>, mult: i32) -> i32 {
    let mut res = 0;
    let colors = colors_map.get(k);

    if colors == None {
        return 0;
    }

    let colors = colors.unwrap();

    for item in colors {
        res += item.1 * mult;
        res += rec2(&item.0, &colors_map, item.1 * mult);
    }

    res
}

fn rec(k: &String, colors_map: &HashMap<String, Vec<String>>) -> bool {
    if k == "shiny gold" {
        return true;
    }

    let value = colors_map.get(k);

    let mut res = false;

    if value != None {
        let value = value.unwrap();
        for item in value {
            res = res | rec(item, &colors_map);
        }
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
