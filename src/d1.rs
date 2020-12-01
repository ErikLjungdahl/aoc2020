use std::fs::*;

pub fn main()->Option<i32> {
    let mut ints = read_to_string("inputs/input1.txt")
        .expect("Something went wrong reading the file")
        [..]
        .split("\n")
        .map(|s| s.parse::<i32>().expect("Bad Parse"))
        .collect::<Vec<i32>>()
        ;
    ints.sort();
    return p2(&ints);
}

fn p2(ints : &Vec<i32>)-> Option<i32> {
    for x in ints {
        for y in ints {
            for z in ints {
                if x+y+z == 2020 {
                    return Some(x*y*z);
                }
            }
        }
    }
    return None;
}

fn p1(ints : Vec<i32>)->Option<i32> {
    let mut l = 0;
    let mut r = ints.len()-1;
    loop {
        if l >= r {
            return None;
        }
        if ints[l] + ints[r] == 2020 {
            return Some (ints[l] * ints[r]);
        } else if ints[l] + ints[r] < 2020 {
            l += 1;
        } else {
            r-= 1;
        }

    }
}