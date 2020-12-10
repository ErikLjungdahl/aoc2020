use std::fs::*;

pub fn main() -> usize {
    let input = read_to_string("inputs/input6.txt").expect("Reading file failed");
    let groups = input.split("\n\n").map(|group| {
        group
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<Vec<char>>()
    });
    let mut sum = 0;
    for mut grouped in groups {
        grouped.sort();
        grouped.dedup();
        sum += grouped.len();
    }
    sum
}
