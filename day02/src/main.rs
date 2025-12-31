use std::env;
use std::fs;
use std::process;
use std::time::Instant;

fn read_data(filename: &str) -> Vec<usize> {
	fs::read_to_string(filename)
		.expect("Expected input file to exist")
		.replace("\n", "")
		.split(',')
		.map(|line| line.parse::<usize>().unwrap())
		.collect()
}

fn add(memory: &mut Vec<usize>, pc : usize) -> usize {
	let p1 = memory[pc+1];
	let p2 = memory[pc+2];
	let p3 = memory[pc+3];
	// println!("add {op1} * {op2} => {dst}");
	memory[p3] = memory[p1] + memory[p2];
	return pc + 4;
}

fn mul(memory: &mut Vec<usize>, pc : usize) -> usize {
	let p1 = memory[pc+1];
	let p2 = memory[pc+2];
	let p3 = memory[pc+3];
	// println!("mul {op1} * {op2} => {dst}");
	memory[p3] = memory[p1] * memory[p2];
	return pc + 4;
}

fn term(memory: &mut Vec<usize>, _pc: usize) -> usize {
	// println!("term at {pc}");
	return memory.len();
}

fn show(memory: &Vec<usize>, pc: usize) {
	for (i, v) in memory.iter().enumerate() {
		if i == pc {
			print!("({v})");
		} else {
			print!("{v}");
		}
		if i < memory.len()-1 {
			print!(",");
		}
	}
	println!("")
}

fn run_program(memory: &mut Vec<usize>, start: usize) -> usize {
	let mut pc: usize = start;

	while pc < memory.len() {
		// show(&memory, pc);
		match memory[pc] {
			1	=> pc = add(memory, pc),
			2	=> pc = mul(memory, pc),
			99 	=> pc = term(memory, pc),
			n 	=> println!("{pc}: unknown operation {n}")
		}
	}

	// show(&memory, pc);
	return memory[0];
}

fn part1(program: &Vec<usize>) -> usize {
	let mut memory = program.clone();	

	if program.len() < 20 {
		// test input
		run_program(&mut memory, 0);
	} else {
		// live input
		memory[1] = 12;
		memory[2] = 2;
		run_program(&mut memory, 0);
	}

	return memory[0];
}

fn part2(program: &Vec<usize>) -> usize {

	if program.len() < 20 {
		// test input
		return 0;
	}

	// live input
	for n in 0..=99 {
		for v in 0..=99 {
			let mut memory = program.clone();	
			memory[1] = n;
			memory[2] = v;
			run_program(&mut memory, 0);

			if memory[0] == 19690720 {
				return 100*n + v;
			}
		}
	}

	return 0;
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
