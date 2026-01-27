use std::env;
use std::process;
use std::fs;
use std::fmt::Write;
use std::time::Instant; // 0.8.2
use std::collections::HashMap;
pub mod intcode;

#[derive(Clone, Debug)]
struct Screen {
    //buffer: Vec<Vec<Pixel>>,
    buffer: HashMap<(i64, i64), i64>,
    x_max: i64,
    y_max: i64,
}

impl Screen {
    fn create() -> Self {
        return Screen {
            buffer: HashMap::new(),
            x_max: 0, y_max: 0,
        };
    }

    fn set(&mut self, x: i64, y: i64, value: i64) {
        // println!("set {x},{y}->{value}");
        self.buffer.insert((x, y), value);

        self.x_max = x.max(self.x_max);
        self.y_max = y.max(self.y_max);
    }

    fn get(&self, x: i64, y: i64) -> i64 {
        return match self.buffer.get(&(x, y)) {
            Some(value) => *value,
            None => 0,
        };
    }

    fn show(&self) {
        let score = self.get(-1, 0);
        let mut raw = "\x1b[2J\x1b[H".to_string();
        write!(raw, "Score: {score}\n").expect("failed");

        for y in 0..=self.y_max {
            for x in 0..=self.x_max {
                match self.get(x, y) {
                    1 => write!(raw, "\x1b[0m\x1b[40m#").expect("failed"),
                    2 => write!(raw, "\x1b[0;37m\x1b[41mB").expect("failed"),
                    3 => write!(raw, "\x1b[0;97m\x1b[42mP").expect("failed"),
                    4 => write!(raw, "\x1b[1;93m\x1b[40mO").expect("failed"),
                    _ => write!(raw, "\x1b[0m ").expect("failed"),
                };
            }
            write!(raw, "\n").expect("failed");
        }
        write!(raw, "\n").expect("failed");
        print!("{raw}\x1b[0m");
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

#[allow(dead_code)]
fn read_input(filename: &str) -> Vec<i64> {
    // create vector of points and their value
    let input = fs::read_to_string(filename)
        .expect("Expected input file to exist")
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    return input;
}

#[allow(dead_code)]
fn playback(program: &Vec<i64>, filename: &str) -> usize {
    let mut screen = Screen::create();
    let mut cpu = intcode::CPU::load_program(program);

    let mut keys: Vec<i64> = read_input(filename).into_iter().rev().collect();

    // set free play mode
    cpu.set_memory(0, 2);

    while !cpu.is_terminated() {
        cpu.run();

        if cpu.output_len() > 1 {
            cpu.output()
                .chunks(3)
                .for_each(|p| screen.set(p[0], p[1], p[2]));

            screen.show();
        }

        let k = keys.pop().unwrap();
        println!("send {k}");
        cpu.push_input(k);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    return screen.get(-1, 0) as usize;
}

// interactive version with "undo" that records and prints out input
// a = left
// d = right
// s or return = nothing
// w = reWind
// q = quit
// play this version until you win, then copy-pate the output to keys.txt
// to use playback()
#[allow(dead_code)]
fn interactive(program: &Vec<i64>) -> usize {
    let mut screen = Screen::create();
    let mut cpu = intcode::CPU::load_program(program);

    let mut save: Vec<(intcode::CPU, Screen, Vec<char>)> = Vec::new();
    let mut keys = Vec::new();

    // set free play mode
    cpu.set_memory(0, 2);

    while !cpu.is_terminated() {
        // limit save states to 100, do it for the memory!
        while save.len() > 100 {
            save.remove(0);
        }
        // save current state of CPU, screen and keylogger
        save.push((cpu.clone(), screen.clone(), keys.clone()));


        cpu.run();

        // if output, update screen
        if cpu.output_len() > 1 {
            cpu.output()
                .chunks(3)
                .for_each(|p| screen.set(p[0], p[1], p[2]));

            screen.show();
        }

        // you can never lose. this will pop back to before you lost
        if screen.will_end() {
            save.pop();
            (cpu, screen, keys) = save.pop().unwrap();
            screen.show();
            println!("RESCUE");
            continue;
        }

        // get input -- with rust you need to hit key and then return
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("failed to read");

        keys.push(line.chars().nth(0).unwrap());
        match line.chars().nth(0).unwrap() {
            'q' => break,                       // quit
            'a' => cpu.push_input(-1),          // left
            's' => cpu.push_input(0),           // nothing
            'd' => cpu.push_input(1),           // right
            'w' => {                            // rewind
                save.pop();
                (cpu, screen, keys) = save.pop().unwrap();
                screen.show();
                println!("REVERSE");
            },
            _ => cpu.push_input(0),             // nothing
        };
    }

    // print out keys we hit for replay
    let keymap: Vec<_> = keys.iter()
        .map(|k| match k {
            'a' => -1,
            'd' => 1,
            's' => 0,
            '\n' => 0,
            _ => 2
        })
        .collect();

    println!("{keymap:?}");
    return screen.get(-1, 0) as usize;
}

fn part2(program: &Vec<i64>) -> usize {
    return interactive(program);
    // return playback(program, "keys.txt");
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
