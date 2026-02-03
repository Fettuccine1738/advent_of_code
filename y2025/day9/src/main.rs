#![allow(dead_code, unused_imports)]
use std::{fs, io, time::{Instant, Duration}};

const SN: &str = "sample.txt";
const IN: &str = "input.txt";

const FN_TRUE: fn() ->  bool = || true; 
//const FN_BOUNDS_CROSSED: fn(&Vertex, &Vertex, &Vec<usize>) -> bool = 

#[derive(Debug, Copy, Clone)]
struct Vertex(u64, u64); // (col, row)

// source and target
#[derive(Debug)]
struct Edge(usize, usize);

impl Vertex {
    pub fn area(&self, other: &Self) -> u64 {
        let col_diff = 1 + self.0.abs_diff(other.0); 
        let row_diff = 1 + self.1.abs_diff(other.1); 

        col_diff * row_diff
    } 
}

// treat as vertices v1 = (col, row) 
// max area between v1 and v2 = (|col1 - col2| + 1) * (|row1 - row2| + 1)
fn read_input(file: &str) -> Vec<Vertex> {
    let input = fs::read_to_string(file).expect("Could not read file.");
    // println!("{}", input);

    let lines = input.trim().lines();
    let mut vertices: Vec<Vertex> = Vec::new();

    for line in lines {
        let temp: Vec<u64> = line.trim().split(",")
            .map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        if !temp.is_empty() {
            vertices.push(Vertex(temp[0], temp[1]));
        }
    }
    vertices
}


fn part_one() -> u64 {
	let vertices: Vec<Vertex> = read_input(IN);
    let mut max_area = 0u64;

    for i in 0..vertices.len() - 1  {
        let s = &vertices[i];
        for j in i+1..vertices.len() {
            let t = &vertices[j];
            // commented out because benchmarks show its faster to not make this check.
            // if s.0 != t.0 && s.1 != t.1 {
            //     // only look at diagonals, non-diagonals will always have a side == 1
            //     max_area = max_area.max(s.area(t)); 
            // }
            max_area = max_area.max(s.area(t)); 
        }
    }

    max_area
}


fn part_two() -> u64 {
	let vertices: Vec<Vertex> = read_input(SN);
    // let _bounds:Vec<Edge> = connect_adj_vertices(&vertices);
    let mut max_area = 0u64;

    for i in 0..vertices.len() - 1  {
        let s = &vertices[i];
        for j in i+1..vertices.len() {
            let t = &vertices[j];

            if s.0 == t.0 || s.1 == t.1 {
                continue; // look at adjacent only. same row / same col are useless because they can never have the biggest area.  
            }
            // check the other 2 corners if they are inside bounds 
            let v1 = Vertex(s.0, t.1);
            let c = vertices[(i + 1) % vertices.len()].0;
            let r = vertices[(j - 1) % vertices.len()].0;
            let b1: bool =  c <= v1.0 && r <= v1.1;

            let v2 = Vertex(s.1, t.0);
            let c = vertices[(i + 1) % vertices.len()].0;
            let r = vertices[(j - 1) % vertices.len()].0;

            let b2: bool = c <= v2.0 && r <= v2.1;

            max_area = if b1 && b2 { max_area.max(s.area(t)) } else { max_area };
        }
    }
    max_area
}

fn connect_adj_vertices(vertices: &Vec<Vertex>) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..vertices.len() {
        // connect 0->1, 1->2, n-1 -> (n %n) == 0 . 
        edges.push(Edge(i, (i + 1) % vertices.len()));
    }
    edges
}

fn main() {
    let now = Instant::now();
    println!("ready.. ");
    let res = part_two();
    let duration = now.elapsed();
    println!("{res} in {}", duration.as_micros());
}
