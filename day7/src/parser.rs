/// Provides a parser for the Advent Assembly Language.

#[derive(Copy, Clone, Debug, PartialEq)]
enum Instruction {
	And(char, char, char),
	Load(char, u32)
}

fn parse_input(input: &str) -> Vec<Instruction> {
	let tokens = tokenize_input(input);

	let mut instructions: Vec<Instruction> = Vec::new();

	for i in 0..tokens.len() {
		let token = tokens[i];

		match token {
			Token::Constant(c) => {
				// Make sure there's enough tokens available
				if i < tokens.len() - 2 {
					match tokens[i+1] {
						Token::Assign => {
							match tokens[i+2] {
								Token::Wire(w) => {
									instructions.push(Instruction::Load(w, c));
								}
								_ => println!("Expression not currently implemented")
							}
						},
						_ => println!("Expression not currently implemented")
					}
				}
			}
			_ => println!("Expression not currently implemented")
		}
	}
	instructions
}

#[test]
fn parse_input_test() {
	let test_cases = vec!(
		("123 -> x", vec!(Instruction::Load('x', 123))),
		("456 -> y", vec!(Instruction::Load('y', 456))),
		("x AND y -> d", vec!(Instruction::And('d', 'x', 'y')))
	);

	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, ref expect) = *test_case;

		println!("Test case #{:?}: input = {:?} expect = {:?}", 
			i, input, expect);

		assert_eq!(*expect, parse_input(input));
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Token {
	And,
	Assign,
	Constant(u32),
	LShift,
	Not,
	Or,
	RShift,
	Wire(char)
}

fn tokenize_input(input: &str) -> Vec<Token> {
	use std::str::FromStr;

	let mut tokens: Vec<Token> = Vec::new();

	let sub_strings: Vec<&str> = input.split_whitespace().collect();

	for sub_string in sub_strings.clone() {
		if sub_string.chars().all(|c| c.is_digit(10)) {
			// We've got a constant so let's create the token 
			// for it 
			let value = u32::from_str(sub_string).unwrap();
			tokens.push(Token::Constant(value));
		} else if sub_string == "->" {
			tokens.push(Token::Assign);
		} else if sub_string == "AND" {
			tokens.push(Token::And);
		} else if sub_string == "OR" {
			tokens.push(Token::Or);
		} else if sub_string == "LSHIFT" {
			tokens.push(Token::LShift);
		} else if sub_string == "RSHIFT" {
			tokens.push(Token::RShift);
		} else if sub_string == "NOT" {
			tokens.push(Token::Not);
		} else if sub_string.len() == 1 {
			let c = sub_string.chars().nth(0).unwrap();
			tokens.push(Token::Wire(c));
		}
	}

	tokens
}

#[test]
fn tokenize_input_test() {
	let test_cases = vec!(
		("123 -> x", vec!(Token::Constant(123), Token::Assign, Token::Wire('x'))),
		("456  -> y", vec!(Token::Constant(456), Token::Assign, Token::Wire('y'))),
		("x AND y -> d", vec!(Token::Wire('x'), Token::And, Token::Wire('y'), Token::Assign, Token::Wire('d'))),
		("x OR y -> e", vec!(Token::Wire('x'), Token::Or, Token::Wire('y'), Token::Assign, Token::Wire('e'))),
		("x LSHIFT 2 -> f", vec!(Token::Wire('x'), Token::LShift, Token::Constant(2), Token::Assign, Token::Wire('f'))),
		("y RSHIFT 2 -> g", vec!(Token::Wire('y'), Token::RShift, Token::Constant(2), Token::Assign, Token::Wire('g'))),
		("NOT x -> h", vec!(Token::Not, Token::Wire('x'), Token::Assign, Token::Wire('h'))),
		("NOT y -> i", vec!(Token::Not, Token::Wire('y'), Token::Assign, Token::Wire('i'))),
	);

	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, ref expect) = *test_case;

		println!("Test case #{:?}: input = {:?} expect = {:?}", 
			i, input, expect);

		assert_eq!(*expect, tokenize_input(input));
	}
}