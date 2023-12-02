use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn submit(day: u16) -> () {
    println!("Submitting day {}", day);
}

pub fn get_input(day: u16) -> File {
    let metadata = fs::metadata(input_file_path(day));
    match metadata {
        Ok(_) => (),
        Err(_) => fetch_input(day),
    }
    let file = File::open(input_file_path(day)).unwrap();
    return file;
}

pub fn fetch_input(day: u16) -> () {
    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    let response = reqwest::blocking::Client::new()
        .get(&url)
        .header("Cookie", format!("session={}", get_session_key()))
        .send()
        .unwrap()
        .text()
        .unwrap();
    create_input_folder_if_not_exists();
    let mut file = File::create(input_file_path(day)).unwrap();
    file.write_all(response.as_bytes()).unwrap();
}

pub fn submit_answer(level: u8, day: u16, answer: String) -> () {
    let url = format!("https://adventofcode.com/2023/day/{}/answer", day);
    let response = reqwest::blocking::Client::new()
        .post(&url)
        .header("Cookie", format!("session={}", get_session_key()))
        .form(&[("level", level.to_string()), ("answer", answer)])
        .send()
        .unwrap()
        .text()
        .unwrap();
    println!("{}", response);
}

fn get_session_key() -> String {
    env::var("SESSION_KEY").expect("SESSION_KEY must be set")
}

fn input_file_path(day: u16) -> String {
    format!("inputs/day{:02}.txt", day)
}

fn create_input_folder_if_not_exists() -> () {
    fs::create_dir(Path::new("inputs")).unwrap_or_else(|error| {
        if error.kind() != std::io::ErrorKind::AlreadyExists {
            return;
        }
    });
}
