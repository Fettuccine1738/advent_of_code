#![allow(dead_code, unused_imports)]
use std::{
    fs::{self, read},
    io,
};

const SAMPLE: &str = "sample.txt";
const INPUT: &str = "input.txt";

fn read_to_table() -> Vec<Vec<bool>> {
    let input = fs::read_to_string(INPUT).expect("Could not read file ");
    println!("{}", input);
    let mut table: Vec<Vec<bool>> = Vec::new();

    let lines = input.trim().lines();
    for mut line in lines {
        line = line.trim();

        let mut tt: Vec<bool> = Vec::new();
        // let mut dt: Vec<bool> = Vec::new();

        if line.is_empty() {
            continue;
        }

        for c in line.chars() {
            if c == '@' {
                tt.push(true);
            } else {
                tt.push(false);
            }
        }

        table.push(tt);
    }

    table
}

fn solution_one() -> u32 {
    let table = read_to_table();
    let mut dbg_table: Vec<Vec<bool>> = table.clone();
    for t in &table {
        println!("Before {:?} ", t);
    }

    let total = empty_neighbours(&table, &mut dbg_table);
    total
}

fn empty_neighbours(t: &Vec<Vec<bool>>, next: &mut Vec<Vec<bool>>) -> u32 {
    let mut count: u32 = 0;
    let directions: [[i32; 2]; 8] = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    let m: usize = t.len();
    let n: usize = t[0].len();

    for r in 0..m {
        for c in 0..n {
            let mut empty: i32 = 0;

            // cell is empty .
            if !t[r][c] {
                continue;
            }

            // we are only concerned with cells that are marked false (have a roll of paper there.)
            for d in directions {
                let new_row = r as i32 + d[0];
                let new_col = c as i32 + d[1];

                if new_row >= 0 && new_row < m as i32 && new_col >= 0 && new_col < n as i32 {
                    let row = new_row as usize;
                    let col = new_col as usize;
                    empty = if t[row][col] { empty + 1 } else { empty };
                }
            }

            if empty < 4 {
                count += 1;
                next[r][c] = false;
            }
        }
    }
    count
}

fn solution_two() -> u32 {
    let mut current: Vec<Vec<bool>> = read_to_table();
    let mut next: Vec<Vec<bool>> = current.clone();
    let mut total: u32 = 0u32;

    loop {
        total += empty_neighbours(&current, &mut next);

        if next == current {
            break;
        }
        current = next.clone();
    }
    total
}

fn main() {
    let res = solution_two();
    println!("solution = {res}");
}
