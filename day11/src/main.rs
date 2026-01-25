use std::env;
use std::process;
use std::cmp;
use std::time::Instant; // 0.8.2
use std::collections::HashMap;

pub mod intcode;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Point {
	x: i64,
	y: i64,
}

impl Point {
	fn add(&mut self, other: &Point) {
		self.x += other.x;
		self.y += other.y;
	}
}

fn read_data(filename: &str) -> Vec<i64> {
    return intcode::read_program(filename);
}

// Runs paint program and returns vector of painted panels
fn paint(program: &Vec<i64>, start_color: i64) -> HashMap<Point, i64> {
    let mut position = Point { x: 0, y: 0 };
    let mut direction = Point { x: 0, y: -1 };
    let mut painted: HashMap<Point, i64> = HashMap::new();
    let mut cpu = intcode::build_cpu(program, &vec![start_color]);

    while !cpu.is_terminated() {
        cpu.run();

        if let Some(color) = cpu.pop_output() {
            painted.insert(position, color);

            if let Some(dir) = cpu.pop_output() { 
                direction = match dir {
                    0 => { Point { x: -direction.y, y: direction.x } },
                    1 => { Point { x: direction.y, y: -direction.x } },
                    _ => { Point { x: 0, y: 0 } },
                };

                // println!("Paint {position:?} = {color} move {direction:?}");
                position.add(&direction);

                // get color under panel and push as input to next iteration
                cpu.push_input(
                    match painted.get(&position) {
                        Some(c) => *c,  // painted color
                        None => 0,   // black
                    }
                );

            } else {
                println!("ERROR: No output direction!");
                break;
            }
        } else {
            println!("ERROR: No output color!");
            break;
        }
    }

    return painted;
}

fn show(panels: &HashMap<Point, i64>) {
    let x1 = panels.iter().min_by_key(|(p, _c)| p.x).map(|(p, _c)| p.x).unwrap();
    let x2 = panels.iter().max_by_key(|(p, _c)| p.x).map(|(p, _c)| p.x).unwrap();
    let y1 = panels.iter().min_by_key(|(p, _c)| p.y).map(|(p, _c)| p.y).unwrap();
    let y2 = panels.iter().max_by_key(|(p, _c)| p.y).map(|(p, _c)| p.y).unwrap();

    for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {
        // need to reverse to see it in the camera?
        for x in (cmp::min(x1, x2)..=cmp::max(x1, x2)).rev() {
            let position = Point {x: x, y: y};
            let color = match panels.get(&position) {
                Some(c) => *c,
                None => 0,   // black
            };

            if color == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }

        println!();
    }
}

fn part1(program: &Vec<i64>) -> usize {
    let painted = paint(program, 0);

    return painted.len();
}

fn part2(program: &Vec<i64>) -> usize {
    let painted = paint(program, 1);

    show(&painted);

    return painted.len();
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
