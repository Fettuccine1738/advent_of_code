#![allow(unused)]
pub mod union_find {

   pub struct UF {
       parent: Vec<usize>,
       size: Vec<u64>,
       pub components: usize,
       pub counter: u64,
   }

   impl UF {
       pub fn new(cap: usize) -> Self {
           UF {
               parent:  (0..cap).collect(),
               size: vec![1u64; cap],
               components: cap,
               counter: 0
           }
       }

       pub fn find(&self, p: usize) -> usize {
           let mut x = p;

           while self.parent[x] != x {
               x = self.parent[x];
           }
           return x;
       }


       pub fn union(&mut self, p: usize, q: usize) -> bool {
           let s = self.find(p);
           let t = self.find(q);
           self.counter += 1;

           if s == t {
               return false;
           }

           // make smaller root point to larger one
           if self.size[s] < self.size[t] {
               self.parent[s] = q;
               self.size[t] += self.size[s];
           } else {
               self.parent[t] = s;
               self.size[s] += self.size[t];
           }
           self.components -= 1;
           true
       }

       pub fn connected(&self, p: usize, q: usize) -> bool {
           self.find(p) == self.find(q)
       }

       pub fn reduce(&mut self) -> u64 {
           let idx = self.size.len() - 1;
           self.size.sort_by(|l, r| r.cmp(l)); // sort desc
           print!("{:?}", self.size);
           self.size[0] * self.size[1] * self.size[2]
       }

   }
}