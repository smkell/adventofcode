/// Provides a parser for the Advent Assembly Language.

/// A Token represents a gramattical token in a program.
/// 
/// # Lifetimes
/// 
/// * `'a` - Represents the lifetime of the Token object.
#[derive(Copy, Clone, Debug, PartialEq)]
enum Token<'a> {
	Assign,
	And,
	Constant(u16),
	LShift,
	Not,
	Or,
	RShift,
	Wire(&'a str),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction<'a> {
	And(&'a str, &'a str, &'a str),
	AndWC(&'a str, &'a str, u16),
	AndCW(&'a str, u16, &'a str),
	Load(&'a str, u16),
	LoadW(&'a str, &'a str),
	LShift(&'a str, &'a str, u16),
	Not(&'a str, &'a str),
	Or(&'a str, &'a str, &'a str),
	OrWC(&'a str, &'a str, u16),
	OrCW(&'a str, u16, &'a str),
	RShift(&'a str, &'a str, u16),
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
	let tokens = tokenize_input(input);

	let mut instructions = Vec::new();

	let mut start = 0;
	while start < tokens.len() {
		match parse_expression(&tokens[start..]) {
			Some((instruction, tokens_consumed)) => {
				instructions.push(instruction);
				start = start + tokens_consumed;
			},
			None => {
				println!("Failed to parse tokens: {:?}", &tokens[start..]);
				break
			}
		}
	}

	instructions
}

#[test]
fn parse_input_test() {
	let test_cases = vec!(
		("123 -> x", vec!(Instruction::Load("x", 123))),
		("123 -> x\n456 -> y", vec!(Instruction::Load("x", 123), Instruction::Load("y", 456))),
		("lx -> a", vec!(Instruction::LoadW("a", "lx"))),
		("x AND y -> d", vec!(Instruction::And("d", "x", "y"))),
		("x AND 1 -> d", vec!(Instruction::AndWC("d", "x", 1))),
		("1 AND x -> d", vec!(Instruction::AndCW("d", 1, "x"))),
		("x OR y -> e",  vec!(Instruction::Or("e", "x", "y"))),
		("x OR 1 -> e",  vec!(Instruction::OrWC("e", "x", 1))),
		("1 OR x -> e",  vec!(Instruction::OrCW("e", 1, "x"))),
		("x LSHIFT 2 -> f", vec!(Instruction::LShift("f", "x", 2))),
		("y RSHIFT 2 -> g", vec!(Instruction::RShift("g", "y", 2))),
		("NOT x -> h", vec!(Instruction::Not("h", "x"))),
		("NOT y -> i", vec!(Instruction::Not("i", "y"))),
		(
			r"123 -> x
			456 -> y
			lx -> a
			x AND y -> d
			x OR y -> e
			x LSHIFT 2 -> f
			y RSHIFT 2 -> g
			NOT x -> h
			NOT y -> i",
			vec!(
				Instruction::Load("x", 123),
				Instruction::Load("y", 456),
				Instruction::LoadW("a", "lx"),
				Instruction::And("d", "x", "y"),
				Instruction::Or("e", "x", "y"),
				Instruction::LShift("f", "x", 2),
				Instruction::RShift("g", "y", 2),
				Instruction::Not("h", "x"),
				Instruction::Not("i", "y"),
			)
		)
	);

	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, ref expect) = *test_case;

		println!("Test case #{:?}: input = {:?} expect = {:?}", i, input, expect);
		assert_eq!(*expect, parse_input(input));
	}
}

fn parse_expression<'a>(tokens: &[Token<'a>]) -> Option<(Instruction<'a>, usize)> {
	match tokens[0] {
		Token::Constant(c) => match tokens[1] {
			Token::Assign => match parse_constant_assign(c, &tokens[1..]) {
				Some((instruction, tokens_consumed)) => {
					Some((instruction, tokens_consumed + 1))
				},
				None => None,
			},
			Token::And => match parse_and_constant(c, &tokens[2..]) {
				Some((instruction, tokens_consumed)) => {
					Some((instruction, tokens_consumed + 2))
				},
				None => None,
			},
			Token::Or => match parse_or_constant(c, &tokens[2..]) {
				Some((instruction, tokens_consumed)) => {
					Some((instruction, tokens_consumed + 2))
				},
				None => None,
			},
			_ => None,
		},
		Token::Wire(w) => match tokens[1] {
			Token::Assign => match parse_wire_assign(w, &tokens[2..]) {
				Some((instruction, tokens_consumed)) => {
					Some((instruction, tokens_consumed + 1))
				},
				None => None,
			},
			Token::And => match parse_and_wire(w, &tokens[2..]) {
				Some((instruction, tokens_consumed)) => {
					Some((instruction, tokens_consumed + 2))
				},
				None => None,
			},
			Token::Or => match parse_or_wire(w, &tokens[2..]) {
				Some((instruction, tokens_consumed)) => {
					Some((instruction, tokens_consumed + 2))
				},
				None => None,
			},
			Token::LShift => match parse_lshift(w, &tokens[2..]) {
				Some((instruction, tokens_consumed)) => {
					Some((instruction, tokens_consumed + 2))
				},
				None => None,
			},
			Token::RShift => match parse_rshift(w, &tokens[2..]) {
				Some((instruction, tokens_consumed)) => {
					Some((instruction, tokens_consumed + 2))
				},
				None => None,
			},
			_ => None
		},
		Token::Not => match parse_not(&tokens[1..]){
			Some((instruction, tokens_consumed)) => {
				Some((instruction, tokens_consumed + 1))
			}, 
			None => None
		},
		_ => None
	}
}

