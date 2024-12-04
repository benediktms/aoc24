use core::num;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(Path::new("./day1/src/input.txt"))?;
    let reader = BufReader::new(file);

    let mut lhs: Vec<i32> = Vec::new();
    let mut rhs: Vec<i32> = Vec::new();

    let pairs = reader
        .lines()
        .map(|input| {
            if let Ok(line) = input {
                let numbers = line
                    .split("   ")
                    .map(|n| n.parse::<i32>().expect("Failed to parse number"))
                    .collect::<Vec<i32>>();

                lhs.push(numbers[0]);
                rhs.push(numbers[1]);

                numbers
            } else {
                vec![0, 0]
            }
        })
        .collect::<Vec<Vec<i32>>>();

    lhs.sort();
    rhs.sort();

    let sum = lhs
        .iter()
        .zip(rhs.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    println!("{:?}", sum);

    Ok(())
}
