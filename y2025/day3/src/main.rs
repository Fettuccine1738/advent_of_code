#![allow(dead_code, unused_imports)]
use std::{fs, io};

const SAMPLE: &str = "sample.txt";
const INPUT: &str = "input.txt";

pub fn solution_one() -> u64 {
    let input = fs::read_to_string(SAMPLE).expect("Could not read file ");
    println!("{}", input);
    let mut total = 0u64;

    let lines = input.trim().lines();
    // let mut b: Battery = Battery(0, 0);

    let mut count = 0;
    for mut line in lines {
        line = line.trim();
        total += select_battery(line);
        count += 1;
    }

    println!("banks = {count}");
    total
}

// b: &mut Battery,
fn select_battery(s: &str) -> u64 {
    let mut tens: u64 = 0; // s[..1].parse::<u64>().unwrap();
    let mut unit: u64 = 0; // s[1..2].parse::<u64>().unwrap();
    // let mut changed: bool = unit > tens;
    let mut idx: usize = 0;

    for c in s.chars() {
        let num = c.to_digit(10).unwrap() as u64;

        // since we are looking for a tens digit, we have to compare to the
        // previous value of unit. so we can always have a unit digit.
        tens = tens.max(unit);
        unit = unit.max(num);

        // if unit finds a larger number
        // only swap if there are digits to the right of 'unit'
        if tens < unit && idx < s.len() - 1 {
            tens = unit;
            unit = 0;
        }

        idx += 1;
    }

    println!("chose tens = {tens} and unit = {unit} from {s}");
    tens * 10 + unit
}

pub fn solution_two() -> u128 {
    let input = fs::read_to_string(INPUT).expect("Could not read file ");
    let mut total = 0u128;

    let lines = input.trim().lines();

    let mut count = 0;
    for mut line in lines {
        line = line.trim();
        total += select_twelve(line) as u128;
        count += 1;
    }

    println!("banks = {count}");
    total
}

fn select_twelve(s: &str) -> u64 {
    let mut total = 0u64; 
    let mut buffer: String = String::new();
    let max_len: usize = 12; 
    let mut index: usize = 0;
    let mut right_bound: usize = 10;

    while buffer.len() < max_len {
        let (fir, sec, idx) = select(s, index, right_bound);
        buffer.push(fir);
        buffer.push(sec);
        // reduce search window. start from the last 'consumed' index.
        index += idx + 1; 
        right_bound = if right_bound >= 2 { right_bound - 2} else {
             0 // value not needed.  
        };
    } 

    println!("{s} : top 12 = {buffer}");

    // buffer is (54323)
    for (char_idx, c) in buffer.char_indices() {
        let mut as_num = c.to_digit(10).unwrap() as u64;
        let raised = (max_len - 1 - char_idx) as u32;
        as_num = as_num * 10u64.pow(raised);
        total += as_num;
    }

    total
}

// take a string slice and a right bound (number of digits) to ignore 
// return the next 2 largest number and the index which to start from.  
fn select(slice: &str, l: usize, r: usize) -> (char, char, usize) {
    let mut tens = '0';
    let mut unit = '0'; 
    // let mut changed: bool = unit > tens;
    let mut idx: usize = 0;
    let current = &slice[l..];
    let bound = current.len() - r; 

    for (i, c) in current.char_indices() {
        if i >= bound {
            break;
        }

        tens = tens.max(unit);
        // unit = unit.max(c);
        if unit < c {
            unit = c;
            idx = i; // record the last time unit changed, that's the index to start the next search from.
        }

        if tens < unit && i < bound - 1 {
            tens = unit;
            unit = '0';
        }
    }

    // println!("chose tens = {tens} and unit = {unit} from {s}");
    (tens, unit, idx)
}

fn main() {
    let f: char = '8'.max('9');
    println!("max is {f}");

    let _ = select_twelve("818181911112111");
    let s = solution_two();
    println!("solution_one {s}");
}
