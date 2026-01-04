use std::env;
use std::fs;
use std::process;
use std::time::Instant;

fn read_data(filename: &str) -> Vec<usize> {
	fs::read_to_string(filename)
		.expect("Expected input file to exist")
		.split("-")
		.map(|num| num.parse::<usize>().unwrap())
		.collect()
}

/* Return true if `n` has digits that repeat exactly `repeat` times.
 * if `repeat` is zero, then return true if any digits repeat any number of times.
 * false otherwise
 */
fn has_repeat(n: usize, repeat: usize) -> bool {
	let mut count = 1;
	let mut last = n % 10;
	let mut num = n / 10;

	while num != 0 {
		if last == (num % 10) {
			// digit matches last digit
			if repeat == 0 {
				// true for part 1 if any repeating
				return true;
			}

			count += 1;
		} else {
			// digit does not match last digit
			if repeat != 0 && count == repeat {
				return true;
			}

			count = 1;
		}

		last = num % 10;
		num /= 10;
	}

	// when last digit is the repeater...
	if repeat != 0 && count == repeat {
		return true;
	}

	return false;
}

/* Return true if the digits of `n` are ascending (or equal) from left to right.
 */
fn is_ascending(n: usize) -> bool {
	let mut last = n % 10;
	let mut num = n / 10;

	while num != 0 {
		if last < (num % 10) {
			return false;
		}

		last = num % 10;
		num /= 10;
	}

	return true;
}

// Part 1
fn part1(bounds: &Vec<usize>) -> usize {
	fn is_password(pass: usize) -> bool {
		is_ascending(pass) && has_repeat(pass, 0)
	}

	let count = (bounds[0]..=bounds[1])
		.fold(0, |acc, pass| {if is_password(pass) { acc+ 1 } else { acc }});

	return count;
}

// Part 2
fn part2(bounds: &Vec<usize>) -> usize {
	fn is_password(pass: usize) -> bool {
		is_ascending(pass) && has_repeat(pass, 2)
	}

	let count = (bounds[0]..=bounds[1])
		.fold(0, |acc, pass| {if is_password(pass) { acc+ 1 } else { acc }});
	
	return count;
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("ERROR: No input file specified");
		process::exit(1);
	}

	let start = Instant::now();

	let data = read_data(&args[1]);
	println!(
		"          parse ({:9.9} ns)",
		Instant::now().duration_since(start).as_nanos()
	);

	let p1_start = Instant::now();
	let p1 = part1(&data);
	println!(
		"{p1:>15} ({:9.9} ns)",
		Instant::now().duration_since(p1_start).as_nanos()
	);

	let p2_start = Instant::now();
	let p2 = part2(&data);

	println!(
		"{p2:>15} ({:9.9} ns)",
		Instant::now().duration_since(p2_start).as_nanos()
	);
	println!(
		"          total ({:9.9} ns)",
		Instant::now().duration_since(start).as_nanos()
	);
}
