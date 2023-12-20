use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    // Load source file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Create a regex splitter by words
    let regex_splitter = Regex::new(r"[a-z]+").unwrap();
    let mut sum: u32 = 0;

    for line in reader.lines() {
        let splitted: Vec<String> = regex_splitter
            .split(line?.as_str())
            .map(|l| l.to_string())
            .filter(|s| s.ne(""))
            .collect();

        let first_char = splitted.first().unwrap().chars().nth(0).unwrap();
        let last_char = splitted.last().unwrap().chars().nth(splitted.last().unwrap().len() - 1).unwrap();

        let mut char_int = String::from(first_char);
        char_int.push(last_char);
        sum += char_int.parse::<u32>().unwrap();
    }

    println!("{:?}", sum);

    Ok(())
}
