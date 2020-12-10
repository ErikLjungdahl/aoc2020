use array_tool::vec::Intersect;
use std::fs::*;

pub fn main() -> usize {
    let input = read_to_string("inputs/input6.txt").expect("Reading file failed");
    p2(input)
}

fn p1(input: String) -> usize {
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
fn p2(input: String) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .fold_first(|p1, p2| p1.intersect(p2))
                .expect("0 persons?")
                .len()
        })
        .sum()
}
