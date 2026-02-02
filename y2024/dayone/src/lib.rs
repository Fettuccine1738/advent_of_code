use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self};

// private by default
fn read_file(filepath: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
    let reader = fs::read_to_string(filepath)?;

    let mut rside: Vec<u32> = Vec::new();
    let mut lside: Vec<u32> = Vec::new();

    let mut alternate = 0;
    for line in reader.lines() {
        for cols in line.split_whitespace() {
            let num: u32 = cols.trim().parse().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Invlaidnumber in column")
            })?;

            if alternate % 2 == 0 {
                lside.push(num)
            } else {
                rside.push(num)
            }
            alternate += 1;
        }
    }
    Ok((lside, rside))
}

pub fn solution_one(path: &str) -> u32 {
    let (mut lside, mut rside) = match read_file(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file {} ", e);
            return 0;
        }
    };

    lside.sort();
    rside.sort();

    // return value
    let result = lside
        .into_iter()
        .zip(rside)
        .map(|(a, b)| a.abs_diff(b))
        .sum();
    result
}

pub fn solution_two(path: &str) -> u32 {
    let (mut lside, mut rside) = match read_file(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file {} ", e);
            return 0;
        }
    };

    let mut map: HashMap<u32, u32> = HashMap::new();
    // println!("From map {:?}", map);
    // dbg!(&map);

    for i in &rside {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }

    // dbg!(&map);
    // result
    let value = lside
        .into_iter()
        .map(|x| x * map.get(&x).unwrap_or(&0))
        .sum();
    value
}

// fn read_file(filepath: String) -> Result<(Vec<i32>, Vec<i32>), io::Error> {
// let file = File::open(filepath)?;
// let reader = BufReader::new(file);
//
// let mut vec_1: Vec<i32> = Vec::new();
// let mut vec_2: Vec<i32> = Vec::new();
//
// for line in reader.lines() {
// let line = line?;
// let mut parts = line.trim().split_whitespace();
//
// if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
// let first_n = first.trim().parse().map_err(|_| {
// io::Error::new(io::ErrorKind::InvalidData, "Invalid number in column 1")
// })?;
//
// vec_1.push(first_n);
//
// let second_n = second.trim().parse().map_err(|_| {
// io::Error::new(io::ErrorKind::InvalidData, "Invalid number in column 2")
// })?;
//
// vec_2.push(second_n);
// }
// }
// Ok((vec_1, vec_2))
// }
//
// fn solution_one(path: &str) -> u32 {
// let (mut column_one, mut column_two) = match read_file(path.to_string()) {
// Ok(data) => data,
// Err(e) => {
// eprintln!("Error reading file {} ", e);
// return 0;
// }
// };
//
// if column_one.len() != column_two.len() {
// eprintln!(
// "Mismatched column lengths {} {}",
// column_one.len(),
// column_two.len()
// )
// }
//
// column_one.sort();
// column_two.sort();
//
// column_one
// .into_iter()
// .zip(column_two) // combines iterators into one with pairs of elements
// .map(|(a, b)| a.abs_diff(b)) // changes value to u32
// .sum()
// }
//
// fn solution_two(path: String) -> i32 {
// let (c1, c2) = match read_file(path.to_string()) {
// Ok(data) => data,
// Err(e) => {
// eprintln!("Error reading file {} ", e);
// return 0;
// }
// };
//
// // let mut map = HashMap::new();
// let mut map: HashMap<i32, i32> = HashMap::new();
//
// for n in &c2 {
// let count = map.entry(*n).or_insert(0);
// *count += 1;
// }
//
// c1.into_iter()
// .map(|x| x * map.get(&x).copied().unwrap_or(0))
// .sum()
// }
//
// #[cfg(test)]
// mod test {
// use super::{solution_one, solution_two};
// // const EXAMPLE: &str = include_str!("example.txt");
// // const ACTUAL: &str = include_str!("input.txt");
//
// #[test]
// fn test_part1() {
// assert_eq!(format!("{}", solution_one("example.txt")), "11");
// }
//
// #[test]
// fn test_part1_actual() {
// assert_eq!(format!("{}", solution_one("input.txt")), "1666427");
// }
//
// #[test]
// fn test_part2() {
// assert_eq!(format!("{}", solution_two("example.txt".to_string())), "31");
// }
//
// #[test]
// fn test_part2_actual() {
// assert_eq!(
// format!("{}", solution_two("example.txt".to_string())),
// "24316233"
// );
// }
// }
