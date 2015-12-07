#![feature(iter_arith)]

/// # Day 6: Probably a Fire Hazard
/// 
/// Because your neighbors keep defeating you in the holiday house decorating contest year after 
/// year, you've decided to deploy one million lights in a 1000x1000 grid.
/// 
/// Furthermore, because you've been especially nice this year, Santa has mailed you instructions on 
/// how to display the ideal lighting configuration.
/// 
/// Lights in your grid are numbered from 0 to 999 in each direction; the lights at each corner are 
/// at `0,0`, `0,999`, `999,999`, and `999,0`. The instructions include whether to `turn on`, `turn off`, 
/// or `toggle` various inclusive ranges given as coordinate pairs. Each coordinate pair represents 
/// opposite corners of a rectangle, inclusive; a coordinate pair like `0,0 through 2,2` therefore 
/// refers to 9 lights in a 3x3 square. The lights all start turned off.
/// 
/// To defeat your neighbors this year, all you have to do is set up your lights by doing the 
/// instructions Santa sent you in order.
/// 
/// For example:
/// 
/// * `turn on 0,0 through 999,999` would turn on (or leave on) every light.
/// 
/// * `toggle 0,0 through 999,0` would toggle the first line of 1000 lights, turning off the ones that 
/// were on, and turning on the ones that were off.
/// 
/// * `turn off 499,499 through 500,500` would turn off (or leave off) the middle four lights.
/// 
/// After following the instructions, *how many lights are lit*?

extern crate regex;

fn main() {
	use std::fs::File;
	use std::io::prelude::*;

	let mut f = File::open("input.txt").unwrap();
	let mut buffer = String::new();

	f.read_to_string(&mut buffer).unwrap();

	let instructions = parse_program(&buffer);
	let lights_on = run_program(&instructions);
	println!("There should be {:?} lights on after running the program", lights_on);
}

type Coordinate = (u32,u32);

#[derive(Copy, Clone, PartialEq, Debug)]
enum Instructions {
	TurnOn(Coordinate, Coordinate),
	Toggle(Coordinate, Coordinate),
	TurnOff(Coordinate, Coordinate),
}

fn parse_program(input: &str) -> Vec<Instructions> {
	use regex::Regex;

	let re = Regex::new(r"(?x)
		(?P<instruction>turn\son|toggle|turn\soff)
		\s*
		(?P<start_begin_range>\d*)
		,
		(?P<end_begin_range>\d*)
		\s*
		through
		\s*
		(?P<start_end_range>\d*)
		,
		(?P<end_end_range>\d*)
		.*").unwrap();

	let mut instructions: Vec<Instructions> = Vec::new();
	for line in input.lines() {
		match re.captures(line) {
			Some(captures) => {
				let start_coord = parse_coordinate(
									captures.name("start_begin_range"),
									captures.name("end_begin_range"));

				let end_coord = parse_coordinate(
									captures.name("start_end_range"),
									captures.name("end_end_range"));

				match parse_instruction(captures.name("instruction"), start_coord, end_coord) {
					Some(i) => {
						instructions.push(i);
					},
					None => println!("Failed to parse line: {:?}", line)
				}
			},
			None => println!("Failed to parse line: {:?}", line)
		};
	}
	instructions
}

fn parse_instruction(
	instruction: Option<&str>, 
	start: Option<Coordinate>, 
	end: Option<Coordinate>) -> Option<Instructions> {

	match (instruction, start, end) {
		(Some(instruction_raw), Some(start_coord), Some(end_coord)) => {
			match instruction_raw {
				"turn on" => {
					Some(Instructions::TurnOn(start_coord, end_coord))
				},
				"turn off" => {
					Some(Instructions::TurnOff(start_coord, end_coord))
				},
				"toggle" => {
					Some(Instructions::Toggle(start_coord, end_coord))
				},
				_ => None
			}
		},
		_ => None
	}
}

fn parse_coordinate(start: Option<&str>, end: Option<&str>) -> Option<Coordinate> {
	use std::str::FromStr;

	match (start, end) {
		(Some(s), Some(e)) => {
			match (u32::from_str(s), u32::from_str(e)) {
				(Ok(x), Ok(y)) => Some((x, y)),
				_ => None
			}
		},
		_ => None
	}
}

#[test]
fn parse_program_test() {
	let test_case = "turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500";
	let expect = vec!(
		Instructions::TurnOn((0,0), (999,999)),
		Instructions::Toggle((0,0), (999,0)),
		Instructions::TurnOff((499,499),(500,500))
	);

	assert_eq!(expect, parse_program(test_case));
}

fn run_program(instructions: &[Instructions]) -> u32 {
	const WIDTH: usize = 1000;
	const HEIGHT: usize = 1000;

	let mut grid: Vec<u32> = vec!(0; WIDTH*HEIGHT);

	for instruction in instructions {
		match *instruction {
			Instructions::TurnOn((start_x, start_y), (end_x, end_y)) => {
				println!("Turning on lights from ({:?},{:?}) to ({:?},{:?})", 
					start_x, start_y, end_x, end_y);

				for y in start_y..end_y + 1 {
					for x in start_x..end_x + 1 {
						let i = (x + (y * WIDTH as u32)) as usize;
						grid[i] = grid[i] + 1;
					}
				}
			},
			Instructions::TurnOff((start_x, start_y), (end_x, end_y)) => {
				println!("Turning off lights from ({:?},{:?}) to ({:?},{:?})", 
					start_x, start_y, end_x, end_y);

				for y in start_y..end_y + 1 {
					for x in start_x..end_x + 1 {
						let i = (x + (y * WIDTH as u32)) as usize;
						if grid[i] > 0 {
							grid[i] = grid[i] - 1;
						}
					}
				}
			},
			Instructions::Toggle((start_x, start_y), (end_x, end_y)) => {
				println!("Toggling lights from ({:?},{:?}) to ({:?},{:?})", 
					start_x, start_y, end_x, end_y);

				for y in start_y..end_y + 1 {
					for x in start_x..end_x + 1{
						let i = (x + (y * WIDTH as u32)) as usize;
						grid[i] = grid[i] + 2;
					}
				}
			},
		}
	}

	grid.iter().sum::<u32>() as u32
}

#[test]
fn run_program_test() {
	let test_case = vec!(
		Instructions::TurnOn((0,0), (999,999)),
		Instructions::Toggle((0,0), (999,0)),
		Instructions::TurnOff((499,499),(500,500))
	);

	let expect = 998000;

	assert_eq!(expect, run_program(&test_case));
}