fn parse_constant_assign<'a>(c: u16, tokens: &[Token<'a>]) -> Option<(Instruction<'a>, usize)> {
	match tokens[0] {
		Token::Assign => match tokens[1] {
			Token::Wire(w) => {
				Some((Instruction::Load(w, c), 2))
			},
			_ => None,
		},
		_ => None
	}
}

fn parse_wire_assign<'a>(w1: &'a str, tokens: &[Token<'a>]) -> Option<(Instruction<'a>, usize)> {
	match tokens[0] {
		Token::Wire(w2) => {
			Some((Instruction::LoadW(w2, w1), 2))
		},
		_ => None,
	}
}

fn parse_and_constant<'a>(c: u16, tokens: &[Token<'a>]) -> Option<(Instruction<'a>, usize)> {
	match tokens[0] {
		Token::Wire(w) => match tokens[1] {
			Token::Assign => match tokens[2] {
				Token::Wire(d) => Some((Instruction::AndCW(d, c, w), 3)),
				_ => None,
			},
			_ => None,
		},
		_ => None,
	}
}

fn parse_and_wire<'a>(w1: &'a str, tokens: &[Token<'a>]) -> Option<(Instruction<'a>, usize)> {
	match tokens[0] {
		Token::Wire(w2) => match tokens[1] {
			Token::Assign => match tokens[2] {
				Token::Wire(d) => Some((Instruction::And(d, w1, w2), 3)),
				_ => None,
			},
			_ => None,
		},
		Token::Constant(c) => match tokens[1] {
			Token::Assign => match tokens[2] {
				Token::Wire(d) => Some((Instruction::AndWC(d, w1, c), 3)),
				_ => None,
			},
			_ => None,
		},
		_ => None,
	}
}

fn parse_or_constant<'a>(c: u16, tokens: &[Token<'a>]) -> Option<(Instruction<'a>, usize)> {
	match tokens[0] {
		Token::Wire(w) => match tokens[1] {
			Token::Assign => match tokens[2] {
				Token::Wire(d) => Some((Instruction::OrCW(d, c, w), 3)),
				_ => None,
			},
			_ => None,
		},
		_ => None,
	}
}

fn parse_or_wire<'a>(w1: &'a str, tokens: &[Token<'a>]) -> Option<(Instruction<'a>, usize)> {
	match tokens[0] {
		Token::Wire(w2) => match tokens[1] {
			Token::Assign => match tokens[2] {
				Token::Wire(d) => Some((Instruction::Or(d, w1, w2), 3)),
				_ => None,
			},
			_ => None,
		},
		Token::Constant(c) => match tokens[1] {
			Token::Assign => match tokens[2] {
				Token::Wire(d) => Some((Instruction::OrWC(d, w1, c), 3)),
				_ => None,
			},
			_ => None,
		},
		_ => None,
	}
}

fn parse_lshift<'a>(w1: &'a str, tokens: &[Token<'a>]) -> Option<(Instruction<'a>, usize)> {
	match tokens[0] {
		Token::Constant(c) => match tokens[1] {
			Token::Assign => match tokens[2] {
				Token::Wire(d) => Some((Instruction::LShift(d, w1, c), 3)),
				_ => None,
			},
			_ => None,
		},
		_ => None,
	}
}

