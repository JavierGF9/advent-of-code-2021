use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

const FILE_PATH: &str = "../inputs/day1.txt";
const WINDOW_SIZE: usize = 3;

fn main() {
    let windows: Vec<i32> = build_windows_vector();
    let mut increase_counter: i32 = 0;

    for (index, current_window) in windows.iter().enumerate() {
        print!("{} ", current_window);

        if index == 0 {
            println!("(N/A - no previous measurement)");
            continue;
        }

        let previous_window: i32 = windows[index - 1];

        if previous_window < *current_window {
            println!("(increased)");
            increase_counter += 1;
        } else if previous_window > *current_window {
            println!("(decreased)")
        } else {
            println!("(no change)")
        }
    }

    println!(
        "\nLarger measurements than the previous measurement: {}",
        increase_counter
    )
}

fn build_windows_vector() -> Vec<i32> {
    let lines = read_lines(FILE_PATH).expect("Error when reading file");
    let mut windows: Vec<i32> = vec![];
    let mut values_added: Vec<usize> = vec![1, 1];

    for (index, line) in lines.enumerate() {
        if let Ok(value) = line {
            let integer_value: i32 = value.parse().expect("Line is not a number");

            if index == 0 {
                windows.push(integer_value);
            } else if index == 1 {
                windows[0] += integer_value;
                windows.push(integer_value);
            } else {
                for i in (index + 1 - WINDOW_SIZE)..=index {
                    if i > (windows.len() - 1) {
                        windows.push(integer_value);
                        values_added.push(1);
                    } else {
                        windows[i] += integer_value;
                        values_added[i] += 1;
                    }
                }
            }
        } else {
            println!("Error when reading value");
        }
    }

    while values_added[values_added.len() - 1] < WINDOW_SIZE {
        values_added.pop();
        windows.pop();
    }

    windows
}

fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(&Path::new(filename))?;
    Ok(BufReader::new(file).lines())
}
