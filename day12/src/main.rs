use std::env;
use std::process;
use std::fs;
use std::cmp::Ordering;
use std::time::Instant; // 0.8.2

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

// Just a point in space
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Point {
	x: i64,
	y: i64,
    z: i64,
}

// Represents a moon, has position and velocity
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Body {
    p: Point,
    velocity: Point,
}

impl Body {
    // Return kenetic energy
    fn kenetic(&self) -> u64 {
        return (self.p.x.abs() + self.p.y.abs() + self.p.z.abs()) as u64;
    }
    
    // Return potential energy
    fn potential(&self) -> u64 {
        return (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()) as u64;
    }

    // Return total energy
    fn energy(&self) -> u64 {
        return self.kenetic() * self.potential();
    }
    
    // Return calculated gravity adjustment in 3 dimensions
    fn calculate_gravity(&self, moons: &Vec<Body>) -> Point {
        // gravity between two objects as seen from point `a`
        let calculate = |a: i64, b: i64| {
            return match a.cmp(&b) {
                Ordering::Greater => { -1 },
                Ordering::Equal => { 0 },
                Ordering::Less => { 1 },
            };
        };

        // fold all gravity calculations into Point
        return moons.iter()
            .fold(Point { x:0, y:0, z:0 }, |gravity, other| Point {
                x: gravity.x + calculate(self.p.x, other.p.x),
                y: gravity.y + calculate(self.p.y, other.p.y),
                z: gravity.z + calculate(self.p.z, other.p.z)
            });
    }

    // // Return new body with gravity applied (possible new velocity)
    // fn apply_gravity(&self, gravity: &Point) -> Body {
    //     return Body { 
    //         p: self.p, 
    //         velocity: Point {
    //             x: self.velocity.x + gravity.x,
    //             y: self.velocity.y + gravity.y,
    //             z: self.velocity.z + gravity.z,
    //         }
    //     };
    // }

    // // Return new body with velocity applied (possible new position)
    // fn apply_velocity(&self) -> Body {
    //     return Body {
    //         p: Point {
    //             x: self.p.x + self.velocity.x,
    //             y: self.p.y + self.velocity.y,
    //             z: self.p.z + self.velocity.z,
    //         },
    //         velocity: self.velocity
    //     };
    // }

    // apply gravity and velocity in one step, return new Body
    fn next_state(&self, gravity: &Point) -> Body {
        return Body { 
            p: Point {
                x: self.p.x + self.velocity.x + gravity.x,
                y: self.p.y + self.velocity.y + gravity.y,
                z: self.p.z + self.velocity.z + gravity.z,
            },
            velocity: Point {
                x: self.velocity.x + gravity.x,
                y: self.velocity.y + gravity.y,
                z: self.velocity.z + gravity.z,
            }
        };
    }
}

// Parse a single line that represents a moon
// <x=3, y=5, z=-1>
fn parse_moon(input: &str) -> Body {
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
            .map(|(moon, gravity)| moon.next_state(&gravity))
            // .map(|(moon, gravity)| moon.apply_gravity(&gravity))
            // .map(|moon| moon.apply_velocity())
            .collect();
    }

    let energy = next.iter()
        .map(|moon| moon.energy())
        .sum();
        
    return energy;
}

// a hash key for the current position of all moons and the
// velocity of moon n
fn hash_state(moons: &Vec<Body>) -> Vec<u64> {
    let mut hashers = vec![
        DefaultHasher::new(),   // x
        DefaultHasher::new(),   // y
        DefaultHasher::new(),   // z
    ];

    for moon in moons {
        moon.p.x.hash(&mut hashers[0]);
        moon.velocity.x.hash(&mut hashers[0]);

        moon.p.y.hash(&mut hashers[1]);
        moon.velocity.y.hash(&mut hashers[1]);

        moon.p.z.hash(&mut hashers[2]);
        moon.velocity.z.hash(&mut hashers[2]);
    }
    
    return hashers.iter()
        .map(|h| h.finish())
        .collect();
}

// Return Greatest Common Divisor of two numbers
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Return Least Common Multiple of two numbers
fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}
// Return Least Common Multiple of a vector
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
    // cache of each axis states (all the ones we have seen before)
    let mut cache: Vec<HashMap<u64, u64>> = vec![
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];

    // size of the loop on each axis
    let mut axis_loop = vec![0; 3];

    // clone so we can work with them
    let mut next = moons.clone();
    let mut step = 0;

    // loop until we find a loop on all axes
    while axis_loop.iter().any(|&n| n == 0) {

        // compute hash of axis states
        let state = hash_state(&next);
        for (axis, &hash) in state.iter().enumerate() {

            if axis_loop[axis] == 0 && cache[axis].contains_key(&hash) {
                // if we have not already found the loop on this axis
                // and the cache contains this state -- we are finding the loop now
                let first = cache[axis].get(&hash).unwrap();
                axis_loop[axis] = step - first; // save the loop size
            } else {
                // otherwise, cache the state
                cache[axis].insert(hash, step);
            }
        }

        // step the system forward
        step += 1;
        next = next.iter()
            .map(|moon| (moon, moon.calculate_gravity(&next)))
            .map(|(moon, gravity)| moon.next_state(&gravity))
            // .map(|(moon, gravity)| moon.apply_gravity(&gravity))
            // .map(|moon| moon.apply_velocity())
            .collect();
    }

    // our answer is the LCM of the axis loop sizes
    let result = lcm_v(&axis_loop);
    return result;
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
