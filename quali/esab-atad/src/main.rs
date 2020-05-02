use std::fmt::{Debug, Display};
use std::io::{stdin, stdout, Write};
use std::str::{FromStr};

fn read_from_stdin<T: FromStr>() -> T where T::Err : Debug {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn read_integers_from_stdin<T: FromStr>() -> Vec<T> where T::Err : Debug {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn output_and_flush<T: Display>(output: T) {
    println!("{}", output);
    stdout().flush().unwrap();
}

fn ask_query(position: usize) -> usize {
    output_and_flush(position + 1);
    read_from_stdin()
}

fn four_possibilities(known_bits: &[Option<usize>]) -> Vec<Vec<Option<usize>>> {
    return vec![
        known_bits.iter().map(|b| *b).collect(),
        known_bits.iter().map(|b| b.map(|x| 1 - x)).collect(),
        known_bits.iter().map(|b| *b).rev().collect(),
        known_bits.iter().map(|b| b.map(|x| 1 - x)).rev().collect(),
    ];
}

fn do_test_case(length: usize) {
    let mut known_bits: Vec<Option<usize>> = vec![None; length];

    loop {

        let possibilities = four_possibilities(&known_bits);
        let first_query_position = (0..length).filter(|&idx| known_bits[idx].is_some()).next().unwrap_or(0);
        let first_query_result = ask_query(first_query_position);

        let remaining_possibilities: Vec<_> = possibilities.iter()
            .filter(|p| p[first_query_position].is_none() || p[first_query_position] == Some(first_query_result))
            .collect();
        let second_query_position = (0..length).filter(|&idx| remaining_possibilities[0][idx] != remaining_possibilities[1][idx]).next().unwrap_or(0);
        let second_query_result = ask_query(second_query_position);

        let correct_possibility = remaining_possibilities.iter()
            .filter(|p| p[second_query_position].is_none() || p[second_query_position] == Some(second_query_result))
            .next().unwrap();

        known_bits = correct_possibility.to_vec();
        for _ in 0..4 {
            if let Some(first_missing_bit) = known_bits.iter().position(|b| b.is_none()) {
                known_bits[first_missing_bit] = Some(ask_query(first_missing_bit));
                known_bits[length - 1 - first_missing_bit] = Some(ask_query(length - 1 - first_missing_bit));
            }
        }

        if known_bits.iter().all(|b| b.is_some()) {
            output_and_flush(known_bits.iter().map(|c| c.unwrap().to_string()).collect::<String>());
            let _result: String = read_from_stdin();
            return;
        }
    }
}

fn main() {
    let input: Vec<usize> = read_integers_from_stdin();
    let (cases, length) = (input[0], input[1]);
    for _ in 0..cases {
        do_test_case(length);
    }
}
