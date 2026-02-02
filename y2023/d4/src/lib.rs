use std::fs;

const SAMPLE: &str = "sample.txt";
const EXCT: &str = "exact.txt";

fn solution_one() -> Result<u32, std::io::Error> {
    let read = fs::read_to_string(SAMPLE)?;
    let mut result = 0;
    let mut array: Vec<Vec<char>> = Vec::new();

    for line in read.lines() {
        let current = line.chars().collect();
        array.push(current);
    }

    for v in &array {
        let mut s: String = String::new();
        for c in v {
            if c.is_ascii_digit() {
                s.push(c);
            }
        }
    }
    Ok(result)
}
