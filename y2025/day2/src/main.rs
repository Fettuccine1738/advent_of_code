#![allow(dead_code, unused_imports, unused_variables, unused_doc_comments)]
use std::{fs, io};

const SAMPLE: &str = "sample.txt";
const INPUT: &str = "input.txt";

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

fn solution<F>(path: &str, f: F) -> u64
where
    F: Fn(u64) -> u64,
{
    let contents = read_file(path).expect("Could not read file");
    println!("{contents}");
    let mut result = 0u64;

    let tokens = contents.trim().split(",");

    for mut token in tokens {
        token = token.trim();
        println!("{token}");
        let mut parts = token.split("-");
        let first = match parts.next() {
            Some(x) => x.trim().parse::<u64>().unwrap(),
            None => {
                eprintln!("Expected to finda number on {token}");
                return 0u64;
            }
        };

        let second = match parts.next() {
            Some(x) => x.trim().parse::<u64>().unwrap(),
            None => {
                eprintln!("Expected to finda number on {token}");
                return 0u64;
            }
        };

        inspect_ranges(first, second, &mut result, &f);
    }
    result
}

// fn solution_two(path: &str) -> u64 {
// let contents = read_file(path).expect("Could not read file");
// let mut result = 0u64;
//
// let tokens = contents.trim().split(",");
//
// for mut token in tokens {
// token = token.trim();
// let mut parts = token.split("-");
// let first = match parts.next() {
// Some(x) => x.trim().parse::<u64>().unwrap(),
// None => {
// eprintln!("Expected to finda number on {token}");
// return 0u64;
// }
// };
//
// let second = match parts.next() {
// Some(x) => x.trim().parse::<u64>().unwrap(),
// None => {
// eprintln!("Expected to finda number on {token}");
// return 0u64;
// }
// };
//
// inspect_ranges(first, second, &mut result, |x| validate_two(x));
// }
// result
// }

fn inspect_ranges<F>(start: u64, end: u64, res: &mut u64, f: F)
where
    F: Fn(u64) -> u64,
{
    for n in start..=end {
        *res += f(n);
    }
}

fn validate(num: u64) -> u64 {
    let s = num.to_string();
    if s.len() % 2 != 0 {
        0
    } else {
        let mid = s.len() / 2;
        let result: bool = &(s[..mid]) == &(s[mid..]);

        if result { num } else { 0 }
    }
}

fn validate_two(num: u64) -> u64 {
    let s = num.to_string();
    let half = s.len() / 2;
    let bound = s.len();
    let mut s_cmp: String = String::new();

    for left in 0..half {
        /// NOTE: if we cannot find a sequence that divides the 'number' evenly
        /// then the number itself cannot possibly be made up of a sequence of this length.
        /// e.g '2121212118' no point checking if a sequence of 4 digit appears N times. 10 % 4 == 2.
        /// It will never reappear enough times to be the only valid sequence.
        let mut complete_match = true;
        s_cmp.push_str(&(s[0..left + 1]));

        if bound % s_cmp.len() != 0 {
            s_cmp.clear();
            continue;
        }

        let mut r_start = s_cmp.len();

        while r_start < bound {
            let r_end = r_start + s_cmp.len();
            let p2 = &(s[r_start..r_end]);

            if &s_cmp[..] != p2 {
                complete_match = false;
                break;
            }

            r_start = r_end;
        }

        if complete_match {
            println!("sequence found {s_cmp} for num {num}");
            return num;
        } else {
            s_cmp.clear();
        }
    }
    0
}

fn main() {
    // let s: String  = "Hello, world!".to_string();
    // for left in 0..s.len() {
    //     let st: &str = &(s[0..left + 1]);
    //     println!("{st}");
    // }

    // let xx = validate_two(2121212118);
    let s = solution(INPUT, |x| validate_two(x));
    println!("{s}");
}
