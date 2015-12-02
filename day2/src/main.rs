#![feature(slice_patterns)]

fn main() {
	use std::fs::File;
	use std::io::prelude::*;

	let mut f = File::open("input.txt").unwrap();
	let mut buffer = String::new();

	f.read_to_string(&mut buffer).unwrap();

	let mut total_paper = 0;
	let mut total_ribbon = 0;
	for input in buffer.lines() {
		let dims = parse_dimensions(input);
		match dims {
			Some((l,w,h)) => {
				total_paper = total_paper + paper_required(l, w, h);
				total_ribbon = total_ribbon + ribbon_required(l, w, h);
			},
			None => {},
		}
	}

	println!("Total paper required: {}", total_paper);
	println!("Total ribbon required: {}", total_ribbon);
}

fn parse_dimensions(dims: &str) -> Option<(i32, i32, i32)> {
	use std::str::FromStr;

	let dims: Vec<&str> = dims.split('x').collect();

	if dims.len() != 3 {
		None 
	} else {
		let length_result = i32::from_str(dims[0]);
		let width_result = i32::from_str(dims[1]);
		let height_result = i32::from_str(dims[2]);

		match (length_result, width_result, height_result) {
			(Ok(l), Ok(w), Ok(h)) => Some((l, w, h)),
			_ => None
		}
	}
}

fn paper_required(l: i32, w: i32, h: i32) -> i32 {
	let area1 = l * w;
	let area2 = w * h;
	let area3 = h * l;

	let min_area = min(area1, min(area2, area3));

	(2 * area1) + (2 * area2) + (2 * area3) + min_area
}

fn ribbon_required(l: i32, w: i32, h: i32) -> i32 {
	let perim1 = (2 * l) + (2 * w);
	let perim2 = (2 * w) + (2 * h);
	let perim3 = (2 * h) + (2 * l);

	let vol = l * w * h;

	let min_perim = min(perim1, min(perim2, perim3));

	min_perim + vol
}

fn min(first: i32, second: i32) -> i32 {
	if first < second {
		first
	} else {
		second
	}
}
