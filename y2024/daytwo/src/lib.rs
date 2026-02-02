// use std::error::Error;
use std::fs;
use std::io::{self};

const MAX_SAFE: i32 = 1;
//
fn read_file(path: &str) -> io::Result<String> {
    // fs::r-t_s returns Result<String, io::Erro>
    let contents = fs::read_to_string(path)?; // content owns the full file as a String
    Ok(contents)
}

pub fn solution_one(path: &str) -> u32 {
    let input = match read_file(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file {}", e);
            return 0;
        }
    };

    let mut valid_levels = 0;
    let mut levels: Vec<u32> = Vec::new();

    for line in input.lines() {
        for num in line.split_whitespace() {
            let num: u32 = num.trim().parse().expect("Not a valid number");
            levels.push(num);
        }

        if is_valid_level(&levels) {
            valid_levels += 1;
            println!("Valid \t {:?}", levels);
        }
        levels.clear();
    }
    valid_levels
}

fn is_valid_diff(a: u32, b: u32, c: u32) -> bool {
    if a.abs_diff(b) > 0 && a.abs_diff(b) <= 3 && b.abs_diff(c) > 0 && b.abs_diff(c) <= 3 {
        return true;
    }
    false
}

fn is_valid_level(vector: &Vec<u32>) -> bool {
    let len = vector.len() - 2; // we will compare 3 values at a time.
    for i in 0..len {
        let left = vector[i];
        let mid = vector[i + 1];
        let right = vector[i + 2];

        if !is_valid_diff(left, mid, right) {
            return false;
        }

        if vector[i] > vector[i + 1] && vector[i + 1] > vector[i + 2] {
            continue;
        } else if vector[i] < vector[i + 1] && vector[i + 1] < vector[i + 2] {
            continue;
        }
        return false;
    }
    true
}

pub fn solution_two(path: &str) -> u32 {
    let mut valid_levels = 0;

    let input = match read_file(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file {}", e);
            return 0;
        }
    };

    let mut levels: Vec<u32> = Vec::new();

    for line in input.lines() {
        for num in line.split_whitespace() {
            let num: u32 = num.trim().parse().expect("Not a valid number");
            levels.push(num);
        }
        if is_valid_level(&levels) {
            valid_levels += 1;
            println!("Valid \t {:?}", levels);
        }
        else if is_valid_level2(&mut levels) {
            valid_levels += 1;
            println!("Valid \t {:?}", levels);
        }
        levels.clear();
    }
    valid_levels
}

fn is_unsafe_level(a: u32, b: u32, c: u32) -> bool {
    let mut desc = false; // ordered in descending
    let mut asc = false; // ascending order
    if a > b && b > c {
        desc = true;
    } else if a < b && b < c {
        asc = true;
    }
    if (asc || desc)
        && a.abs_diff(b) > 0
        && a.abs_diff(b) <= 3
        && b.abs_diff(c) > 0
        && b.abs_diff(c) <= 3
    {
        return true;
    }
    false
}

fn is_valid_level2(vector: &mut Vec<u32>) -> bool {
    let max = vector.len() - 2;
    let mut unsafe_lvl: i32 = 0;

    for i in 0..max {
        let left = vector[i];
        let mid = vector[i + 1];
        let right = vector[i + 2];

        if unsafe_lvl <= MAX_SAFE {
            // let check_lvl = is_unsafe_level(left, mid, right);
            if is_unsafe_level(left, mid, right) == false {
                // find culprit
            }
            unsafe_lvl += 1;
        } else {
            return false;
        }
    }
    true
}

fn find_culprit(vector: &mut Vec<u32>, index: usize) -> usize {
    let a = vector[index];
    let b = vector[index + 1];
    let c = vector[index + 2];

    let mut no_culprit = false;
    let mut error: char = 'x';
    let checker = |x: u32, y: u32| -> bool { 
                let diff = x.abs_diff(y);
                diff > 0 && diff < 4
            };

    if a > b {
        if b > c {
            no_culprit = true;
        }
        else { // a > c or // < c
            let check = checker(a, b);
            let check2 = checker(a, c);
            if (!check) {
                // b is faulty
            } 
            

        }
    } else if a < b {
        if b < c {
            no_culprit = true;
        }
    }

    let mut diff_a = 0;
    let mut diff_b = 0;
    // a, b and c is ordered
    if no_culprit {
        // a and b are valid
        if a.abs_diff(b) < 0 && a.abs_diff(b) <= 3 {
            if 

        }
        else {

        }
    }
    0
}
