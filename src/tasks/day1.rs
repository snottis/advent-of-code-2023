use std::{fs::File, io::BufRead, io::BufReader, sync::Arc};

use once_cell::sync::Lazy;
use regex::{Match, Matches, Regex, RegexSet};

pub fn solve_day_01(input: File, level: u8) -> u64 {
    if (level == 1) {
        BufReader::new(input)
            .lines()
            .fold(0, |acc, line| acc + get_value_from_line(line.unwrap()))
    } else if (level == 2) {
        return solve_day_01_2(input);
    } else {
        panic!("Invalid level")
    }
}

pub fn solve_day_01_2(input: File) -> u64 {
    BufReader::new(input)
        .lines()
        .fold(0, |acc, line| acc + get_value_from_line_2(line.unwrap()))
}

fn get_value_from_line_2(line: String) -> u64 {
    const PATTERNS: [&str; 10] = [
        r"one", r"two", r"three", r"four", r"five", r"six", r"seven", r"eight", r"nine", r"[1-9]",
    ];
    static set: Lazy<RegexSet> = Lazy::new(|| RegexSet::new(&PATTERNS).unwrap());
    static regexes: Lazy<Vec<Regex>> = Lazy::new(|| {
        set.patterns()
            .iter()
            .map(|pat| Regex::new(pat).unwrap())
            .collect()
    });
    let mut matches: Vec<Match> = regexes
        .iter()
        .flat_map(|re| re.find_iter(line.as_str()).collect::<Vec<Match>>())
        .collect::<Vec<Match>>();
    matches.sort_by(|a, b| a.start().cmp(&b.start()));
    let first = parse_num(matches.first().unwrap().as_str());
    let second = parse_num(matches.last().unwrap().as_str());
    format!("{}{}", first, second).parse::<u64>().unwrap()
}

fn parse_num(string: &str) -> u64 {
    match string {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => string.parse::<u64>().unwrap(),
    }
}

fn get_value_from_line(line: String) -> u64 {
    let characters = line.chars().into_iter();
    let character_reverse = characters.clone().rev();
    let first = find_num_from_char_iter(characters);
    let second = find_num_from_char_iter(character_reverse);
    format!("{}{}", first, second).parse::<u64>().unwrap()
}

fn find_num_from_char_iter(mut chars: impl Iterator<Item = char>) -> char {
    loop {
        let character = chars.next();
        match character {
            None => panic!("Invalid input"),
            Some(char) => {
                if char.is_numeric() {
                    break char;
                }
            }
        }
    }
}
