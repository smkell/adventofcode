/// # Day 5: Doesn't He Have Intern-Elves For This? ---
/// 
/// ## Part One
/// 
/// Santa needs help figuring out which strings in his text file are naughty or nice.
/// 
/// A *nice string* is one with all of the following properties:
/// 
/// * It contains at least three vowels (`aeiou` only), like `aei`, `xazegov`, or `aeiouaeiouaeiou`.
/// 
/// * It contains at least one letter that appears twice in a row, like `xx`, `abcdde` (`dd`), or `aabbccdd` (`aa`, `bb`, `cc`, or `dd`).
/// 
/// * It does not contain the strings `ab`, `cd`, `pq`, or `xy`, even if they are part of one of the other requirements.
/// 
/// For example:
/// 
/// * `ugknbfddgicrmopn` is nice because it has at least three vowels (`u...i...o...`), a double letter (`...dd...`), and none of the disallowed substrings.
/// 
/// * `aaa` is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
/// 
/// * `jchzalrnumimnmhp` is naughty because it has no double letter.
/// 
/// * `haegwjzuvuyypxyu` is naughty because it contains the string xy.
/// 
/// * `dvszwmarrgswjxmb` is naughty because it contains only one vowel.
/// 
/// ## Part Two
/// 
/// Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.
/// 
/// Now, a nice string is one with all of the following properties:
/// 
/// * It contains a pair of any two letters that appears at least twice in the string without overlapping, like `xyxy` (`xy`) or `aabcdefgaa` (`aa`), but not like `aaa` (`aa`, but it overlaps).
/// 
/// * It contains at least one letter which repeats with exactly one letter between them, like `xyx`, `abcdefeghi` (`efe`), or even `aaa`.
/// 
/// For example:
/// 
/// * `qjhvhtzxzqqjkmpb` is nice because is has a pair that appears twice (`qj`) and a letter that repeats with exactly one letter between them (`zxz`).
/// 
/// * `xxyxx` is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
/// 
/// * `uurcxstgmygtbstg` is naughty because it has a pair (`tg`) but no repeat with a single letter between them.
/// 
/// * `ieodomkazucvgmuy` is naughty because it has a repeating letter with one between (`odo`), but no pair that appears twice.

fn main() {
	use std::fs::File;
	use std::io::prelude::*;

	let mut f = File::open("input.txt").unwrap();
	let mut buffer = String::new();

	f.read_to_string(&mut buffer).unwrap();
	let count = buffer.lines().filter(|line| is_nice(line, IsNiceAlgorithm::Version1)).count();
	println!("Found {:?} nice strings using algorithm {:?}", count, IsNiceAlgorithm::Version1);

	let count = buffer.lines().filter(|line| is_nice(line, IsNiceAlgorithm::Version2)).count();
	println!("Found {:?} nice strings using algorithm {:?}", count, IsNiceAlgorithm::Version2);
}

#[derive(Copy, Clone, Debug)]
enum IsNiceAlgorithm {
	Version1,
	Version2
}

fn is_nice(input: &str, algo: IsNiceAlgorithm) -> bool {
	match algo {
		IsNiceAlgorithm::Version1 => 
			(count_vowels(input) >= 3) 
				&& (count_repeat(input) >= 1)
				&& !(contains_naughty_strings(input)),
		IsNiceAlgorithm::Version2 =>
			count_pairs(input) >= 1 && count_repeat_overlap(input) >= 1,
	}
	
}

fn count_vowels(input: &str) -> u32 {
	let chars = input.chars();
	let vowel_count = chars.filter(|c| is_vowel(*c)).count() as u32;

	vowel_count
}

fn is_vowel(c: char) -> bool {
	match c {
		'a' | 'e' | 'i' | 'o' | 'u' => true,
		_ => false
	}
}

fn count_repeat(input: &str) -> u32 {
	let char_vec: Vec<char> = input.chars().collect();
	let chars: &[char] = &char_vec[..];

	if chars.len() <= 1 {
		return 0
	}

	let mut prev = chars[0];
	let mut count = 0;
	for current in &chars[1..] {
		if *current == prev {
			count = count + 1;
		}
		prev = *current;
	}

	count
}

fn contains_naughty_strings(input: &str) -> bool {
	let naughty_strings = vec!(
		"ab",
		"cd",
		"pq",
		"xy");

	naughty_strings.iter().any(|s| input.contains(s))
}

