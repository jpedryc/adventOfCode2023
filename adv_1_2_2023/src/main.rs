use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use aho_corasick::AhoCorasick;

fn main() -> io::Result<()> {
    // Load source file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Create a regex splitter by words
    //one|two|three|four|five|six|seven|eight|nine
    let regex_splitter = Regex::new(r"[a-z]+").unwrap();
    let mut sum: u32 = 0;

    let patterns = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let replacers = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let ac = AhoCorasick::new(patterns).unwrap();

    for line in reader.lines() {
        let current_line = line?;

        let conv_line = ac.replace_all(current_line.as_str(), replacers);

        let splitted: Vec<String> = regex_splitter
            .split(conv_line.as_str())
            .map(|l| l.to_string())
            .filter(|s| s.ne(""))
            .collect();

        let first_char = splitted.first().unwrap().chars().nth(0).unwrap();
        let last_char = splitted.last().unwrap().chars().nth(splitted.last().unwrap().len() - 1).unwrap();

        let mut char_int = String::from(first_char);
        char_int.push(last_char);

        println!("{:?} -> {:?} -> {:?}", current_line, conv_line, char_int);

        sum += char_int.parse::<u32>().unwrap();
    }

    println!("{:?}", sum);

    Ok(())
}
