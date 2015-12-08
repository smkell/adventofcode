#![feature(plugin)]
#![plugin(regex_macros)]

/// # Day 7: Some Assembly Required
/// 
/// ## Part One
/// 
/// This year, Santa brought little Bobby Tables a set of wires and *bitwise* logic gates! Unfortunately, 
/// little Bobby is a little under the recommended age range, and he needs help assembling the circuit.
/// 
/// Each wire has an identifier (some lowercase letters) and can carry a *16-bit* signal (a number from 0 
/// to 65535). A signal is provided to each wire by a gate, another wire, or some specific value. Each 
/// wire can only get a signal from one source, but can provide its signal to multiple destinations. A 
/// gate provides no signal until all of its inputs have a signal.
/// 
/// The included instructions booklet describes how to connect the parts together: `x AND y -> z` means to 
/// connect wires `x` and `y` to an AND gate, and then connect its output to wire `z`.
/// 
/// For example:
/// 
/// * `123 -> x` means that the signal `123` is provided to wire `x`.
/// 
/// * `x AND y -> z` means that the *bitwise AND* of wire `x` and wire `y` is provided to wire `z`.
/// 
/// * `p LSHIFT 2 -> q` means that the value from wire `p` is *left-shifted* by `2` and then provided to 
/// wire `q`.
/// 
/// * `NOT e -> f` means that the *bitwise complement* of the value from wire `e` is provided to wire `f`.
/// 
/// Other possible gates include `OR` (*bitwise OR*) and `RSHIFT` (*right-shift*). If, for some reason, 
/// you'd like to _emulate_ the circuit instead, almost all programming languages (for example, *C*, 
/// *JavaScript*, or *Python*) provide operators for these gates.
/// 
/// For example, here is a simple circuit:
/// 
/// 	123 -> x
/// 	456 -> y
/// 	x AND y -> d
/// 	x OR y -> e
/// 	x LSHIFT 2 -> f
/// 	y RSHIFT 2 -> g
/// 	NOT x -> h
/// 	NOT y -> i
/// 
/// After it is run, these are the signals on the wires:
/// 
/// 	d: 72
/// 	e: 507
/// 	f: 492
/// 	g: 114
/// 	h: 65412
/// 	i: 65079
/// 	x: 123
/// 	y: 456
/// 
/// In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is 
/// ultimately provided to _wire_ `a`?

mod parser;

use std::collections::HashMap;

fn main() {
	use std::fs::File;
	use std::io::prelude::*;

	let mut f = File::open("input.txt").unwrap();
	let mut buffer = String::new();

	f.read_to_string(&mut buffer).unwrap();

	let registers = run_program(&buffer);
	println!("The value in register 'a' after running the program is {:?}", registers.get(&"a").unwrap());
}

/// Runs a program.
///
/// # Notes
/// We need to implement dependncy resolution such that we verify that instructions are executed in 
/// the correct order. We can do this by sorting by the destination of each instruction.
///
/// Instead of iterating over the instructions we should pop the top instruction off the stack. If 
/// it's a Load instruction then execute it. If it's any other instruction then check to see if the 
/// neccesary registers are initialized, if they are not then push the instruction back on the stack,
/// and so on until all instructions have been resolved.
fn run_program(input: &str) -> HashMap<&str, u16> {
	use parser::{Instruction, parse_input};
	let mut instructions = parse_input(input);
	instructions.reverse();

	let mut registers = HashMap::new();

	while instructions.len() > 0 {
		let instruction = instructions.pop().unwrap();

		println!("Processing instruction {:?}", instruction);
		match instruction {
			Instruction::Load(d, c) => {
				registers.insert(d, c);
			},
			Instruction::And(d, x, y) => {
				// Check to see if we've initialized the neccesary 
				// registers
				if registers.contains_key(&x) && registers.contains_key(&y) {
					// OK! we can run the instruction
					let val_x = *registers.get(&x).unwrap();
					let val_y = *registers.get(&y).unwrap();

					registers.insert(d, val_x & val_y);
				} else {
					instructions.insert(0, instruction);
				}
			},
			Instruction::Or(d, x, y) => {
				// Check to see if we've initialized the neccesary 
				// registers
				if registers.contains_key(&x) && registers.contains_key(&y) {
					// OK! we can run the instruction
					let val_x = *registers.get(&x).unwrap();
					let val_y = *registers.get(&y).unwrap();

					registers.insert(d, val_x | val_y);
				} else {
					instructions.insert(0, instruction);
				}
			},
			Instruction::LShift(d, x, c) => {
				// Check to see if we've initialized the neccesary 
				// registers
				if registers.contains_key(&x) {
					// OK! we can run the instruction
					let val_x = *registers.get(&x).unwrap();

					registers.insert(d, val_x << c);
				} else {
					instructions.insert(0, instruction);
				}
			},
			Instruction::RShift(d, x, c) => {
				// Check to see if we've initialized the neccesary 
				// registers
				if registers.contains_key(&x) {
					// OK! we can run the instruction
					let val_x = *registers.get(&x).unwrap();

					registers.insert(d, val_x >> c);
				} else {
					instructions.insert(0, instruction);
				}
			},
			Instruction::Not(d, x) => {
				// Check to see if we've initialized the neccesary 
				// registers
				if registers.contains_key(&x) {
					// OK! we can run the instruction
					let val_x = *registers.get(&x).unwrap();

					registers.insert(d, !val_x);
				} else {
					instructions.insert(0, instruction);
				}
			},
		}
		println!("Register state {:?}", registers);
	}

	registers
}

#[test]
fn run_program_test() {
	let program = 
		r"123 -> x
			456 -> y
			x AND y -> d
			x OR y -> e
			x LSHIFT 2 -> f
			y RSHIFT 2 -> g
			NOT x -> h
			NOT y -> i";
	let mut expect = HashMap::new();
	expect.insert("d", 72);
	expect.insert("e", 507);
	expect.insert("f", 492);
	expect.insert("g", 114);
	expect.insert("h", 65412);
	expect.insert("i", 65079);
	expect.insert("x", 123);
	expect.insert("y", 456);

	println!("Running program {}", program);

	assert_eq!(expect, run_program(program));

	let program = 
		r"b RSHIFT 5 -> f
		e AND f -> h
		b RSHIFT 3 -> e
		44430 -> b";
	let mut expect = HashMap::new();
	expect.insert("b", 44430);
	expect.insert("e", 44430 >> 3);
	expect.insert("f", 44430 >> 5);
	expect.insert("h", (44430 >> 3) & (44430 >> 5));

	println!("Running program {}", program);
	assert_eq!(expect, run_program(program));
}