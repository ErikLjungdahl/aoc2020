use std::fs::*;

pub fn main() -> usize {
    let mut a = read_to_string("inputs/input5.txt")
        .expect("Reading file failed")
        .lines()
        .map(|l| seat_id(row_col(l)))
        .collect::<Vec<usize>>();
    a.sort();
    find_missing(a)
}

fn find_missing(ids: Vec<usize>) -> usize {
    let mut start = ids[0];
    for id in ids {
        if id != start {
            return start;
        }
        start += 1;
    }
    return 0;
}

fn seat_id((row, col): (usize, usize)) -> usize {
    return row * 8 + col;
}

fn row_col(line: &str) -> (usize, usize) {
    let mut i = 64;
    let (row, col) = line.split_at(7);
    let mut r_sum = 0;
    for c in row.chars() {
        if c == 'B' {
            r_sum += i;
        }
        i = i / 2;
    }
    i = 4;
    let mut c_sum = 0;
    for c in col.chars() {
        if c == 'R' {
            c_sum += i;
        }
        i = i / 2;
    }
    return (r_sum, c_sum);
}
