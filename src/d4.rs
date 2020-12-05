use std::fs::*;

pub fn main() -> usize {
    read_to_string("inputs/input4.txt")
        .expect("Reading file failed")
        .split("\n\n")
        .map(|entry| {
            contains_all(
                entry
                    .split_whitespace()
                    .map(|field| field.split(":").collect::<Vec<&str>>()[0])
                    .collect(),
            )
        })
        .sum()
}

fn contains_all(x: Vec<&str>) -> usize {
    (x.contains(&"byr")
        && x.contains(&"iyr")
        && x.contains(&"eyr")
        && x.contains(&"hgt")
        && x.contains(&"hcl")
        && x.contains(&"ecl")
        && x.contains(&"pid")) as usize
}
