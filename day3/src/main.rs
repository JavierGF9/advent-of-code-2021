use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

const FILE_PATH: &str = "../inputs/day3.txt";

struct Counter {
    zeroes: u32,
    ones: u32,
}

fn main() {
    let lines = read_lines(FILE_PATH).expect("Error when reading file");
    let mut gamma_rate: i32 = 0;
    let mut epsilon_rate: i32 = 0;
    let mut counters: Vec<Counter> = Vec::new(); // Counter which contains numbers of zeros and ones

    for line in lines {
        if let Ok(value) = line {
            if counters.is_empty() {
                let value_size = value.chars().count();
                while counters.len() < value_size {
                    counters.push(Counter { zeroes: 0, ones: 0 });
                }
            }

            for (i, c) in value.chars().enumerate() {
                if c == '0' {
                    counters[i].zeroes += 1
                } else {
                    counters[i].ones += 1
                }
            }
        }
    }

    for counter in counters.iter() {
        gamma_rate <<= 1;
        epsilon_rate <<= 1;

        if counter.ones > counter.zeroes {
            gamma_rate += 1;
        } else if counter.ones < counter.zeroes {
            epsilon_rate += 1;
        } else {
            panic!("I don't know what to do in this case");
        }
    }

    println!("Gamma rate is {}", gamma_rate);
    println!("Epsilon rate is {}", epsilon_rate);
    println!("Gamma * Epsilon = {}", gamma_rate * epsilon_rate);
}

fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(&Path::new(filename))?;
    Ok(BufReader::new(file).lines())
}
