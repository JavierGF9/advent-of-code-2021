use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

const FILE_PATH: &str = "../inputs/day1.txt";

fn main() {
    let lines = read_lines(FILE_PATH).expect("Error when reading file");
    let mut old_line: i32 = -1;
    let mut increase_counter: u32 = 0;

    for line in lines {
        if let Ok(value) = line {
            print!("{} ", value);
            let integer_value: i32 = value.parse().expect("Line is not a number");

            if old_line < 0 {
                println!("(N/A - no previous measurement)")
            } else if old_line < integer_value {
                println!("(increased)");
                increase_counter += 1;
            } else {
                println!("(decreased)")
            }

            old_line = integer_value;
        } else {
            println!("Error when reading value");
        }
    }

    println!(
        "\nLarger measurements than the previous measurement: {}",
        increase_counter
    )
}

fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(&Path::new(filename))?;
    Ok(BufReader::new(file).lines())
}
