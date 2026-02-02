// use fancy_regex::Regex;
// use regex::Regex;
use std::env;
use std::fs;

fn parse_args() -> String {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!(
            "wrong number of args, expected 1 (filename), got {}",
            args.len(),
        );
        std::process::exit(1);
    }
    args[0].clone()
}

fn solution_one() -> Result<u32, std::io::Error> {
    let filename: String = parse_args();
    let data = fs::read_to_string(&filename)?;
    let mut result = 0u32;

    for line in data.lines() {
        let mut seen: bool = false;
        let mut num: u32 = 0;

        for c in line.chars() {
            let is_num = c.is_numeric();
            if is_num && !seen {
                num = c as u32;
                num = num - 48;
                result += num * 10;
                seen = true;
            } else if is_num && seen {
                num = c as u32;
                num = num - 48;
            }
        }
        result += num; // add last digit 
    }

    Ok(result)
}

fn parse_start_end(s: &str) -> u32 {
    if s.starts_with("one") {
        return 1;
    } else if s.starts_with("two") {
        return 2;
    } else if s.starts_with("three") {
        return 3;
    } else if s.starts_with("four") {
        return 4;
    } else if s.starts_with("five") {
        return 5;
    } else if s.starts_with("six") {
        return 6;
    } else if s.starts_with("seven") {
        return 7;
    } else if s.starts_with("eight") {
        return 8;
    } else if s.starts_with("nine") {
        return 9;
    }
    0
}

fn find_digit(s: &str, start: bool) -> u32 {
    let array: Vec<char> = s.chars().collect();

    let st = 0;
    let end = s.len();

    if start {
        for i in st..end {
            if let Some(ch) = array.get(i) {
                let c = *ch;
                if c.is_ascii_digit() {
                    return (c as u32) - 48;
                } else {
                    continue;
                }
            }
        }
    } else {
        for i in (st..end).rev() {
            if let Some(ch) = array.get(i) {
                let c = *ch;

                if c.is_ascii_digit() {
                    return (c as u32) - 48;
                } else {
                    continue;
                }
            }
        }
    }
    0
}

fn find_all_overlapping_digits(line: &str) -> Vec<String> {
    let digit_patterns = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut matches = Vec::new();
    let chars: Vec<char> = line.chars().collect();

    for i in 0..chars.len() {
        // Check for single digit
        if chars[i].is_ascii_digit() {
            matches.push((i, chars[i].to_string()));
            continue;
        }

        // Check for spelled-out digits
        let remaining: String = chars[i..].iter().collect();
        for &(pattern, digit) in &digit_patterns {
            if remaining.starts_with(pattern) {
                matches.push((i, pattern.to_string()));
                break; // Only take the first match at this position
            }
        }
    }

    // Sort by position and return the matched strings
    matches.sort_by_key(|&(pos, _)| pos);
    matches.into_iter().map(|(_, s)| s).collect()
}

fn solution() -> Result<u32, std::io::Error> {
    let filename: String = parse_args();
    let data = fs::read_to_string(&filename)?;
    let mut res: u32 = 0;

    for line in data.lines() {
        let spelled_digit = find_all_overlapping_digits(line);
        let length = spelled_digit.len();
        //println!("line, {line} \t {:?}", spelled_digit);

        let mut start = match spelled_digit.get(0) {
            Some(num) => parse_start_end(num),
            None => 0,
        };
        if start == 0 {
            start = find_digit(&line, true);
        }

        let mut end = match spelled_digit.get(length - 1) {
            Some(num) => parse_start_end(num),
            None => 0,
        };
        if end == 0 {
            end = find_digit(&line, false);
        }

        res += start * 10 + end;
    }
    Ok(res)
}

fn main() {
    let res = match solution_one() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read from file {}", e);
            std::process::exit(1);
        }
    };

    println!("Result = {}\n\n", res);

    let res2 = match solution() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read from file {}", e);
            std::process::exit(1);
        }
    };

    println!("Result = {}", res2);
}
