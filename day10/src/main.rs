use std::env;
use std::fs;
use std::process;
use std::time::Instant; // 0.8.2
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    c: char,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Neighbor {
    r: f64,
    point: Point,
}

impl Eq for Neighbor {}
impl Ord for Neighbor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.r.partial_cmp(&other.r)
            .unwrap_or_else(|| {    // put NaN at end
                if self.r.is_nan() && other.r.is_nan() {
                    Ordering::Equal
                } else if self.r.is_nan() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
    }
}
impl PartialOrd for Neighbor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// A point (x, y) with a map of all other points converted to polar coordinates
// the other points are stored in a BTreeMap keyed by their theta (with 0 being
// the positive Y axis, and clockwise increasing values). The neighbors are then
// in a vector kept sorted by their distance from the origin, closest first.
struct PolarPointView {
    x: usize,
    y: usize,
    radials: BTreeMap<u64, Vec<Neighbor>>
}

impl PolarPointView {
    // build a new PolarPointView from origin of all points
    fn build(origin: &Point, points: &Vec<&Point>) -> Self {
        let mut me = Self { 
            x: origin.x, 
            y: origin.y, 
            radials: BTreeMap::new()
        };

        points.iter().for_each(|p| me.insert(p));
        return me;
    }

    // insert point as polar coordinates relative to current origin point
    // stored as BTreeMap keyed on theta in vectors sorted by their distance
    // (r) from the origin.
    fn insert(&mut self, p: &Point) {
        // normalize p to self
        let x = p.x as f64 - self.x as f64;
        let y = p.y as f64 - self.y as f64;

        // standard angle from +X, CCW
        // rotate so 0 at +Y, flip to CW
        let r = ((x * x) + (y * y)).sqrt();
        let theta = (std::f64::consts::FRAC_PI_2 - -y.atan2(x)).rem_euclid(2.0 * std::f64::consts::PI);

        // don't insert self
        if r != 0.0 {
            // cant use f64 as BTreeMap key, use bit-pattern as usize
            let bucket = self.radials.entry(f64::to_bits(theta)).or_insert_with(Vec::new);
            let neighbor = Neighbor { r, point: p.clone() };
            match bucket.binary_search(&neighbor) {
                Ok(pos) | Err(pos) => bucket.insert(pos, neighbor)
            }
        }
    }
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

fn part1(points: &Vec<Point>) -> usize {
    let asteroids: Vec<&Point> = points.iter().collect();

    let base = asteroids.iter()
        .map(|asteroid| PolarPointView::build(*asteroid, &asteroids))
        .max_by_key(|asteroid| asteroid.radials.len())
        .unwrap();

    return base.radials.len();
}

fn part2(points: &Vec<Point>) -> usize {
    let asteroids: Vec<&Point> = points.iter().collect();

    // get best location for base and the asteroids that are visible from there
    let base = asteroids.iter()
        .map(|asteroid| PolarPointView::build(*asteroid, &asteroids))
        .max_by_key(|asteroid| asteroid.radials.len())
        .unwrap();

    // get list of theta values that are visible
    let thetas: Vec<_> = base.radials.keys().collect();

    // theta value of last asteroid to be zapped (based on test vs live input)
    let last_index = if thetas.len() < 200 { thetas.len() } else { 199 };

    // unwrap the original point for the last asteroid zapped
    let lasteroid = base.radials[thetas[last_index]].first().unwrap().point;

    // println!("Lasteroid = {lasteroid:?}");
    return lasteroid.x * 100 + lasteroid.y;
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
