use std::fmt::Debug;
use std::io::stdin;
use std::str::FromStr;

fn read_from_stdin<T: FromStr>() -> T where T::Err : Debug {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn read_integers_from_stdin() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().chars().map(|s| s.to_digit(10).unwrap() as usize).collect()
}

fn create_nested_string<I: IntoIterator<Item = usize>>(inputs: I) -> String {
    let mut result = String::new();
    let mut current_depth = 0;

    for input in inputs {
        if current_depth < input { result.push_str(&"(".repeat(input - current_depth)); }
        if current_depth > input { result.push_str(&")".repeat(current_depth - input)); }
        result.push_str(&input.to_string());
        current_depth = input;
    }

    if current_depth > 0 { result.push_str(&")".repeat(current_depth)); }

    result
}

fn do_test_case(index: usize) {
    let input: Vec<usize> = read_integers_from_stdin();
    println!("Case #{}: {}", index + 1, create_nested_string(input));
}

fn main() {
    let cases: usize = read_from_stdin();
    for index in 0..cases {
        do_test_case(index);
    }
}
