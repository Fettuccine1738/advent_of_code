use std::fs;

const _SAMPLE: &str = "sample.txt";
const EXACT: &str = "Exact.txt";
const BOUND: (u32, u32, u32) = (12, 13, 14);

pub fn solution_one() -> Result<u32, std::io::Error> {
    let read = fs::read_to_string(EXACT)?;
    println!("{}", &read[..10]);
    let mut result: u32 = 0;

    for line in read.lines() {
        //  println!("{line}");
        result += parse_into_cube(&line);
    }
    Ok(result)
}

pub fn solution_two() -> Result<u32, std::io::Error> {
    let read = fs::read_to_string(EXACT)?;
    println!("{}", &read[..10]);
    let mut result: u32 = 0;

    for line in read.lines() {
        //  println!("{line}");
        let power = get_fewest_colors(&line);
        result += pow(power);
    }

    Ok(result)
}

fn get_fewest_colors(line: &str) -> (u32, u32, u32) {
    let list: Vec<&str> = line.split(|c: char| c == ';' || c == ':').collect();

    let mut t: (u32, u32, u32, u32) = (0, 0, 0, 0);

    for (i, &w) in list.iter().enumerate() {
        let face: Vec<&str> = w.trim().split_whitespace().collect();
        // println!("face {i} {:?}", face);
        if i == 0 {
            // id
            if let Some(value) = face.get(1) {
                t.0 = value.parse::<u32>().unwrap();
            }
        } else {
            for r in 0..face.len() {
                if r % 2 != 0 {
                    continue;
                }
                if let Some(val) = face.get(r) {
                    let color = face.get(r + 1).unwrap();
                    if color.starts_with("red") {
                        t.1 = t.1.max(val.parse::<u32>().unwrap());
                    } else if color.starts_with("green") {
                        t.2 = t.2.max(val.parse::<u32>().unwrap());
                    } else if color.starts_with("blue") {
                        t.3 = t.3.max(val.parse::<u32>().unwrap());
                    }
                }
            }
        }
    }
    println!("{}\t{}\t{}\t{}", t.0, t.1, t.2, t.3);
    (t.1, t.2, t.3)
}

fn pow(color: (u32, u32, u32)) -> u32 {
    color.0 * color.1 * color.2
}

fn parse_into_cube(line: &str) -> u32 {
    let list: Vec<&str> = line.split(|c: char| c == ';' || c == ':').collect();

    let mut t: (u32, u32, u32, u32) = (0, 0, 0, 0);

    for (i, &w) in list.iter().enumerate() {
        let face: Vec<&str> = w.trim().split_whitespace().collect();
        // println!("face {i} {:?}", face);
        if i == 0 {
            // id
            if let Some(value) = face.get(1) {
                t.0 = value.parse::<u32>().unwrap();
            }
        } else {
            t.1 = 0;
            t.2 = 0;
            t.3 = 0;

            for r in 0..face.len() {
                if r % 2 != 0 {
                    continue;
                }
                if let Some(val) = face.get(r) {
                    let color = face.get(r + 1).unwrap();
                    if color.starts_with("red") {
                        t.1 = val.parse::<u32>().unwrap();
                    } else if color.starts_with("green") {
                        t.2 = val.parse::<u32>().unwrap();
                    } else if color.starts_with("blue") {
                        t.3 = val.parse::<u32>().unwrap();
                    }
                }
            }

            if !is_valid_config(t.1, t.2, t.3) {
                println!("invalid config {} face {}, {}, {}", t.0, t.1, t.2, t.3);
                return 0;
            }
        }
    }
    // println!("{}\t{}\t{}\t{}", t.0, t.1, t.2, t.3);
    t.0
}

fn is_valid_config(r: u32, g: u32, b: u32) -> bool {
    return BOUND.0 >= r && BOUND.1 >= g && BOUND.2 >= b;
}
