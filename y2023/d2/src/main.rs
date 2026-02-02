use d2::solution_one;
use d2::solution_two;

fn main() {
    let line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";

    println!("Hello, world!");
    let list: Vec<&str> = line
        .split(|c: char| c == ';' || c == ':' || c == ',')
        .collect();

    println!("Result {:?}", list);

    // HACK: since the returned error is mapped to something else,
    // eprintln returns a unit struct affecting the type of the variable
    let res: Result<u32, ()> =
        solution_two().map_err(|err| eprintln!("IO error opening file {}", err));

    if let Ok(val) = res {
        println!("in Result = {val}");
    }

    // match res {
    //     Ok(v) => println!("result = {v}"),
    //     _ => eprintln!("IO error"),
    // }
}
