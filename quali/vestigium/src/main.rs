use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::stdin;
use std::str::FromStr;

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

fn is_unique<T: Eq + Hash, I: IntoIterator<Item = T>>(items: I) -> bool {
    let mut seen = HashSet::new();
    for item in items {
        if seen.contains(&item) { return false; }
        seen.insert(item);
    }
    true
}

fn do_test_case(index: usize) {
    let n: usize = read_from_stdin();
    let matrix: Vec<Vec<usize>> = (0..n).map(|_| read_integers_from_stdin()).collect();

    let row_count = (0..n).filter(|&row| !is_unique((0..n).map(|col| matrix[row][col]))).count();
    let col_count = (0..n).filter(|&col| !is_unique((0..n).map(|row| matrix[row][col]))).count();
    let trace: usize = (0..n).map(|k| matrix[k][k]).sum();

    println!("Case #{}: {} {} {}", index + 1, trace, row_count, col_count);
}

fn main() {
    let cases: usize = read_from_stdin();
    for index in 0..cases {
        do_test_case(index);
    }
}
