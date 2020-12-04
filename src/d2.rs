use std::fs::*;
use std::iter::*;

struct Pwd<'a> {
    min: usize,
    max: usize,
    char: char,
    pwd: &'a str
}

pub fn main()->i32 {
    read_to_string("inputs/input2.txt")
        .expect("Something went wrong reading the file")
        [..]
        .lines()
        .map(|s| parse_line(s)) 
        .map(count2).sum()
}
fn parse_line(line : &str ) -> Pwd {
    let mut res : Vec<&str> = line.split(|c| c == '-' || c == ':' || c == ' ')
                            .filter(|s| s != &"")
                            .collect();

    let pwd  = res.pop().expect("Missing item");
    let char = res.pop().expect("Missing item").parse::<char>().unwrap();
    let max  = res.pop().expect("Missing item").parse::<usize>().unwrap();
    let min  = res.pop().expect("Missing item").parse::<usize>().unwrap();
    
    return Pwd { min, max, char, pwd};
    
}

fn count(Pwd {min, max, char, pwd} : Pwd)-> i32 {
    let length = pwd.chars().filter(|c| c == &char).collect::<Vec<char>>().len();
    if min <= length && length <= max {
        return 1;
    } else {
        return 0
    }
}

fn count2(Pwd {min, max, char, pwd} : Pwd)-> i32 {
    let pwd = pwd.chars().collect::<Vec<char>>();
    if (char == pwd[min-1]) ^ (char == pwd[max-1]) {
        return 1;
    } else {
        return 0
    }
}

