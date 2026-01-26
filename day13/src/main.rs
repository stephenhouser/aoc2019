use std::env;
use std::process;
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

#[derive(Clone, Debug)]
struct Screen {
    //buffer: Vec<Vec<Pixel>>,
    buffer: HashMap<(i64, i64), i64>,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,

}

impl Screen {
    fn create() -> Self {
        return Screen {
            buffer: HashMap::new(),
            x_min: 0, x_max: 0, 
            y_min: 0, y_max: 0,
        };
    }

    fn set(&mut self, x: i64, y: i64, value: i64) {
        // println!("set {x},{y}->{value}");
        self.buffer.insert((x, y), value);

        // self.x_min = x.min(self.x_min);
        self.x_max = x.max(self.x_max);
        // self.y_min = y.min(self.y_min);
        self.y_max = y.max(self.y_max);
    }

    fn get(&self, x: i64, y: i64) -> i64 {
        return match self.buffer.get(&(x, y)) {
            Some(value) => *value,
            None => 0,
        };
    }

    fn show(&self) {
        print!("\x1b[2J\x1b[H");
        for y in 0..=self.y_max {
            for x in 0..=self.x_max {
                match self.get(x, y) {
                    1 => print!("#"),
                    2 => print!("b"),
                    3 => print!("-"),
                    4 => print!("O"),
                    _ => print!(" "),
                }
            }
            println!();
        }
        println!();
        let score = self.get(-1, 0);
        println!("Score: {score}");
    }

    fn will_end(&self) -> bool {
        for x in 0..=self.x_max {
            if self.get(x, self.y_max) == 4 {
                return true;
            }
        }

        return false;
    }
}

fn read_data(filename: &str) -> Vec<i64>  {
    return intcode::read_program(filename);
}

fn part1(program: &Vec<i64>) -> usize {
    let mut cpu = intcode::CPU::load_program(program);

    cpu.run();

    // count block tiles (2) that are produced
    let output = cpu.output();
    let blocks: Vec<_> = output
        .chunks(3)
        .filter(|&threesome| threesome[2] == 2)
        .collect();

    return blocks.len();
}

fn part2(program: &Vec<i64>) -> usize {
    let mut screen = Screen::create();
    let mut cpu = intcode::CPU::load_program(program);

    let mut save: Vec<(intcode::CPU, Screen)> = Vec::new();

    // set free play mode
    cpu.set_memory(0, 2);

    while !cpu.is_terminated() {
        while save.len() > 100 {
            save.remove(0);
        }
        save.push((cpu.clone(), screen.clone()));

        cpu.run();

        if cpu.output_len() > 1 {
            cpu.output()
                .chunks(3)
                .for_each(|p| screen.set(p[0], p[1], p[2]));

            screen.show();
        }

        if screen.will_end() {
            save.pop();
            (cpu, screen) = save.pop().unwrap();
            screen.show();
            println!("RESCUE");
            continue;
        }

        let mut line = String::new();
        let input = std::io::stdin().read_line(&mut line).expect("failed to read");
        match line.chars().nth(0).unwrap() {
            'a' => cpu.push_input(-1),
            's' => cpu.push_input(0),
            'd' => cpu.push_input(1),
            'w' => {
                save.pop();
                (cpu, screen) = save.pop().unwrap();
                screen.show();
                println!("RESTORE");
            },
            _ => cpu.push_input(0),
        };
    }

    return program.len();
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
