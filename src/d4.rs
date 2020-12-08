use std::fs::*;

pub fn main() -> usize {
    read_to_string("inputs/input4.txt")
        .expect("Reading file failed")
        .split("\n\n")
        .map(|entry| {
            (entry
                .split_whitespace()
                .all(|field| check(field.split(":").collect::<Vec<&str>>()))
                && contains_all(
                    entry
                        .split_whitespace()
                        .map(|field| field.split(":").collect::<Vec<&str>>()[0])
                        .collect(),
                )) as usize
        })
        .sum()
}

fn check(x: Vec<&str>) -> bool {
    let field = x[0];
    let value = x[1];
    match field {
        "byr" => {
            let v = value.parse::<usize>().expect("NaN");
            return 1920 <= v && v <= 2002;
        }
        "iyr" => {
            let v = value.parse::<usize>().expect("NaN");
            return 2010 <= v && v <= 2020;
        }
        "eyr" => {
            let v = value.parse::<usize>().expect("NaN");
            return 2020 <= v && v <= 2030;
        }
        "hgt" => {
            let mut v = value.chars().collect::<Vec<char>>();
            v.pop();
            match v.pop().expect("Empty Value") {
                'i' => {
                    let i = v.iter().collect::<String>().parse::<usize>().expect("NaN");
                    return 59 <= i && i <= 76;
                }
                'c' => {
                    let i = v.iter().collect::<String>().parse::<usize>().expect("NaN");
                    return 150 <= i && i <= 193;
                }
                _ => false,
            }
        }
        "hcl" => {
            let (head, tail) = value.split_at(1);
            return tail.len() == 6
                && head == "#"
                && tail
                    .chars()
                    .all(|c| ('0' <= c && c <= '9') || ('a' <= c && c <= 'f'));
        }
        "ecl" => {
            return value == "amb"
                || value == "blu"
                || value == "brn"
                || value == "gry"
                || value == "grn"
                || value == "hzl"
                || value == "oth"
        }
        "pid" => return value.chars().count() == 9,
        "cid" => return true,
        _ => return false,
    }
}

fn contains_all(x: Vec<&str>) -> bool {
    x.contains(&"byr")
        && x.contains(&"iyr")
        && x.contains(&"eyr")
        && x.contains(&"hgt")
        && x.contains(&"hcl")
        && x.contains(&"ecl")
        && x.contains(&"pid")
}
