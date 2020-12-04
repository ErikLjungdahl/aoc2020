use std::fs::*;

pub fn main() -> usize {
    let mat: Vec<Vec<char>> = read_to_string("inputs/input3.txt")
        .expect("Reading file failed")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let tests: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    tests
        .iter()
        .map(|(right, down)| count_trees(&mat, right, down))
        .product()
}

fn count_trees(mat: &Vec<Vec<char>>, right: &usize, down: &usize) -> usize {
    let mut r: usize = 0;
    let mut c: usize = 0;
    let mut sum: usize = 0;
    loop {
        if mat[r][c] == '#' {
            sum += 1;
        }
        c = (c + right) % mat[r].len();
        r += down;
        if r >= mat.len() {
            break;
        }
    }
    sum
}
