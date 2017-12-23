fn main() {
    use std::io::{self, Read};
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let pairs = parse_pairs(&buffer);
    let result_a = add_pairs(pairs);
    println!("Sum (A): {}", result_a);

    let half_pairs = parse_halfway(&buffer);
    let result_b = add_pairs(half_pairs);
    println!("Sum (B): {}", result_b);
}

fn parse_pairs(input: &str) -> Vec<char> {
    let input_chars: Vec<char> = input.trim().chars().collect();
    let mut result: Vec<char> = vec!();

    for i in 0..(input_chars.len()-1) {
        if input_chars[i] == input_chars[i+1] {
            result.push(input_chars[i]);
        }
    }

    if input_chars[input_chars.len()-1] == input_chars[0] {
        result.push(input_chars[0]);
    }
    return result;
}

fn parse_halfway(input: &str) -> Vec<char> {
    let input_chars: Vec<char> = input.trim().chars().collect();
    let mut result: Vec<char> = vec!();

    let half = input_chars.len() / 2;
    for i in 0..(input_chars.len()) {
        let mut next = i + half;
        if next > input_chars.len() - 1 {
            next = next - input_chars.len();
        }

        if input_chars[i] == input_chars[next] {
            result.push(input_chars[i]);
        }
    }

    return result;
}

fn add_pairs(pairs: Vec<char>) -> u32 {
    pairs.iter()
        .map(|x| x.to_digit(10).unwrap())
        .fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pairs_should_work() {
        let test_cases = vec!(
            ("1122", vec!('1', '2')),
            ("1111", vec!('1', '1', '1', '1')),
            ("1234", vec!()),
            ("91212129", vec!('9'))
        );

        for (input, expect) in test_cases {
            let actual = parse_pairs(input);
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn parse_halfway_should_work() {
        let test_cases = vec!(
            ("1212", vec!('1', '2', '1', '2')),
            ("1221", vec!()),
            ("123425", vec!('2', '2')),
            ("123123", vec!('1', '2', '3', '1', '2', '3')),
            ("12131415", vec!('1', '1', '1', '1'))
        );

        for (input, expect) in test_cases {
            let actual = parse_halfway(input);
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn add_pairs_should_work() {
        let test_cases = vec!(
            (vec!('1', '2'), 3),
            (vec!('1', '1', '1', '1'), 4),
            (vec!(), 0),
            (vec!('9'), 9)
        );
        for (input, expect) in test_cases {
            let actual = add_pairs(input);
            assert_eq!(expect, actual);
        }
        
    }
}