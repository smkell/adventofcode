fn main() {
    use std::collections::BTreeMap;
    use std::io::{self, Read};
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();


    let mut frequencies: Vec<BTreeMap<char, i32>> = Vec::new();
    for _ in 0..buffer.lines().nth(0).unwrap().len() {
        frequencies.push(BTreeMap::new());
    }

    for line in buffer.lines() {
        for (i, c) in line.chars().enumerate() {
            *(frequencies[i].entry(c).or_insert(0)) += 1;
        }
    }

    let mut output = String::new();
    for (_, frequency) in frequencies.iter().enumerate() {
        let max = match frequency.iter().max_by_key(|&(_, v)| v) {
            Some((c, _)) => c,
            None => panic!("Uh oh!")
        };
        output.push(*max);
    }

    println!("{}", output);
}
