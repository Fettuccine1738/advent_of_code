use dayone::{solution_one, solution_two};

const ACTUAL: &str = "input.txt";
const SAMPLE: &str = "example.txt";

fn main() {
    let result_one = solution_one(SAMPLE);
    let result_two = solution_two(SAMPLE);

    // assert_eq!(result_one, 11);
    // assert_eq!(result_two, 31);

    println!("{} result one ", result_one);
    println!("{} result two ", result_two);

    let result_one = solution_one(ACTUAL);
    let result_two = solution_two(ACTUAL);

    assert_eq!(result_one, 1666427);
    assert_eq!(result_two, 24316233);

    println!("{} result one ", result_one);
    println!("{} result two ", result_two);
}

//fn main() -> Result<(), Box<dyn Error>> {
//let input_txt = "input.txt";
//let example_txt = "example.txt";
//
//let actual = fs::read_to_string(input_txt)?;
//let example = fs::read_to_string(example_txt)?;
//
//// let reader = BufReader::new(actual);
//// let reader = BufReader::new(actual);
//
//println!("sample dataset\n {}", example);
//
//let mut right_column: Vec<i32> = Vec::new();
//let mut left_column: Vec<i32> = Vec::new();
//
//let mut count = 0;
//
//for line in example.lines() {
//for column in line.split_whitespace() {
//let number: i32 = column.trim().parse().map_err(|_| {
//io::Error::new(io::ErrorKind::InvalidData, "Invlaidnumber in column")
//})?;
//if count % 2 == 0 {
//right_column.push(number)
//} else {
//left_column.push(number)
//}
//count += 1;
//}
//}
//
//println!("Count = {count}");
//// when count hits None : = 12
//assert_eq!((count) % 2 == 0, true);
//
//left_column
//.iter()
//.zip(right_column.clone())
//.for_each(|(a, b)| println!("{} \t {}", a, b));
//
//let mut map: HashMap<i32, i32> = HashMap::new();
//
//for num in &right_column {
//let app = map.entry(*num).or_insert(0);
//*app += 1;
//}
//
//println!("map {:?}", map);
//
//let sum: i32 = left_column
//.iter()
//.map(|x| x * map.get(x).unwrap_or(&0))
//.sum();
//println!("sum {sum}");
//
//Ok(())
//}
