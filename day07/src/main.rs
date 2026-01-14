use std::collections::VecDeque;
use std::env;
use std::fs;
use std::iter::FromIterator;
use std::process;
use std::time::Instant; // 0.8.2

struct CPU {
    pc: usize,        // program counter
    memory: Vec<i64>, // memory
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

impl CPU {
    fn store(&mut self, address: usize, value: i64) {
        self.memory[address] = value;
    }

    fn load(&self, address: usize, direct: i64) -> i64 {
        let operand = self.memory[address];

        if direct == 0 {
            let address = usize::try_from(operand).unwrap();
            return self.memory[address];
        }

        return operand;
    }

    fn add(&mut self, mode1: i64, mode2: i64) {
        let o1 = self.load(self.pc + 1, mode1);
        let o2 = self.load(self.pc + 2, mode2);
        let o3 = self.load(self.pc + 3, 1); // writes always use direct
                                            // println!("add {o1}:{mode1} + {o2}:{mode2} => {o3}");

        self.store(usize::try_from(o3).unwrap(), o1 + o2);
        self.pc += 4;
    }

    fn mul(&mut self, mode1: i64, mode2: i64) {
        let o1 = self.load(self.pc + 1, mode1);
        let o2 = self.load(self.pc + 2, mode2);
        let o3 = self.load(self.pc + 3, 1); // writes always use direct
                                            // println!("mul {o1}:{mode1} * {o2}:{mode2} => {o3}");

        self.store(usize::try_from(o3).unwrap(), o1 * o2);
        self.pc += 4;
    }

    fn inp(&mut self, _mode1: i64) {
        if self.input.len() >= 1 {
            let value = self.input.pop_front().unwrap();

            let o1 = self.load(self.pc + 1, 1); // writes always use direct
            let address = usize::try_from(o1).unwrap();

            self.store(address, value);
            self.pc += 2;

            // println!("inp {address}:{_mode1} => {value}");
        }
    }

    fn out(&mut self, mode1: i64) {
        let value = self.load(self.pc + 1, mode1);
        self.output.push_back(value);
        self.pc += 2;

        // let address = self.memory[self.pc+1];
        // println!("out {address}:{mode1} => {value}");
    }

    fn jit(&mut self, mode1: i64, mode2: i64) {
        let o1 = self.load(self.pc + 1, mode1);
        let o2 = self.load(self.pc + 2, mode2);
        // println!("jit {o1}:{mode1}, {o2}:{mode2}");

        if o1 != 0 {
            self.pc = usize::try_from(o2).unwrap();
        } else {
            self.pc += 3;
        }
    }

    fn jif(&mut self, mode1: i64, mode2: i64) {
        let o1 = self.load(self.pc + 1, mode1);
        let o2 = self.load(self.pc + 2, mode2);
        // println!("jif {o1}:{mode1}, {o2}:{mode2}");

        if o1 == 0 {
            self.pc = usize::try_from(o2).unwrap();
        } else {
            self.pc += 3;
        }
    }

    fn lt(&mut self, mode1: i64, mode2: i64) {
        let o1 = self.load(self.pc + 1, mode1);
        let o2 = self.load(self.pc + 2, mode2);
        let o3 = self.load(self.pc + 3, 1); // writes always use direct
                                            // println!("lt {o1}:{mode1} < {o2}:{mode2} => {o3}");

        if o1 < o2 {
            self.store(usize::try_from(o3).unwrap(), 1);
        } else {
            self.store(usize::try_from(o3).unwrap(), 0);
        }

        self.pc += 4;
    }

    fn eq(&mut self, mode1: i64, mode2: i64) {
        let o1 = self.load(self.pc + 1, mode1);
        let o2 = self.load(self.pc + 2, mode2);
        let o3 = self.load(self.pc + 3, 1); // writes always use direct
                                            // println!("eq {o1}:{mode1} == {o2}:{mode2} => {o3}");

        if o1 == o2 {
            self.store(usize::try_from(o3).unwrap(), 1);
        } else {
            self.store(usize::try_from(o3).unwrap(), 0);
        }

        self.pc += 4;
    }

    fn term(&mut self) {
        // println!("term");
        self.pc = self.memory.len();
    }

    fn step(&mut self) {
        let pc = self.pc;

        let instruction = self.memory[self.pc];
        let op = instruction % 100;
        let m1 = (instruction / 100) % 10;
        let m2 = (instruction / 1000) % 10;
        // let m3 = (instruction / 10000) % 10;

        if self.pc < self.memory.len() {
            match op {
                1 => self.add(m1, m2),
                2 => self.mul(m1, m2),
                3 => self.inp(m1),
                4 => self.out(m1),
                5 => self.jit(m1, m2),
                6 => self.jif(m1, m2),
                7 => self.lt(m1, m2),
                8 => self.eq(m1, m2),
                99 => self.term(),
                n => println!("{pc}: unknown operation {n}"),
            }
        }
    }

    fn run(&mut self) -> i64 {
        // self.show();
        while self.pc < self.memory.len() {
            self.step();
            // self.show();
        }

        return self.memory[0];
    }

    #[allow(dead_code)]
    fn show(&self) {
        for (i, v) in self.memory.iter().enumerate() {
            if i == self.pc {
                print!("({v})");
            } else {
                print!("{v}");
            }
            if i < self.memory.len() - 1 {
                print!(",");
            }
        }
        println!("")
    }

    #[allow(dead_code)]
    fn show_output(&self) {
        for (i, v) in self.output.iter().enumerate() {
            println!("test {i} ==> {v}");
        }
    }

    fn push_input(&mut self, data: i64) {
        self.input.push_back(data);
    }

    fn pop_output(&mut self) -> i64 {
        return match self.output.pop_front() {
            Some(out) => out,
            None => 0
        };
    }
}

fn build_cpu(program: &Vec<i64>, input: &Vec<i64>) -> CPU {
    return CPU {
        pc: 0,
        memory: program.clone(),
        input: VecDeque::from_iter(input.clone()),
        output: VecDeque::new(),
    };
}

fn read_data(filename: &str) -> Vec<i64> {
    fs::read_to_string(filename)
        .expect("Expected input file to exist")
        .replace("\n", "")
        .split(',')
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn run_program(program: &Vec<i64>, input: &Vec<i64>) -> i64 {
    let mut cpu = build_cpu(&program, input);
    cpu.run();
    return cpu.pop_output();
}

// return all permutations of a without replacement
// https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn heaps_permute(k: usize, a: &mut [i64], out: &mut Vec<Vec<i64>>) {
    if k == 1 {
        out.push(a.to_vec());
    } else {
        heaps_permute(k - 1, a, out);
        for i in 0..(k - 1) {
            if k % 2 == 0 {
                a.swap(i, k - 1);
            } else {
                a.swap(0, k - 1);
            }
            heaps_permute(k - 1, a, out);
        }
    }
}

fn phase_permutations() -> Vec<Vec<i64>> {
    let mut phases = vec![0, 1, 2, 3, 4];
    let mut permutations = Vec::new();
    heaps_permute(phases.len(), &mut phases, &mut permutations);
	return permutations;
}

fn part1(program: &Vec<i64>) -> i64 {
	let result = phase_permutations()
		.iter()
		.map(|input| input	// for each permutation, compute it's output power
				.iter()
				.fold(0, |power, &phase| run_program(&program, &vec![phase, power])))
		.max().unwrap();

    return result;
}

fn part2(_program: &Vec<i64>) -> i64 {
    return 0;
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
