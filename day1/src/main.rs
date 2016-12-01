extern crate regex;

use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
enum Direction {
	North,
	East,
	South,
	West
}

#[derive(Debug, Clone)]
struct Position {
	north: i64,
	west: i64,
	direction: Direction
}

impl Position {
	fn new() -> Position {
		return Position{
			north: 0,
			west: 0,
			direction: Direction::North
		}
	}

	fn turn_right(&mut self) {
		self.direction = match self.direction {
			Direction::North => Direction::East,
			Direction::East => Direction::South,
			Direction::South => Direction::West,
			Direction::West => Direction::North,
		}
	}

	fn turn_left(&mut self) {
		self.direction = match self.direction {
			Direction::North => Direction::West,
			Direction::East => Direction::North,
			Direction::South => Direction::East,
			Direction::West => Direction::South,
		}
	}

	fn go(&mut self, blocks: i64) {
		match self.direction {
			Direction::North => self.north += blocks,
			Direction::East => self.west -= blocks,
			Direction::South => self.north -= blocks,
			Direction::West => self.west += blocks,
		}
	}

	fn distance(&self) -> i64 {
		self.north.abs() + self.west.abs()
	}
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.north == other.north && self.west == other.west
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.north.hash(state);
        self.west.hash(state);
    }
}

fn main() {
	use std::io::prelude::*;
	use std::fs::File;
	use regex::Regex;
	use std::collections::HashSet;

	let mut f = File::open("data.txt").unwrap();
	let mut input = String::new();
	f.read_to_string(&mut input).unwrap();

	let re = Regex::new(r"([LR])(\d+)").unwrap();

	let mut pos = Position::new();

	let mut visited = HashSet::new();
	visited.insert(pos.clone());

	for cap in re.captures_iter(&input) {
		let turn = cap.at(1).unwrap();
		let blocks = cap.at(2).unwrap().parse::<i64>().unwrap();

		match turn {
			"L" => pos.turn_left(),
			"R" => pos.turn_right(),
			_ => panic!("Bad Input"),
		}

		for _ in 0..blocks {
			pos.go(1);

			if visited.contains(&pos) {
				println!("Found Repeat at {:?}, {} blocks away.", pos, pos.distance());
				return;
			} else {
				visited.insert(pos.clone());
			}
		}

		println!("Direction: {}\tBlocks: {},\tPosition: {:?}", turn, blocks, pos);
	}

	println!("Path Never Overlapped, ended at {:?}, {} blocks away.", pos, pos.distance());
}
