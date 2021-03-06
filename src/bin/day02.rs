use advent::{get_my_lines, iter_lines, InputError};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
    aim: u32,
}

#[derive(Debug)]
enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0, aim: 0 }
    }

    fn update(&mut self, dir: &Direction) {
        match dir {
            Direction::Forward(units) => self.x += units,
            Direction::Up(units) => self.y -= units,
            Direction::Down(units) => self.y += units,
        };
    }

    fn update_aimed(&mut self, dir: &Direction) {
        match dir {
            Direction::Forward(units) => {
                self.x += units;
                self.y += units * self.aim;
            }
            Direction::Up(units) => self.aim -= units,
            Direction::Down(units) => self.aim += units,
        };
    }
}

impl FromStr for Direction {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(forward|up|down) (\d+)")?;
        let cap = re.captures(s).ok_or(InputError)?;
        let direction = cap.get(1).ok_or(InputError)?.as_str();
        let units = cap.get(2).ok_or(InputError)?.as_str().parse::<u32>()?;
        match direction {
            "forward" => Ok(Self::Forward(units)),
            "up" => Ok(Self::Up(units)),
            "down" => Ok(Self::Down(units)),
            _ => Err(InputError),
        }
    }
}

fn follow_directions() -> u32 {
    let mut pos = Position::new();
    get_my_lines!().for_each(|line| pos.update(&line.parse::<Direction>().unwrap()));
    pos.x * pos.y
}

fn follow_directions_better() -> u32 {
    let mut pos = Position::new();
    get_my_lines!().for_each(|line| pos.update_aimed(&line.parse::<Direction>().unwrap()));
    pos.x * pos.y
}

fn main() {
    println!("Final multiple: {}", follow_directions());
    println!("Aimed multiple: {}", follow_directions_better());
}
