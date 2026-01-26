use std::env;
use std::process;
use std::fs;
use std::cmp::Ordering;
use std::time::Instant; // 0.8.2

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Point {
	x: i64,
	y: i64,
    z: i64,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Body {
    p: Point,
    velocity: Point,
}

impl Point {
	// fn add(&mut self, other: &Point) {
	// 	self.x += other.x;
	// 	self.y += other.y;
    //     self.z += other.z;
	// }

    // fn equal(&self, other: &Point) -> bool {
    //     return self.x == other.x && self.y == other.y && self.z == other.z;
    // }
} 

// fn pair_gravity(b1: &Body, b2: &Body) -> Point {
//     return Point {
//         x: match b1.p.x.cmp(&b2.p.x) {
//             Ordering::Greater => { -1 },
//             Ordering::Equal => { 0 },
//             Ordering::Less => { 1 },
//         },
//         y: match b1.p.y.cmp(&b2.p.y) {
//             Ordering::Greater => { -1 },
//             Ordering::Equal => { 0 },
//             Ordering::Less => { 1 },
//         },
//         z: match b1.p.z.cmp(&b2.p.z) {
//             Ordering::Greater => { -1 },
//             Ordering::Equal => { 0 },
//             Ordering::Less => { 1 },
//         },
//     }
// }

impl Body {
    // fn equal(&self, other: &Body) -> bool {
    //     return self.p.equal(&other.p) && self.velocity.equal(&other.velocity);
    // }

    fn kenetic(&self) -> u64 {
        return (self.p.x.abs() + self.p.y.abs() + self.p.z.abs()) as u64;
    }
    
    fn potential(&self) -> u64 {
        return (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()) as u64;
    }

    fn energy(&self) -> u64 {
        return self.kenetic() * self.potential();
    }

    fn apply_velocity(&self) -> Body {
        return Body {
            p: Point {
                x:self.p.x + self.velocity.x,
                y:self.p.y + self.velocity.y,
                z:self.p.z + self.velocity.z,
            },
            velocity: self.velocity
        };
    }

    fn calculate_gravity(&self, moons: &Vec<Body>) -> Point {
        let mut gravity = Point { x: 0, y: 0, z: 0 };

        for other in moons {
            gravity.x += match self.p.x.cmp(&other.p.x) {
                Ordering::Greater => { -1 },
                Ordering::Equal => { 0 },
                Ordering::Less => { 1 },
            };

            gravity.y += match self.p.y.cmp(&other.p.y) {
                Ordering::Greater => { -1 },
                Ordering::Equal => { 0 },
                Ordering::Less => { 1 },
            };

            gravity.z += match self.p.z.cmp(&other.p.z) {
                Ordering::Greater => { -1 },
                Ordering::Equal => { 0 },
                Ordering::Less => { 1 },
            };
        }


        return gravity;
    }

    fn apply_gravity(&self, gravity: &Point) -> Body {
        return Body { 
            p: self.p, 
            velocity: {
                Point {
                    x: self.velocity.x + gravity.x,
                    y: self.velocity.y + gravity.y,
                    z: self.velocity.z + gravity.z,
                }
            }
        };
    }
}

fn parse_moon(input: &str) -> Body {
    // <x=3, y=5, z=-1>
    let parts: Vec<&str> = input.split(&['<', '=', ',', ' ', '>']).collect();
    let x = parts[2].parse::<i64>().unwrap();
    let y = parts[5].parse::<i64>().unwrap();
    let z = parts[8].parse::<i64>().unwrap();
    return Body { p: Point { x:x, y:y, z:z }, velocity: Point { x:0, y:0, z:0 } };
}

fn read_data(filename: &str) -> Vec<Body> {
    // create vector of points and their value
    let points = fs::read_to_string(filename)
        .expect("Expected input file to exist")
        .split('\n')
        .map(|line| parse_moon(line))
        .collect();

    return points;
}

fn part1(moons: &Vec<Body>) -> u64 {
    let steps = match moons[0].p.y {
        0 => 10,    // test.txt
        -10 => 100, // test-2.txt
        _ => 1000,  // input.txt
    };

    let mut next = moons.clone();
    for _step in 1..=steps {
        next = next.iter()
            .map(|moon| (moon, moon.calculate_gravity(&next)))
            .map(|(moon, gravity)| moon.apply_gravity(&gravity))
            .map(|moon| moon.apply_velocity())
            .collect();
    }

    let energy = next.iter()
        .map(|moon| moon.energy())
        .sum();
        
    return energy;
}

// a hash key for the current position of all moons and the
// velocity of moon n
fn key_for(moon: &Body, gravity: &Point) -> u64 {
    let mut hasher = DefaultHasher::new();

    moon.p.x.hash(&mut hasher);
    moon.p.y.hash(&mut hasher);
    moon.p.z.hash(&mut hasher);

    moon.velocity.x.hash(&mut hasher);
    moon.velocity.y.hash(&mut hasher);
    moon.velocity.z.hash(&mut hasher);

    gravity.x.hash(&mut hasher);
    gravity.y.hash(&mut hasher);
    gravity.z.hash(&mut hasher);
    
    return hasher.finish();
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}
// Function to compute LCM of a vector
fn lcm_v(numbers: &Vec<u64>) -> u64 {
    if numbers.is_empty() {
        return 0;
    }

    let mut result = numbers[0];
    for &next in numbers.iter().skip(1) {
        result = lcm(result, next);
    }
    result
}

fn part2(moons: &Vec<Body>) -> u64 {
    let mut moon_caches: Vec<HashMap<u64, u64>> = Vec::new();
    for moon in moons {
        moon_caches.push(HashMap::new());
    }

    let mut repeater = vec![0; moons.len()];
    let mut repeats =  vec![0; moons.len()];

    let mut next = moons.clone();
    // for step in 1..=2773 {
    let mut step = 0;
    // while repeater.iter().any(|&n| n == 0) {
    while repeats.iter().any(|&n| n < 2) {
        let gravity: Vec<_> = next.iter()
            .map(|moon| moon.calculate_gravity(&next))
            .collect();

        // snapshot of current state of each moon for caching
        let states: Vec<_> = next.iter().enumerate()
            .map(|(i, moon)| key_for(moon, &gravity[i]))
            .collect();

        // print!("{step:4}: ");
        // let m = next[0];
        // let s = states[0];
        // println!("{s}: {m:?}");
    
        for n in 0..moons.len() {
            // if repeater[n] == 0 {
                if moon_caches[n].contains_key(&states[n]) {
                    let h = states[n];
                    let first = moon_caches[n].get(&states[n]).unwrap();
                    let diff = step - first;
                    println!("Moon {n} repeats at step {step:6} from {first:6} - {diff:6}");
                    moon_caches[n].insert(states[n], step);
                    repeater[n] = step;
                    repeats[n] += 1;
                } else {
                    moon_caches[n].insert(states[n].clone(), step.clone());
                }    
            // }
        }

        step += 1;
        next = next.iter()
            .map(|moon| (moon, moon.calculate_gravity(&next)))
            .map(|(moon, gravity)| moon.apply_gravity(&gravity))
            .map(|moon| moon.apply_velocity())
            .collect();
    }

    let result = lcm_v(&repeater);
    println!("repeater={repeater:?} => {result}");

    return moons.len() as u64;
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
