fn main() {
	use std::fs::File;
	use std::io::prelude::*;
	use std::str::FromStr;

	let mut args = std::env::args();
	let num_santas = if args.len() > 1 {
		i32::from_str(&args.nth(1).unwrap()).unwrap()
	} else {
		1
	};

	println!("Number of santas {:?}", num_santas);

	let mut f = File::open("input.txt").unwrap();
	let mut buffer = String::new();

	f.read_to_string(&mut buffer).unwrap();

	let distinct_houses = count_distinct_houses(&buffer, num_santas);
	println!("Distinct houses visisted: {}", distinct_houses);
}

fn count_distinct_houses(directions: &str, num_santas: i32) -> i32 {
	use std::collections::HashMap;

	let mut grid = HashMap::new();
	let mut santas = Vec::new();

	for _ in 0..num_santas {
		santas.push((0,0));
	}

	grid.insert((0,0), num_santas);

	let mut current_santa = 0;

	for direction in directions.chars() {

		let current_pos = santas.pop().unwrap();

		let (x, y) = current_pos;
		let next = match direction {
			'^' => Some((x, y + 1)),
			'<' => Some((x - 1, y)),
			'>' => Some((x + 1, y)),
			'v' => Some((x, y - 1)),
			_   => None,
		};

		match next {
			Some(pos) => {
				// Update the accumulator
				santas.insert(0, pos);	// Need to insert at beginning instead of push to end.

				// Update the grid
				if !grid.contains_key(&pos) {
					grid.insert(pos, 1);
				} else {
					if let Some(x) = grid.get_mut(&pos) {
						*x = *x + 1;
					}
				}
			}
			None => {}
		};

		if current_santa + 1 >= santas.len() {
			current_santa = 0;
		} else {
			current_santa = current_santa + 1;
		}
	}

	grid.len() as i32
}

#[test]
fn count_distinct_houses_test() {
	let num_santas = 1;

	let input1 = "^v";
	let expect1 = 2;
	assert_eq!(expect1, count_distinct_houses(input1, num_santas));

	let input2 = "^>v<";
	let expect2 = 4;
	assert_eq!(expect2, count_distinct_houses(input2, num_santas));

	let input3 = "^v^v^v^v^v";
	let expect3 = 2;
	assert_eq!(expect3, count_distinct_houses(input3, num_santas));
}

#[test]
fn count_distinct_houses_multisanta_test() {
	let num_santas = 2;

	let input1 = "^v";
	let expect1 = 3;
	assert_eq!(expect1, count_distinct_houses(input1, num_santas));

	let input2 = "^>v<";
	let expect2 = 3;
	assert_eq!(expect2, count_distinct_houses(input2, num_santas));

	let input3 = "^v^v^v^v^v ";
	let expect3 = 11;
	assert_eq!(expect3, count_distinct_houses(input3, num_santas));
}