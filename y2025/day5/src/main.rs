#![allow(dead_code, unused_imports)]
use std::collections::HashSet;
use std::{fs, io};

const SAMPLE: &str = "sample.txt";
const INPUT: &str = "input.txt";

pub fn read_input() -> (Vec<(u64, u64)>, Vec<u64>) {
    let input = fs::read_to_string(INPUT).expect("Could not read file ");
    // println!("{}", input);

    let lines = input.trim().lines();
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    let mut _count = 0;

    for mut line in lines {
        line = line.trim();

        if !line.is_empty() {
            if line.contains("-") {
                let mut x = line.split("-");
                let lr = match x.next() {
                    Some(n) => n.parse::<u64>().expect("could not parse"),
                    None => panic!(""),
                };

                let rr = match x.next() {
                    Some(n) => n.parse::<u64>().expect("could not parse"),
                    None => panic!(""),
                };

                ranges.push((lr, rr));
            } else {
                let num: u64 = line.parse::<u64>().expect("Parse unsuccesful");
                ids.push(num);
            }
        }
    }

    (ranges, ids)
}

fn part_one() -> u64 {
    let mut result = 0u64;
    let (range, ids) = read_input();

    for i in &ids {
        for j in &range {
            if *i >= j.0 && *i <= j.1 {
                result += 1;
                break; // do not consider other ranges.
            }
        }
    }
    result
}

fn part_two() -> u64 {
    let (mut range, _) = read_input();
    let sz = range.len();
    let mut stack: Vec<(u64, u64)> = Vec::new();

    range.sort_by(|l, r| l.0.cmp(&r.0));
    for i in &range {
        println!("{} -> {}", i.0, i.1);
    }

    stack.push(range[0]);

    for i in 1..sz {
        let index = stack.len() - 1;
        let can_merge = stack[index];
        let current = range[i];

        if overlap(can_merge, current) {
            let new_entry = (can_merge.0, can_merge.1.max(current.1));
            println!(
                "Compressing {}-{} and {}-{} into {}-{}",
                can_merge.0, can_merge.1, current.0, current.1, new_entry.0, new_entry.1
            );
            stack[index] = new_entry;
        } else {
            stack.push(current);
            println!(
                "No overlap between {}-{} and {}-{}",
                can_merge.0, can_merge.1, current.0, current.1,
            );
        }
    }

    let total = calculate_ranges(&mut stack);
    total
}

fn calculate_ranges(stack: &mut Vec<(u64, u64)>) -> u64 {
    let mut total = 0u64;
    //   println!("compressed ranges = {}", stack.len());

    for tuple in stack {
        println!("a {} - {}", tuple.0, tuple.1);
        total += tuple.1 - tuple.0 + 1; // bounds are inclusive
    }

    return total;
}

// checks intersection of ranges from.
// rhs.0 <= lhs.1 <= rhs.1
// edge case : lsh.0 <= rhs.0 . true overlap
// lhs.0 >= rhs.0  ; range rhs.0 - rhs.1 encloses the range lhs.0 to lhs.1
fn overlap(lhs: (u64, u64), rhs: (u64, u64)) -> bool {
    lhs.1 >= rhs.0
}

fn main() {
    let res = part_two();
    println!("result = {res}");

    // previous result (too large = 314877583939389)
    //                              357907198933894
    // let (mut range, _) = read_input();
    // range.sort();
    // // range.sort_by(|l, r| l.1.cmp(&r.1));
    // for i in range {
    // println!("{} -> {}", i.0, i.1);
    // }
}
