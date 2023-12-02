use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Game {
    id: u64,
    sets: Vec<GameSet>,
}

#[derive(Debug)]
struct GameSet {
    cubes: Vec<Cubes>,
}
#[derive(Debug)]
struct Cubes {
    colour: Colour,
    amount: u64,
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Colour {
    Red,
    Green,
    Blue,
}

const L1_RED: u64 = 12;
const L1_GREEN: u64 = 13;
const L1_BLUE: u64 = 14;

pub fn solve_day_02(input: File, level: u8) -> u64 {
    match level {
        1 => level_1(input),
        2 => level_2(input),
        _ => panic!("Level not implemented"),
    }
}

fn level_1(input: File) -> u64 {
    BufReader::new(input)
        .lines()
        .map(|line| parse_line(line.unwrap()))
        .filter(check_l1_possibility)
        .fold(0, |acc, game| acc + game.id)
}

fn level_2(input: File) -> u64 {
    BufReader::new(input)
        .lines()
        .map(|line| parse_line(line.unwrap()))
        .map(|game| {
            let mut colourmap: HashMap<Colour, Vec<u64>> = HashMap::new();
            colourmap.insert(Colour::Blue, vec![]);
            colourmap.insert(Colour::Green, vec![]);
            colourmap.insert(Colour::Red, vec![]);
            game.sets.iter().for_each(|set| {
                set.cubes.iter().for_each(|cube| {
                    let vec = colourmap.get_mut(&cube.colour).unwrap();
                    vec.push(cube.amount);
                })
            });
            let num = colourmap
                .into_values()
                .into_iter()
                .map(|vec| vec.into_iter().max().unwrap())
                .fold(1, |acc, next| acc * next);
            num
        })
        .fold(0, |acc, next| acc + next)
}

fn parse_line(line: String) -> Game {
    let mut game_and_cubes = line.split(": ").collect::<Vec<&str>>();
    let gameId = game_and_cubes[0]
        .split(" ")
        .last()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let sets = game_and_cubes[1].split("; ");
    let game = Game {
        id: gameId,
        sets: sets.map(|set| parse_set(set.to_string())).collect(),
    };
    return game;
}

fn parse_set(set: String) -> GameSet {
    let cubes = set.split(", ");
    let gameSet = GameSet {
        cubes: cubes.map(|cube| parse_cube(cube.to_string())).collect(),
    };
    return gameSet;
}

fn parse_cube(cube: String) -> Cubes {
    let colour_and_number = cube.split(" ").collect::<Vec<&str>>();
    let colour = match *colour_and_number.last().unwrap() {
        "red" => Colour::Red,
        "green" => Colour::Green,
        "blue" => Colour::Blue,
        _ => panic!("Invalid colour"),
    };
    Cubes {
        colour: colour,
        amount: colour_and_number.first().unwrap().parse::<u64>().unwrap(),
    }
}
fn check_l1_possibility<'a>(game: &'a Game) -> bool {
    game.sets.iter().all(|set| {
        set.cubes.iter().all(|cube| match cube.colour {
            Colour::Red => cube.amount <= L1_RED,
            Colour::Green => cube.amount <= L1_GREEN,
            Colour::Blue => cube.amount <= L1_BLUE,
        })
    })
}
