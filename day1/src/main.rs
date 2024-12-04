use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let lines = reader.lines().map(|input| {
        if let Ok(line) = input {
            line.split("   ")
                .map(|nums| nums.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        } else {
            vec![0, 0]
        }
    });

    Ok(())
}
