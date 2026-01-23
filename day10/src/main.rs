use std::env;
use std::fs;
use std::process;
use std::time::Instant; // 0.8.2

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    c: char,
}

#[derive(Debug, Clone, Copy)]
struct PolarPoint {
    #[allow(dead_code)]
    r: f64,
    theta: f64,
    p: Point,
}

fn read_data(filename: &str) -> Vec<Point> {
    // create vector of points and their value
    let points = fs::read_to_string(filename)
        .expect("Expected input file to exist")
        .split('\n')
        .enumerate()
        .map(|(y, line)| line.chars()
            .enumerate()
            .map(|(x, c)| Point { x:x, y:y, c:c } )
            .collect::<Vec<Point>>()
        )
        .flatten()
        .filter(|p| p.c == '#')
        .collect();

    return points;
}

// is there another point blocking the path from p1 to p2
fn is_blocked(points: &Vec<&Point>, p1: &Point, p2: &Point) -> bool {
    // print!("{p1:?},{p2:?} ");

    // vertical line
    if p1.x == p2.x {
        let x = p1.x;
        let y1 = std::cmp::min(p1.y, p2.y);
        let y2 = std::cmp::max(p1.y, p2.y);

        // check all possible points along path...
        for y in y1+1..y2 {
            // if there are any points, this path is blocked
            if points.iter().any(|p| p.x == x && p.y == y) {
                // println!(" blocked by {x},{y}");
                return true;
            }
        }

        // println!("");
        return false;
    }


    let (x1, y1, x2, y2) = if p1.x < p2.x {
        (p1.x, p1.y, p2.x, p2.y)
    } else {
        (p2.x, p2.y, p1.x, p1.y)
    };

    let dx = x2 as f64 - x1 as f64;
    let dy = y2 as f64 - y1 as f64;
    let m = dy / dx;
    // print!("{x1},{y1} {x2},{y2} slope {m}");
    
    // non-vertical line
    // if there are any points along the path...
    for x in x1+1..x2 {
        let y = y1 as f64 + m * (x as f64 - x1 as f64);

        // if y is a whole number and
        // if there are any points, this path is blocked
        if y.fract() == 0.0 {
            let y = y as usize;
            if points.iter().any(|p| p.x == x && p.y == y) {
                // println!(" blocked by {x},{y}");
                return true;
            }
        }
    }

    // println!("");
    return false;
}

fn same(p1: &Point, p2: &Point) -> bool {
    return (p1.x == p2.x) && (p1.y == p2.y);
}

// for point return vec of other visible points
fn visible_points(points: &Vec<&Point>, p: &Point) -> Vec<Point> {
    let mut visible = Vec::new();

    // let x = p.x;
    // let y = p.y;
    // println!("\n{x},{y}");
    for op in points {
        if !same(&p, &op) && !is_blocked(&points, &p, &op) {
            visible.push((*op).clone());
        }
    }

    // let n = visible.len();
    // println!("\n{x},{y} --> {n}");
    return visible;
}

// p1 is 0,0, p2 is what we want polar for
fn to_polar(p1: &Point, p2: &Point) -> PolarPoint {
    // println!("p1={p1:?}, p2={p2:?}");
    // normalize to p1 as 0,0
    let x = p2.x as f64 - p1.x as f64;
    let y = p2.y as f64 - p1.y as f64;

    let r = ((x * x) + (y * y)).sqrt();

    // standard angle from +X, CCW
    let a = -y.atan2(x);
    // rotate so 0 at +Y, flip to CW
    let theta = (std::f64::consts::FRAC_PI_2 - a).rem_euclid(2.0 * std::f64::consts::PI);
   
    return PolarPoint { r: r, theta: theta, p: p2.clone()};
}

fn part1(points: &Vec<Point>) -> usize {
    let asteroids: Vec<&Point> = points.iter().collect();

    // base = ( Point, Vec<Point> of visible asteriods )
    let base = asteroids.iter()
        .map(|asteroid| (asteroid, visible_points(&asteroids, &asteroid)))
        .max_by_key(|(_asteroid, visible)| visible.len())
        .unwrap();

    return base.1.len();
}

fn part2(points: &Vec<Point>) -> usize {
    let asteroids: Vec<&Point> = points.iter().collect();

    // get best location for base and the asteroids that are visible from there
    let (base, visible) = asteroids.iter()
        .map(|asteroid| (asteroid, visible_points(&asteroids, &asteroid)))
        .max_by_key(|(_asteroid, visible)| visible.len())
        .unwrap();

    // convert the visible asteroids to polar coordinates from the perspective
    // of the chosen base asteroid.
    let mut polars: Vec<_> = visible.iter()
        .map(|asteroid| to_polar(base, asteroid))
        .collect();

    // sort by angle (theta)
    polars.sort_by(|a, b| a.theta.partial_cmp(&b.theta).unwrap());

    // find the 199th (or last) asteroid to be blasted
    let lasteroid = if polars.len() < 100 { 
        polars[polars.len()-1] 
    } else { 
        polars[199]
    };

    println!("Lasteroid = {lasteroid:?}");
    return lasteroid.p.x * 100 + lasteroid.p.y;
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
