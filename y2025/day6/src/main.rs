#![allow(dead_code, unused_imports)]
use std::collections::HashMap;
use std::{collections, fs, io};

// tired of rewriting name during debugging.
const SN: &str = "sample.txt";
const IN: &str = "input.txt";

pub fn read_input() -> (Vec<Vec<u32>>, Vec<bool>) {
    let input = fs::read_to_string(SN).expect("Could not read file ");
    println!("intput \n{}", input);

    let mut matrix: Vec<Vec<u32>> = Vec::new();
    // represent + as true and * as false.. save memory and extra checks
    let mut signs: Vec<bool> = Vec::new();

    let lines = input.trim().lines();

    for mut line in lines {
        line = line.trim();
        let mut v: Vec<u32> = Vec::new();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("*") || line.starts_with("+") {
            for s in line.split_whitespace() {
                signs.push(if s.starts_with("+") { true } else { false });
            }
        } else {
            for s in line.split_whitespace() {
                let n = s.parse::<u32>().unwrap();
                v.push(n);
            }
            matrix.push(v);
        }
    }

    assert_eq!(signs.len(), matrix[0].len());
    (matrix, signs)
}

fn part_one() -> u64 {
    let (num, signs) = read_input();
    let rows = num.len();
    let cols = num[0].len();
    let mut total: u64 = 0;

    for i in 0..cols {
        let mut sum: u64 = num[0][i] as u64;
        let sign = signs[i];

        for j in 1..rows {
            let m = num[j][i] as u64;

            if sign {
                // println!("{sum} + {m} = {}", sum + m);
                sum += m;
            } else {
                // println!("{sum} + {m} = {}", sum * m);
                sum *= m;
            }
        }

        total += sum;
    }

    total
}

fn part_two() -> u64 {
    let input = fs::read_to_string(SN).expect("Could not read file ");
    println!("{input}");
    let mut lines: Vec<String> = input.trim().lines().map(|ln| ln.to_string()).collect();

    let mut total = 0u64;
    let mut temp: Vec<String> = Vec::new();
    let mut sign = lines.pop().unwrap();

    loop {
        if consumed(&lines) {
            break;
        } 

        temp.push(extract_digit(&mut lines));

        if is_empty_columns(&lines) || consumed(&lines) { 
            sign = sign.trim().to_string();
            let ch = sign.remove(sign.len() - 1);
            let op = ch == '+';
            let mut current = if op { 0u64 } else { 1u64 };

        // this loop consumes a colon until the signage is consumed.
        // e.g 1 2 3
        //     4 5 6
        //     +      == > this results in '36', '25', and '14+' being consumed, strip + and do the operation.
        // loop {

        // }
            for s in &temp {
                current = if op { current + s.parse::<u64>().unwrap() } else {
                    current * s.parse::<u64>().unwrap() };
            }
            total += current;
            temp.clear();

            // consume the column of whitespace that separates the next numberlines.
            if !consumed(&lines) {
                for line in &mut lines {
                    let _ = line.remove(line.len() - 1);
                }
            }
        }
    }
    total
}

fn is_empty_columns(lines: &Vec<String>) -> bool {
    for line in lines {
        if !line.ends_with(" ") {
            return false;
        }
    }
    true
}

fn consumed(lines: &Vec<String>) -> bool {
    for line in lines {
        if !line.is_empty() {
            return false;
        }
    }
    true
}

fn extract_digit(row: &mut Vec<String>) -> String {
    let mut buf = String::new();

    for r in row {
        if !r.is_empty() {
            if r.chars().nth(r.len() - 1) != Some(' ')  {
                buf.push(r.remove(r.len() - 1));
            } else {
                r.remove(r.len() - 1);
            }
        }
    }
    buf
}

fn print_all(m: &Vec<Vec<u32>>) {
    for i in m {
        for j in i {
            print!("{j}\t");
        }
        println!("");
    }
}

fn main() {
    // let s = "9 8";
    let r = part_two();
    println!("r = {r} : ");
}



