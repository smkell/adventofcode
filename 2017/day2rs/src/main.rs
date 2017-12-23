fn main() {
    use std::io::{self, Read};
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let checksum: u32 =
        buffer.trim()
              .lines()
              .map(|l| parse_row(l))
              .map(|r| row_checksum(r))
              .fold(0, |accum, x| accum + x);
    println!("checksum: {:?}", checksum);

    let divisible: u32 =
        buffer.trim()
              .lines()
              .map(|l| parse_row(l))
              .map(|r| evenly_divisible(r))
              .fold(0, |accum, x| accum + x);
    println!("divisible: {:?}", divisible);
}

fn parse_row(input: &str) -> Vec<u32> {
    input.split('\t')
         .map(|x| x.parse::<u32>().unwrap())
         .collect()
}

fn row_checksum(row: Vec<u32>) -> u32 {
    row.iter().max().unwrap() - row.iter().min().unwrap()
}

fn evenly_divisible(row: Vec<u32>) -> u32 {
    for i in 0..row.len()-1 {
        for j in i+1..row.len() {
            let (x, y) = (row[i], row[j]);
            println!("row[i] = {}\trow[j] = {}", x, y);

            if x > y && y > 0 && x % y == 0 {
                return (x / y) as u32
            }

            if y > x && x > 0 && y % x == 0 {
                return (y / x) as u32
            }
        }
    }
    return 0
}

fn spreadsheet_checksum(row_checksums: Vec<u32>) -> u32 {
    row_checksums.iter().fold(0, |accum, x| accum + x)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_row() {
        let test_cases = vec!(
            ("5\t1\t9\t5", vec!(5, 1, 9, 5)),
            ("7\t5\t3", vec!(7, 5, 3)),
            ("2\t4\t6\t8", vec!(2, 4, 6, 8))
        );

        for (input, expect) in test_cases {
            let actual = parse_row(input);
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn test_row_checksum() {
        let test_cases = vec!(
            (vec!(5, 9, 2, 8), 4),
            (vec!(9, 4, 7, 3), 3),
            (vec!(3, 8, 6, 5), 2)
        );

        for (input, expect) in test_cases {
            let actual = evenly_divisible(input);
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn test_evenly_divisible() {
        let test_cases = vec!(
            (vec!(5, 1, 9, 5), 8),
            (vec!(7, 5, 3), 4),
            (vec!(2, 4, 6, 8), 6)
        );

        for (input, expect) in test_cases {
            let actual = row_checksum(input);
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn test_spreadsheet_checksum() {
        let test_cases = vec!(
            (vec!(8, 4, 6), 18)
        );

        for (input, expect) in test_cases {
            let actual = spreadsheet_checksum(input);
            assert_eq!(expect, actual);
        }
    }
}