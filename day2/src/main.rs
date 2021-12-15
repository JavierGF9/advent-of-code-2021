use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

const FILE_PATH: &str = "../inputs/day2.txt";

fn main() {
    let lines = read_lines(FILE_PATH).expect("Error when reading file");
    let mut depth: i32 = 0;
    let mut horizontal_position: i32 = 0;

    for line in lines {
        if let Ok(values) = line {
            let split_values: Vec<&str> = values.split(" ").collect();
            let order: &str = split_values[0];
            let amount: i32 = (*split_values[1]).parse().expect("Amount is not a number");

            match order {
                "forward" => horizontal_position += amount,
                "up" => depth -= amount,
                "down" => depth += amount,
                _ => panic!("Unknown order: {}", order),
            }
        }
    }

    println!("Horizontal position is {}", horizontal_position);
    println!("Depth is {}", depth);
    println!("Result is {}", horizontal_position * depth);
}

fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(&Path::new(filename))?;
    Ok(BufReader::new(file).lines())
}
