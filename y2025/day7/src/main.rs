#![allow(dead_code, unused_imports)]
use core::time;
use std::{collections::HashMap, fs, io, time::Instant};

const SN: &str = "sample.txt";
const IN: &str = "input.txt";

fn part_one() -> u64 {
    let input = fs::read_to_string(IN).expect("Could not read file ");
    let mut lines: Vec<Vec<char>> = input.trim().lines().map(|s| s.trim().chars().collect()).collect();

    let mut start = 0usize;
    for ch in &lines[0] {
        if *ch == 'S' {
            break;
        } 
        start = start + 1;
    };

    let mut splitted: u64  = 0u64;

    // iterate from 1..end - 1, if char above == | or == 'S' 
    // set the current char to |. if char below is ^ (splitter). increase splitted count and add to adjacent line below a '|'. 
    for i in 0..lines.len() - 1 {
        // let (_, rest) = lines.split_at_mut(i);
        let (left, right) = lines.split_at_mut(i + 1);

        let current = &mut left[left.len() - 1];
        let next = &mut right[0];

        for j in 0..current.len() {
            if current[j] == '|' || current[j] == 'S' {
                if next[j] == '^'  {
                    splitted += 1;
                    if j > 0 && j < current.len() - 1{
                        next[j - 1] = '|';
                        next[j + 1] = '|';
                    } else if j == 0 {
                        next[j + 1] = '|';
                    } else {
                        next[j - 1] = '|';
                    }
                } else {
                    next[j] = '|';
                }
            }
        }

            }

    splitted
}

fn part_two() -> u64 {
    let input = fs::read_to_string(IN).expect("Could not read file ");
    let mut lines: Vec<Vec<char>> = input.trim().lines().map(|s| s.trim().chars().collect()).collect();
    let mut visuals = Vec::new(); // lines.clone();
    let mut init_pos = 0usize;
    for ch in &lines[0] {
        if *ch == 'S' {
            break;
        } 
        init_pos += 1;
    };

    let mut cache: HashMap<(usize, usize), u64> = HashMap::new();

    return memoized_dfs(&mut lines, &mut visuals, 0usize, init_pos, &mut cache);
}

fn visualize(lines: &Vec<Vec<char>>) {
    for i in lines {
        println!("{:?}", i);
    }
    println!("\n");
}

fn memoized_dfs(state: &Vec<Vec<char>>, visualizer: &mut Vec<Vec<char>>, index: usize, beam: usize, cache: &mut HashMap<(usize, usize), u64>)  -> u64 {
    // visualize(&visualizer);
    if index == state.len() - 1 {
        return 1;
    } 

    if let Some(value) = cache.get(&(index, beam)) {
        return *value;
    }

    let mut result = 0u64;
    let next = index + 1;

    // note where the '|' or 'S' is at. beam is always pointing to the current position of the beam 
    // get state[index][pointer] == .  ? set state[index + 1][pointer] == '|'  
    // and call dfs(state, timelines, index + 1, pointer) else  if == '^' set
    // state[index + 1][pointer +- 1]; and call dfs(state, timelines, index + 1, pointer +- 1);
    //  
    // pointer to left and right 
    match state[next][beam] {
        '.' => {
                // visualizer[next][beam] = '|';
                result += memoized_dfs(state, visualizer, next, beam, cache); 
                // visualizer[next][beam] = '.';
            },
        '^' => {
            let width = state[index + 1].len();
            if beam > 0 {
                // visualizer[next][beam - 1] = '|';
                result += memoized_dfs(state, visualizer, next, beam - 1, cache);
                // visualizer[next][beam - 1] = '.';
            }
            if beam < width {
                // visualizer[next][beam + 1] = '|';
                result += memoized_dfs(state, visualizer,  next, beam + 1, cache);
                // visualizer[next][beam + 1] = '.';
            }
        },
        _ => panic!("unexpected char found")
    }
    cache.insert((index, beam), result);
    result
}

fn main() {
    let start = Instant::now();
    let res = part_two();
    let duration = start.elapsed();
    println!("{res} in {}", duration.as_micros());
}

fn dfs(state: &mut Vec<Vec<char>>, timelines: &mut u64, index: usize, beam: usize) {
    visualize(state);
    if index == state.len() - 1 {
        *timelines += 1;
        return;
    } 


    // note where the '|' or 'S' is at. beam is always pointing to the current position of the beam 
    // get state[index][pointer] == .  ? set state[index + 1][pointer] == '|'  
    // and call dfs(state, timelines, index + 1, pointer) else  if == '^' set
    // state[index + 1][pointer +- 1]; and call dfs(state, timelines, index + 1, pointer +- 1);
    //  
    // pointer to left and right 
    if state[index + 1][beam] == '.' {
        state[index + 1][beam] = '|';
        dfs(state, timelines, index + 1, beam);
        state[index + 1][beam] = '.'; // reset for visualizetion later.
    } else if state[index + 1][beam] == '^' {
        if beam > 0 {
            state[index + 1][beam - 1] = '|';
            dfs(state, timelines, index + 1, beam - 1);
            state[index + 1][beam - 1] = '.';

        } 
        if beam < state[index + 1].len() {
            state[index + 1][beam + 1] = '|';
            dfs(state, timelines, index + 1, beam + 1);
            state[index + 1][beam + 1] = '.';
        } 
    }
}