fn parse_rshift<'a>(w1: &'a str, tokens: &[Token<'a>]) -> Option<(Instruction<'a>, usize)> {
	match tokens[0] {
		Token::Constant(c) => match tokens[1] {
			Token::Assign => match tokens[2] {
				Token::Wire(d) => Some((Instruction::RShift(d, w1, c), 3)),
				_ => None,
			},
			_ => None,
		},
		_ => None,
	}
}

fn parse_not<'a>(tokens: &[Token<'a>]) -> Option<(Instruction<'a>, usize)> {
	match tokens[0] {
		Token::Wire(w) => match tokens[1] {
			Token::Assign => match tokens[2] {
				Token::Wire(d) => Some((Instruction::Not(d, w), 3)),
				_ => None,
			},
			_ => None,
		},
		_ => None,
	}
}

#[test]
fn parse_expression_test() {
	let input = vec!(Token::Constant(123), Token::Assign, Token::Wire("x"));
	let expect = Instruction::Load("x", 123);
	let (actual, tokens_consumed) = parse_expression(&input[..]).unwrap();
	assert_eq!(expect, actual);
	assert_eq!(3, tokens_consumed);

	let input = vec!(
		Token::Constant(123), Token::Assign, Token::Wire("x"),
		Token::Constant(456), Token::Assign, Token::Wire("y"),
	);
	let expect = Instruction::Load("x", 123);
	let start = 0;
	let (actual, tokens_consumed) = parse_expression(&input[start..]).unwrap();
	assert_eq!(expect, actual);
	assert_eq!(3, tokens_consumed);

	let expect = Instruction::Load("y", 456);
	let (actual, tokens_consumed) = parse_expression(&input[start+tokens_consumed..]).unwrap();
	assert_eq!(expect, actual);
	assert_eq!(3, tokens_consumed);
}
                                                                        
fn tokenize_input(input: &str) -> Vec<Token> {
	use std::str::FromStr;											   
                                                                       
	let mut tokens: Vec<Token> = Vec::new();						   
	                                                                   
	let sub_strings: Vec<&str> = input.split_whitespace().collect();   
                                                                       
	for sub_string in sub_strings {                                    
		match sub_string {
			"->" => {
				tokens.push(Token::Assign);
			},
			"AND" => {
				tokens.push(Token::And);
			},
			"OR" => {
				tokens.push(Token::Or);
			},
			"LSHIFT" => {
				tokens.push(Token::LShift);
			},
			"RSHIFT" => {
				tokens.push(Token::RShift);
			},
			"NOT" => {
				tokens.push(Token::Not);
			},
			s => if s.chars().all(|c| c.is_digit(10)) {
				let constant = u16::from_str(s).unwrap();
				tokens.push(Token::Constant(constant));
			} else {
				let wire_name: &str = s.clone();
				tokens.push(Token::Wire(wire_name));
			},
		}
	}

	tokens
}

#[test]
fn tokenize_input_test() {
	let test_cases = vec!(
		("123 -> x", vec!(Token::Constant(123), Token::Assign, Token::Wire("x"))),
		("456  -> y", vec!(Token::Constant(456), Token::Assign, Token::Wire("y"))),
		("x AND y -> d", vec!(Token::Wire("x"), Token::And, Token::Wire("y"), Token::Assign, Token::Wire("d"))),
		("x OR y -> e", vec!(Token::Wire("x"), Token::Or, Token::Wire("y"), Token::Assign, Token::Wire("e"))),
		("x LSHIFT 2 -> f", vec!(Token::Wire("x"), Token::LShift, Token::Constant(2), Token::Assign, Token::Wire("f"))),
		("y RSHIFT 2 -> g", vec!(Token::Wire("y"), Token::RShift, Token::Constant(2), Token::Assign, Token::Wire("g"))),
		("NOT x -> h", vec!(Token::Not, Token::Wire("x"), Token::Assign, Token::Wire("h"))),
		("NOT y -> i", vec!(Token::Not, Token::Wire("y"), Token::Assign, Token::Wire("i"))),
	);

	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, ref expect) = *test_case;

		println!("Test case #{:?}: input = {:?} expect = {:?}", 
			i, input, expect);

		assert_eq!(*expect, tokenize_input(input));
	}
}