use std::env;
use std::fs;
use std::process;
use std::time::{Instant};

fn read_data(filename: &str) -> Vec<u64> {
	fs::read_to_string(filename)
		.expect("Expected input file to exist")
		.lines()
		.map(|line| line.parse::<u64>().unwrap())
		.collect()
}

fn part1(masses: &Vec<u64>) -> u64 {
	masses
		.iter().map(|m| m / 3 - 2)
		.fold(0, |acc, x| acc + x)
}

fn fuel_needed(mass: &u64) -> u64 {
	if (*mass/3) <= 2 {
		return 0;
	}

	let fuel = mass / 3 - 2;
	return fuel + fuel_needed(&fuel) ;
}

fn part2(masses: &Vec<u64>) -> u64 {
	masses
		.iter().map(fuel_needed)
		.fold(0, |acc, x| acc + x)
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
