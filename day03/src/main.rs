use std::env;
use std::fs;
use std::process;
use std::time::Instant;
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

fn move_for(instruction: &str) -> i64 {
	if instruction.len() > 1 {
		return match instruction[1..].parse() {
			Ok(num) => num,
			Err(_) 	=> 0
		};
	}

	return 0;
}

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

// Make set of points that lie on the wire described by the move instructions
fn make_wire(instructions: &Vec<String>, origin: &Point) -> HashSet<Point> {
	let mut circuit = HashSet::<Point>::new();
	let mut p = origin.clone();

	for node in instructions {
		let dir = dir_for(node);
		let n = move_for(node);

		circuit.insert(p.clone());
		for _i in 0..n {
			p.add(&dir);
			circuit.insert(p.clone());
		}
	}

	return circuit;
}

fn make_circuit(wires: &Vec<Vec<String>>, origin: &Point) -> Vec<HashSet<Point>> {
	let mut circuit = Vec::new();

	for instructions in wires {
		let wire = make_wire(instructions, origin);
		circuit.push(wire);
	}

	return circuit;

}

fn find_intersections(wires: &Vec<HashSet<Point>>) -> HashSet<Point> {
	if wires.is_empty() {
		return HashSet::new();
	}

	let mut intersection = wires[0].clone();
	for wire in wires.iter().skip(1) {
		intersection = intersection
			.intersection(wire)
			.cloned()
			.collect();
	}

	return intersection;
}

fn part1(circuits: &Vec<Vec<Vec<String>>>) -> usize {
	let origin = Point {x:0, y:0};

	let mut closest: usize = 0;	
	for wires in circuits {
		let circuit = make_circuit(wires, &origin);

		closest = find_intersections(&circuit)
			.iter()
			.filter(|p| !(p.x == origin.x && p.y == origin.y))
			.min().expect("Oops")
			.manhattan_distance(&origin)
			.try_into().unwrap();
	}

	return closest;
}

fn part2(wires: &Vec<Vec<Vec<String>>>) -> usize {
	return wires.len();
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
