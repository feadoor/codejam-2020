use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::stdin;
use std::str::FromStr;

#[derive(Copy, Clone)]
struct Activity {
	start: usize,
	end: usize,
}

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

fn read_activities_from_stdin(n: usize) -> Vec<Activity> {
	return (0..n).map(|_| {
		let value = read_integers_from_stdin();
		Activity { start: value[0], end: value[1] }
	}).collect();
}

fn overlap(a1: Activity, a2: Activity) -> bool {
	if a1.start <= a2.start && a1.end > a2.start { true }
	else if a2.start <= a1.start && a2.end > a1.start { true }
	else { false }
}

fn find_bipartite_assignment(activities: Vec<Activity>) -> String {
	let mut overlaps = vec![vec![]; activities.len()];
	for idx in 0..activities.len() {
		for jdx in 0..activities.len() {
			if idx != jdx && overlap(activities[idx], activities[jdx]) {
				overlaps[idx].push(jdx);
				overlaps[jdx].push(idx);
			}
		}
	}

	let mut assignment: Vec<isize> = vec![0; activities.len()];
	while let Some(first_unassigned) = assignment.iter().position(|&x| x == 0) {
		let mut to_fill = vec![(first_unassigned, 1)];
		while let Some((index, value)) = to_fill.pop() {
			if assignment[index] == -value { return "IMPOSSIBLE".to_string(); }
			else if assignment[index] == 0 {
				assignment[index] = value;
				for &neighbour in &overlaps[index] {
					to_fill.push((neighbour, -value));
				}
			}
		}
	}

	assignment.iter().map(|&v| if v == 1 { "C" } else { "J" }).collect()
}

fn do_test_case(index: usize) {
	let n: usize = read_from_stdin();
	let activities = read_activities_from_stdin(n);
    println!("Case #{}: {}", index + 1, find_bipartite_assignment(activities));
}

fn main() {
    let cases: usize = read_from_stdin();
    for index in 0..cases {
        do_test_case(index);
    }
}