/// Counts the number of times a pair of characters repeats in a string.
///
/// *Warning* This implementation currently assumes single byte characters.
fn count_pairs(input: &str) -> u32 {
	let char_vec: Vec<char> = input.chars().collect();
	let chars: &[char] = &char_vec[..];

	if chars.len() <= 1 {
		return 0
	}

	let mut repeated_pairs = 0;

	for start in 0..input.len()-3 {
		let end = start + 2;
		let current_pair = &input[start..end];
		let rest = &input[end..];
		let contains_pair = rest.contains(current_pair);

		if contains_pair {
			repeated_pairs = repeated_pairs + 1;
		}
	}

	repeated_pairs
}

/// Counts the number of times a character repeats with one character 
/// interleaving them (aba, but not aa).
///
/// *Warning* This implementation currently assumes single byte characters.
fn count_repeat_overlap(input: &str) -> u32 {
	let char_vec: Vec<char> = input.chars().collect();
	let chars: &[char] = &char_vec[..];

	if chars.len() < 3 {
		return 0
	}

	let mut repeat_count = 0;
	for i in 2..chars.len() {
		let prev = chars[i-2];
		let current = chars[i];

		if prev == current {
			repeat_count = repeat_count + 1;
		}
	}

	repeat_count
}

#[test]
fn is_naughty_test() {
	let test_cases = vec!(
		("ugknbfddgicrmopn", IsNiceAlgorithm::Version1, true),
		("aaa",              IsNiceAlgorithm::Version1, true),
		("jchzalrnumimnmhp", IsNiceAlgorithm::Version1, false),
		("haegwjzuvuyypxyu", IsNiceAlgorithm::Version1, false),
		("dvszwmarrgswjxmb", IsNiceAlgorithm::Version1, false),
		("qjhvhtzxzqqjkmpb", IsNiceAlgorithm::Version2, true),
		("xxyxx", IsNiceAlgorithm::Version2, true),
		("uurcxstgmygtbstg", IsNiceAlgorithm::Version2, false),
		("ieodomkazucvgmuy", IsNiceAlgorithm::Version2, false)
	);

	println!("");
	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, algo, expect) = *test_case;
		println!("{:?}, {:?}, {:?}, {:?}", i, input, algo, expect);

		assert_eq!(expect, is_nice(input, algo));
	}
}

#[test]
fn is_vowel_test() {
	let test_cases = vec!(
		('a', true),
		('e', true),
		('i', true),
		('o', true),
		('u', true),
		('c', false),
		('b', false),
	);

	println!("");
	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, expect) = *test_case;
		println!("{:?}, {:?}, {:?}", i, input, expect);

		assert_eq!(expect, is_vowel(input));
	}
}

#[test]
fn count_vowels_test() {
	let test_cases = vec!(
		("a", 1),
		("ae", 2),
		("aeiou", 5),
		("abc", 1)
	);

	println!("");
	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, expect) = *test_case;
		println!("{:?}, {:?}, {:?}", i, input, expect);

		assert_eq!(expect, count_vowels(input));
	}
}

#[test]
fn count_repeat_test() {
	let test_cases = vec!(
		("a", 0),
		("aa", 1),
		("abba", 1),
		("aabb", 2)
	);

	println!("");
	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, expect) = *test_case;
		println!("{:?}, {:?}, {:?}", i, input, expect);

		assert_eq!(expect, count_repeat(input));
	}
}

#[test]
fn contains_naughty_strings_test() {
	let test_cases = vec!(
		("a", false),
		("ab", true),
		("cd", true),
		("pq", true),
		("xy", true),
		("abcd", true),
		("zxwy", false),
	);

	println!("");
	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, expect) = *test_case;
		println!("{:?}, {:?}, {:?}", i, input, expect);

		assert_eq!(expect, contains_naughty_strings(input));
	}
}

#[test]
fn count_pairs_test() {
	let test_cases = vec!(
		("xyxy", 1),
		("aabcdefgaa", 1),
		("aaa", 0)
	);

	println!("");
	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, expect) = *test_case;
		println!("{:?}, {:?}, {:?}", i, input, expect);

		assert_eq!(expect, count_pairs(input));
	}
}

#[test]
fn count_repeat_overlap_test() {
	let test_cases = vec!(
		("xyx", 1),
		("abcdefeghi", 1),
		("aaa", 1),
		("uurcxstgmygtbstg", 0)
	);

	println!("");
	for (i, test_case) in test_cases.iter().enumerate() {
		let (input, expect) = *test_case;
		println!("{:?}, {:?}, {:?}", i, input, expect);

		assert_eq!(expect, count_repeat_overlap(input));
	}
}
