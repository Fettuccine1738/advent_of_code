#![allow(unused)]
use std::collections::HashMap;
use std::fs;
use std::io::pipe;
use day8::union_find::UF;
/// NOTE: Connected components Graph theory.
/// Another definition of components involves the equivalence classes of an equivalence relation defined on the graph's vertices.
/// In an undirected graph, a vertex v {\displaystyle v} is reachable from a vertex u {\displaystyle u} if there is a path from u  to v,
/// or equivalently a walk (a path allowing repeated vertices and edges). Reachability is an equivalence relation, since:
/// It is reflexive: There is a trivial path of length zero from any vertex to itself.
/// It is symmetric: If there is a path from u {\displaystyle u} to v {\displaystyle v}, the same edges in the reverse order form a path from v {\displaystyle v} to u {\displaystyle u}.
/// It is transitive: If there is a path from u {\displaystyle u} to v {\displaystyle v} and a path from v {\displaystyle v} to w {\displaystyle w}, the two paths may be concatenated together to form a walk from u {\displaystyle u} to w {\displaystyle w}.

static SN: &str = "sample.txt";
static AN: &str = "actual.txt";

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Vertex(u64, u64, u64);

#[derive(Debug, Clone, Copy)]
pub struct Edge(usize, usize, f64);

impl Vertex {
   pub fn distance(&self, other: &Self) -> f64 {
       let a = self.0.abs_diff(other.0) as f64;
       let b = self.1.abs_diff(other.1) as f64;
       let c = self.2.abs_diff(other.2) as f64;

       (a * a + b * b + c * c).sqrt()
   }
}

pub fn init_edges(vertices: &Vec<Vertex>) -> Vec<Edge> {
   let mut edges: Vec<Edge> = Vec::new();

   for s in 0..vertices.len() - 1 {
       for t in s+1..vertices.len() {
           edges.push(create_edge(s, t, &vertices));
       }
   }

   edges
}

fn create_edge(s: usize, t: usize, v: &Vec<Vertex>) -> Edge {
   Edge(s, t, v[s].distance(&v[t]))
}


pub fn get_input(input: &str) -> Vec<Vertex> {
   let sample = fs::read_to_string(input).expect("Could not read file.");
   let lines = sample.lines();
   // let set: HashSet<usize> = HashSet::new();
   let mut vertices: Vec<Vertex> = Vec::new();

   for line in lines {
       let current = line
           .split(",")
           .map(|ch| ch.parse::<u64>().unwrap())
           .collect::<Vec<u64>>();
       let s: Vertex = Vertex(current[0], current[1], current[2]);
       vertices.push(s);
   }
   vertices
}

pub fn part_one(limit: u64) -> u64 {
   let vertices = get_input(AN);
   let mut disjoint_set: UF = UF::new(vertices.len()); 
   let mut edges: Vec<Edge> = init_edges(&vertices);

   edges.sort_by(|lhs, rhs| lhs.2.partial_cmp(&rhs.2).unwrap());
   for e in &edges {
       if disjoint_set.counter == limit {
           return disjoint_set.reduce();
       }

       // connections
       if disjoint_set.union(e.0, e.1) {
           println!("Connection made {:?} to {:?} with edge {:?}", vertices[e.0], vertices[e.1], e);
       } else {
           println!("{:?}  are already in the same connected component {:?} with edge {:?}", vertices[e.0], vertices[e.1], e);
       }
   }

   disjoint_set.counter
}

fn part_two() -> u64 {
   let vertices = get_input(AN);
   let mut disjoint_set: UF = UF::new(vertices.len()); 
   let mut edges: Vec<Edge> = init_edges(&vertices);
   let mut connections: u64 = 0u64;

   edges.sort_by(|lhs, rhs| lhs.2.partial_cmp(&rhs.2).unwrap());

   for e in &edges {
       // connections
       if disjoint_set.union(e.0, e.1) {
           println!("Connection made {:?} to {:?} with edge {:?}", vertices[e.0], vertices[e.1], e);
       } else {
           println!("{:?}  are already in the same connected component {:?} with edge {:?}", vertices[e.0], vertices[e.1], e);
       }


       if disjoint_set.components ==  1 {
           let source =  &vertices[e.0];
           let target =  &vertices[e.1];

           println!("Fully Connected with {:?} and {:?}  on edge {:?}", source, target, e);
           return  source.0 * target.0;
       }
   }

   std::u64::MAX
}

fn main() {
   // let v =  get_input();
   let r = part_two();
   println!("{r}");
}
 