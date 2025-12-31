use std::env;
use std::fs;
use std::process;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
	x: i64,
	y: i64,
}

impl Point {
	fn add(&mut self, other: &Point) {
		self.x += other.x;
		self.y += other.y;
	}

	fn manhattan_distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
		let origin = Point {x:0, y:0};
		self.manhattan_distance(&origin).cmp(&other.manhattan_distance(&origin))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_data(filename: &str) -> Vec<Vec<Vec<String>>> {
	fs::read_to_string(filename)
		.expect("Expected input file to exist")
		.split("\n\n")
		.map(|l| l.split("\n").map(
					|i| i.split(",")
						 .map(|w| String::from(w))
						 .collect())
					.collect())
		.collect()
}

// Return the number of steps to take for an instruction
fn move_for(instruction: &str) -> i64 {
	if instruction.len() > 1 {
		return match instruction[1..].parse() {
			Ok(num) => num,
			Err(_) 	=> 0
		};
	}

	return 0;
}

// Return the 2D direction vector for an instruction
// R=east, L=west, U=north, D=south
fn dir_for(instruction: &str) -> Point {
	if instruction.len() > 1 {
		return match instruction.chars().nth(0).unwrap() {
			'R' => Point {x: 1, y: 0},
			'L' => Point {x:-1, y: 0},
			'D' => Point {x: 0, y:-1},
			'U' => Point {x: 0, y: 1},
			_	=> Point {x: 0, y: 0}
		};
	}

	return Point {x:0, y:0}
}

// Return a wire constructed from the directional instructions.
// The wire is a HashMap of points and the minimum number of steps 
// (from starting point) to get to the point on the wire.
fn make_wire(instructions: &Vec<String>, origin: &Point) -> HashMap<Point, u64> {
	let mut circuit = HashMap::<Point, u64>::new();
	let mut p = origin.clone();
	let mut steps = 0;

	for node in instructions {
		let dir = dir_for(node);
		let n = move_for(node);

		// don't add starting point (includes origin)
		// circuit.entry(p.clone()).or_insert(0);
		for _step in 0..n {
			p.add(&dir);
			steps += 1;
			circuit.entry(p.clone()).or_insert(steps);
		}
	}

	return circuit;
}

// Return collection of wires that make up the circuit.
// (there are only ever two wires. This code can handle any number.)
fn make_circuit(wires: &Vec<Vec<String>>, origin: &Point) -> Vec<HashMap<Point, u64>> {
	let mut circuit = Vec::new();

	for instructions in wires {
		let wire = make_wire(instructions, origin);
		circuit.push(wire);
	}

	return circuit;
}

// Return a HashSet of the points where the collection of wires intersect
fn find_intersections(wires: &Vec<HashMap<Point, u64>>) -> HashSet<Point> {
	if wires.is_empty() {
		return HashSet::new();
	}

	let mut intersection: HashSet<Point> = wires[0].keys().cloned().collect();

	for wire in wires.iter().skip(1) {
		intersection.retain(|k| wire.contains_key(k));
	}

	return intersection;
}

// Part 1
fn part1(circuits: &Vec<Vec<Vec<String>>>) -> usize {
	let origin = Point {x:0, y:0};

	// return the manhattan distance to the closest intersection
	return circuits.iter()
		.map(|instr| make_circuit(instr, &origin))
		.map(|circuit| find_intersections(&circuit).iter()
						.min().expect("Cannot find a minimum intersection.")
						.manhattan_distance(&origin).try_into().unwrap()
					)
		.min().expect("No minimum circuit distance.");
}

// Part 2
fn part2(circuits: &Vec<Vec<Vec<String>>>) -> usize {
	let origin = Point {x:0, y:0};

	// return the sum of steps to point `p` for each wire in the circuit
	fn steps_to(circuit: &Vec<HashMap<Point, u64>>, p: &Point) -> u64 {
		circuit.iter().fold(0, |acc, w| acc + w.get(p).unwrap())
	}
	
	// return the sum of steps to the closest intersection
	// closest is the smallest sum of steps across all wires in the circuit
	return circuits.iter()
		.map(|instr| make_circuit(instr, &origin))
		.map(|circuit| find_intersections(&circuit).iter()
							.map(|p| steps_to(&circuit, &p))
							.min().expect("Cannot find a minimum intersection.")
							.try_into().unwrap()
					)
		.min().expect("No minimum circuit distance.");
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
