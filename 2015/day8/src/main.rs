fn main() {
    use std::fs::File;
	use std::io::prelude::*;

	let mut f = File::open("input.txt").unwrap();
	let mut buffer = String::new();

	f.read_to_string(&mut buffer).unwrap();

	let mut code_count = 0;
	let mut str_count = 0;
	let mut encode_count = 0;
	for input in buffer.lines() {
		code_count = code_count + count_code_literals(input);
		str_count = str_count + count_string_literals(input);

		let escape = escape_str(input.to_string());
		encode_count = encode_count + (escape.len() as u32) + 2;
	} 

	println!("code_count - str_count = {} - {} = {}", code_count, str_count, code_count - str_count);
	println!("encode_count - code_count = {} - {} = {}", encode_count, code_count, encode_count - code_count);
}

fn escape_str(input: String) -> String {
	let mut buffer = String::new();

	for c in input.chars() {
		match c {
			'"' | '\\' => {
				buffer.push('\\');
				buffer.push(c);
			}
			_ => buffer.push(c),
		}
	}
	
	buffer
}

#[test]
fn escape_str_test() {
	let test_cases = vec!(
		(r#""""#, r#"\"\""#),
		(r#""abc""#, r#"\"abc\""#),
		(r#""aaa\"aaa""#, r#"\"aaa\\\"aaa\""#),
		(r#""\x27""#, r#"\"\\x27\""#),
	);

	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, expect) = *test_case;

		println!("Test Case #{:?}: input = {} expect = {}", i, input, expect);
		assert_eq!(expect.to_string(), escape_str(input.to_string()));
	}
}

fn count_code_literals(input: &str) -> u32 {
	input.len() as u32
}

#[test]
fn count_code_literals_test() {
	let test_cases = vec!(
		(r#""""#, 2),
		(r#""abc""#, 5),
		(r#""aaa\"aaa""#, 10),
		(r#""\x27""#, 6),
	);

	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, expect) = *test_case;

		println!("Test Case #{:?}: input = {} expect = {}", i, input, expect);
		assert_eq!(expect, count_code_literals(input));
	}
}

fn count_string_literals(input: &str) -> u32 {
	let mut i = 0;
	let mut count = 0;

	while i < input.len() {
		let (chars, chars_consumed) = parse_next(&input[i..]);
		i = i + chars_consumed as usize;
		count = count + chars;
	}

	count
}

fn parse_next(input: &str) -> (u32, u32) {
	let chars: Vec<char> = input.chars().collect();
	if chars.len() > 0 {
		match chars[0] {
			'"' => (0, 1),
			// Encountered an escape character 
			'\\' => match chars[1] {
				'\\' => (1, 2),
				'\'' => (1, 2),
				'\"' => (1, 2),
				'x' => match chars[2] {
					'0' ... '9' | 'a' ... 'z' | 'A' ... 'Z' => match chars[3] {
						'0' ... '9' | 'a' ... 'z' | 'A' ... 'Z' => (1, 4),
						_ => (1, 3),
					},
					_ => (1, 3),
				},
				_ => (1, 1),
			},
			_ => (1, 1),
		}
	} else {
		(0, 0)
	}
}

#[test]
fn count_string_literals_test() {
	let test_cases = vec!(
		(r#""""#, 0),
		(r#""abc""#, 3),
		(r#""aaa\"aaa""#, 7),
		(r#""\x27""#, 1),
	);

	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, expect) = *test_case;

		println!("Test Case #{:?}: input = {} expect = {}", i, input, expect);
		assert_eq!(expect, count_string_literals(input));
	}
}
