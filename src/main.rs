mod solutions;

use solutions::{
    day_1,
    day_2,
    day_3,
};

use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::path::Path;

fn main() {
    let input = read_input_to_lines("input/day_3.txt");

    match input {
        Ok(lines) => {
            let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
            println!("Solution: {}", day_3::solve_2(&lines));
        },
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn read_input_to_lines(path: impl AsRef<Path>) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader.lines()
        .collect()
}
