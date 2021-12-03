use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_file_content<T, P>(filename: P) -> Vec<T>
where
    P: AsRef<Path>,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let lines = read_lines(filename).expect("Unable to readlines");
    lines
        .map(|line| {
            let line = line.expect("Unable to read line");
            line.parse().expect("Unable to parse value from line")
        })
        .collect()
}
