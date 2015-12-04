#![feature(advanced_slice_patterns, slice_patterns)]

/// Day 4 of http://adventofcode.com
///
/// --- Day 4: The Ideal Stocking Stuffer ---
/// 
/// Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the 
/// economically forward-thinking little girls and boys.
/// 
/// To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. 
/// The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a 
/// number in decimal. To mine AdventCoins, you must find Santa the lowest positive number (no 
/// leading zeroes: 1, 2, 3, ...) that produces such a hash.
/// 
/// For example:
/// 
/// If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts 
/// with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
/// If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting 
/// with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....
/// 
/// Your puzzle input is bgvyzdsv.

extern crate crypto;

fn main() {
	let key = "bgvyzdsv";

	match mine_adventcoin5(key) {
		Some(n) => {
			println!("The result for key {:?} with 5 zeroes is {:?}", key, n);
		},
		None => {
			println!("Uh oh. Couldn't locate a valid result.");
		}
	};

	match mine_adventcoin6(key) {
		Some(n) => {
			println!("The result for key {:?} with 6 zeroes is {:?}", key, n);
		},
		None => {
			println!("Uh oh. Couldn't locate a valid result.");
		}
	};
}

fn mine_adventcoin5(key: &str) -> Option<u32> {
	use crypto::md5::Md5;
	use crypto::digest::Digest;

	let mut sh = Md5::new();

	let mut result: Option<u32> = None;
	let mut test: u32 = 1;

	while result.is_none() {
		let key_string = key.to_string();
		let test_str = test.to_string();
		let test_key = key_string + &test_str;

		sh.input_str(&test_key);

		let digest = sh.result_str();

		//println!("Digest: {:?}", digest);

		let digest_bytes: Vec<char> = digest.chars().collect();

		//println!("digest_bytes: {:?}", digest_bytes);

		result = match &digest_bytes[..] {
			['0', '0', '0', '0', '0', ..] => Some(test),
			_ => {
				test = test + 1;
				None
			}
		};

		sh.reset();
	}

	Some(test)
}

fn mine_adventcoin6(key: &str) -> Option<u32> {
	use crypto::md5::Md5;
	use crypto::digest::Digest;

	let mut sh = Md5::new();

	let mut result: Option<u32> = None;
	let mut test: u32 = 1;

	while result.is_none() {
		let key_string = key.to_string();
		let test_str = test.to_string();
		let test_key = key_string + &test_str;

		sh.input_str(&test_key);

		let digest = sh.result_str();

		//println!("Digest: {:?}", digest);

		let digest_bytes: Vec<char> = digest.chars().collect();

		//println!("digest_bytes: {:?}", digest_bytes);

		result = match &digest_bytes[..] {
			['0', '0', '0', '0', '0', '0', ..] => Some(test),
			_ => {
				test = test + 1;
				None
			}
		};

		sh.reset();
	}

	Some(test)
}

#[test]
fn mine_adventcoin5_test() {

	let key1 = "abcdef";
	let expect1 = 609043;
	println!("Test case 1: key = {:?}, expect = {:?}", key1, expect1);
	assert_eq!(expect1, mine_adventcoin5(key1).unwrap());

	let key2 = "pqrstuv";
	let expect2 = 1048970;
	println!("Test case 1: key = {:?}, expect = {:?}", key2, expect2);
	assert_eq!(expect2, mine_adventcoin5(key2).unwrap());
}

#[test]
fn crypto_test() {
	use crypto::md5::Md5;
	use crypto::digest::Digest;

	let msg = "";
	let mut sh = Md5::new();
	sh.input_str(msg);

	let out_str = sh.result_str();
	assert_eq!(out_str, "d41d8cd98f00b204e9800998ecf8427e");
}