use std::env;
use std::process;
use std::fs;
use std::cmp::Ordering;
use std::time::Instant; // 0.8.2

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
	fn add(&mut self, other: &Point) {
		self.x += other.x;
		self.y += other.y;
        self.z += other.z;
	}
} 

impl Body {
    fn kenetic(&self) -> u64 {
        return (self.p.x.abs() + self.p.y.abs() + self.p.z.abs()) as u64;
    }
    
    fn potential(&self) -> u64 {
        return (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()) as u64;
    }

    fn energy(&self) -> u64 {
        // let k = self.kenetic();
        // let p = self.potential();
        // println!("{self:?} k={k},p={p}");
        return self.kenetic() * self.potential();
    }
}

fn pair_gravity(b1: &Body, b2: &Body) -> Point {
    return Point {
        x: match b1.p.x.cmp(&b2.p.x) {
            Ordering::Greater => { -1 },
            Ordering::Equal => { 0 },
            Ordering::Less => { 1 },
        },
        y: match b1.p.y.cmp(&b2.p.y) {
            Ordering::Greater => { -1 },
            Ordering::Equal => { 0 },
            Ordering::Less => { 1 },
        },
        z: match b1.p.z.cmp(&b2.p.z) {
            Ordering::Greater => { -1 },
            Ordering::Equal => { 0 },
            Ordering::Less => { 1 },
        },
    }
}

fn physics(moons: &Vec<Body>) -> Vec<Body> {
    // gravity is starting velocity of moon
    let mut gravity: Vec<Point> = moons.iter()
        .map(|moon| moon.velocity)
        .collect();

    // update with all pair-wise gravity
    for i in 0..moons.len() {
        for j in i+1..moons.len() {
            gravity[i].add(&pair_gravity(&moons[i], &moons[j]));
            gravity[j].add(&pair_gravity(&moons[j], &moons[i]));
        }
    }

    return moons.iter().enumerate()
        // apply gravity to moons
        .map(|(i, moon)| Body { p: moon.p, velocity: gravity[i] })
        // apply (calculated) velocity to moons
        .map(|moon| Body { 
            p: Point { 
                x:moon.p.x + moon.velocity.x,
                y:moon.p.y + moon.velocity.y,
                z:moon.p.z + moon.velocity.z,
            },
            velocity: moon.velocity 
        })
        .collect();
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
    // for moon in moons {
    //     println!("{moon:?}");
    // }

    let steps = match moons[0].p.y {
        0 => 10,    // test.txt
        -10 => 100, // test-2.txt
        _ => 1000,  // input.txt
    };

    let mut next = moons.clone();
    for _step in 1..=steps {
        next = physics(&next);

        // let energy: u64 = next.iter()
        //     .map(|moon| moon.energy())
        //     .sum();

        // println!("{step:4}: {energy}");
        // println!();
        // for moon in &next {
        //     println!("{moon:?}");
        // }    
    }

    let energy = next.iter()
        .map(|moon| moon.energy())
        .sum();
        
    return energy;
}

fn part2(moons: &Vec<Body>) -> u64 {

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
