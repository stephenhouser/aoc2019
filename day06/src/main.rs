use std::env;
use std::fs;
use std::process;
use std::time::{Instant};
use std::collections::HashMap;

/* Create HashMap where key is object and value is what it orbits
 * this is the reverse order in which they are specified, so ... 
 * reverse them when creating the hash map.
*/
fn read_data(filename: &str) -> HashMap<String, String> {
	fs::read_to_string(filename)
		.expect("Expected input file to exist")
		.lines()
		.filter_map(|line| line.split_once(')'))
		.map(|(k, v)| (v.to_string(), k.to_string()))	// A)B is B->A
		.collect()
}

/* Return a vector of planets traversing from source to dest.
 * Return an empty vector if there is no path or source == dest.
 */
fn path_to(universe: &HashMap<String, String>, source: &str, dest: &str) -> Vec<String> {
	let mut path = Vec::new();

	let mut current = source;
	while current != dest {
		let next = universe.get(current).unwrap();
		path.push(next.to_string());
		current = next;
		// if let Some(next) = universe.get(current) {
		// 	path.push(next.to_string());
		// 	current = next;
		// } else {
		// 	return vec![];
		// }
	}

	return path;
}
/* Return how many orbital transfers are needed to go from 
 * what you orbits to what san orbits.
 */
fn transfers_needed(universe: &HashMap<String, String>, you: &str, san: &str) -> usize {
	/* Find the number by finding the first common planet in the paths to the
	 * center (COM). Then the distances for each to that common planet.
	 */
	let you_path = path_to(universe, &you, "COM");
	let san_path = path_to(universe, &san, "COM");

	// find common planet
	for planet in you_path {
		if san_path.contains(&planet) {
			// return sum of distances to common planet
			let you_here = path_to(universe, you, &planet).len();
			let san_here = path_to(universe, san, &planet).len();
			return you_here + san_here;
		}
	}

	// no path
	return 0;
}

fn part1(universe: &HashMap<String, String>) -> usize {
	return universe.keys()
		.map(|planet| path_to(universe, planet, "COM"))
		.fold(0, |acc, path| acc + path.len());
}

fn part2(universe: &HashMap<String, String>) -> usize {
	// live input data
	if universe.contains_key("YOU")  && universe.contains_key("SAN") {
		let you = &universe["YOU"];
		let san = &universe["SAN"];
		return transfers_needed(universe, you, san);
	}

	// test input data
	if universe.contains_key("L")  && universe.contains_key("H") {
		// hand chosen locations to match example
		return transfers_needed(universe, "K", "I");
	}

	return universe.len();
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("ERROR: No input file specified");
		process::exit(1);
	}

	let start = Instant::now();

	let masses = read_data(&args[1]);
	println!("          parse ({:9.9} ns)", Instant::now().duration_since(start).as_nanos());

	let p1_start = Instant::now();
	let p1 = part1(&masses);
	println!("{p1:>15} ({:9.9} ns)", Instant::now().duration_since(p1_start).as_nanos());

	let p2_start = Instant::now();
	let p2 = part2(&masses);

	println!("{p2:>15} ({:9.9} ns)", Instant::now().duration_since(p2_start).as_nanos());
	println!("          total ({:9.9} ns)", Instant::now().duration_since(start).as_nanos());
}
