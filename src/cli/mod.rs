use clap::{Arg, ArgMatches, Command};
pub fn get_matches() -> ArgMatches {
    let matches = Command::new("aoc")
        .about("Advent of Code CLI")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Sauli Nevalainen")
        .subcommand(fetch_subcommand())
        .subcommand(solve_subcommand())
        .subcommand(submit_subcommand())
        .get_matches();
    return matches;
}

fn fetch_subcommand() -> Command {
    Command::new("fetch")
        .about("Fetches input for a given day")
        .arg(
            Arg::new("day")
                .help("The day to fetch input for")
                .required(true)
                .index(1),
        )
}

fn solve_subcommand() -> Command {
    Command::new("solve")
        .about("Solves a given day")
        .arg(
            Arg::new("day")
                .help("The day to solve")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("level")
                .help("The level to solve")
                .default_value("1")
                .index(2),
        )
}

fn submit_subcommand() -> Command {
    Command::new("submit")
        .about("Submits a given day")
        .arg(
            Arg::new("day")
                .help("The day to submit")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("level")
                .help("The level to solve")
                .default_value("1")
                .index(2),
        )
}
