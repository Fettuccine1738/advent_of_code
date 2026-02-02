#![allow(dead_code, unused_imports)]
use std::{fs, io};

const SN: &str = "sample.txt";
const IN: &str = "input.txt";

#[derive(Debug, Copy, Clone)]
struct Edge(u64, u64); // (col, row)

impl Edge {
    pub fn area(&self, other: &Self) -> u64 {
        let col_diff = 1 + self.0.abs_diff(other.0); 
        let row_diff = 1 + self.1.abs_diff(other.1); 

        col_diff * row_diff
    } 
}

// treat as vertices v1 = (col, row) 
// max area between v1 and v2 = (|col1 - col2| + 1) * (|row1 - row2| + 1)
fn read_input(file: &str) -> Vec<Edge> {
    let input = fs::read_to_string(file).expect("Could not read file ");
    // println!("{}", input);

    let lines = input.trim().lines();
    let mut edges: Vec<Edge> = Vec::new();

    for line in lines {
        let temp: Vec<u64> = line.trim().split(",")
            .map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        if !temp.is_empty() {
            edges.push(Edge(temp[0], temp[1]));
        }
    }
    edges
}


fn part_one() -> u64 {
	let edges: Vec<Edge> = read_input(IN);
    let mut max_area = 0u64;

    for i in 0..edges.len() - 1  {
        let s = &edges[i];
        for j in i+1..edges.len() {
            let t = &edges[j];
            max_area = max_area.max(s.area(t));
        }
    }

    max_area
}


fn part_two() {
//	let input = read_input(IN);
    todo!()
}

fn main() {
    println!("ready.. ");
    let res = part_one();
    println!("{res}");
}
