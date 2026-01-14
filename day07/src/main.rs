use std::collections::VecDeque;
use std::env;
use std::fs;
use std::iter::FromIterator;
use std::process;
use std::time::Instant; // 0.8.2
use std::ops::Range;

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

    fn add(&mut self, mode1: i64, mode2: i64) -> bool {
        let o1 = self.load(self.pc + 1, mode1);
        let o2 = self.load(self.pc + 2, mode2);
        let o3 = self.load(self.pc + 3, 1); // writes always use direct

        let p1 = self.format_operand(self.pc + 1, mode1);
        let p2 = self.format_operand(self.pc + 2, mode2);
        let p3 = self.format_operand(self.pc + 3, 1);
        println!("add\t{p1}, {p2} -> {p3}");

        self.store(usize::try_from(o3).unwrap(), o1 + o2);
        self.pc += 4;

        return true;
    }

    fn mul(&mut self, mode1: i64, mode2: i64) -> bool {
        let o1 = self.load(self.pc + 1, mode1);
        let o2 = self.load(self.pc + 2, mode2);
        let o3 = self.load(self.pc + 3, 1); // writes always use direct

        let p1 = self.format_operand(self.pc + 1, mode1);
        let p2 = self.format_operand(self.pc + 2, mode2);
        let p3 = self.format_operand(self.pc + 3, 1);
        println!("mul\t{p1}, {p2} -> {p3}");

        self.store(usize::try_from(o3).unwrap(), o1 * o2);
        self.pc += 4;

        return true;
    }

    fn inp(&mut self) -> bool {
        let o1 = self.load(self.pc + 1, 1); // writes always use direct

        if self.input.len() >= 1 {
            let value = self.input.pop_front().unwrap();

            let p1 = self.format_operand(self.pc + 1, 1);
            println!("inp\t{p1} << {value}");
            
            let address = usize::try_from(o1).unwrap();

            self.store(address, value);
            self.pc += 2;

            return true;
        }

        let p1 = self.format_operand(self.pc + 1, 1);
        println!("inp\t{p1} << No input available");
        return false;
    }

    fn out(&mut self, mode1: i64) -> bool {
        let o1 = self.load(self.pc + 1, mode1);

        let p1 = self.format_operand(self.pc + 1, 1);
        println!("out\t{p1}");

        self.output.push_back(o1);
        self.pc += 2;

        return true;
    }

    fn jit(&mut self, mode1: i64, mode2: i64) -> bool {
        let o1 = self.load(self.pc + 1, mode1);
        let o2 = self.load(self.pc + 2, mode2);

        let p1 = self.format_operand(self.pc + 1, mode1);
        let p2 = self.format_operand(self.pc + 2, mode2);
        println!("jit\t{p1}, {p2}");

        if o1 != 0 {
            self.pc = usize::try_from(o2).unwrap();
        } else {
            self.pc += 3;
        }

        return true;
    }

    fn jif(&mut self, mode1: i64, mode2: i64) -> bool {
        let o1 = self.load(self.pc + 1, mode1);
        let o2 = self.load(self.pc + 2, mode2);

        let p1 = self.format_operand(self.pc + 1, mode1);
        let p2 = self.format_operand(self.pc + 2, mode2);
        println!("jif\t{p1}, {p2}");

        if o1 == 0 {
            self.pc = usize::try_from(o2).unwrap();
        } else {
            self.pc += 3;
        }

        return true;
    }

    fn lt(&mut self, mode1: i64, mode2: i64) -> bool {
        let o1 = self.load(self.pc + 1, mode1);
        let o2 = self.load(self.pc + 2, mode2);
        let o3 = self.load(self.pc + 3, 1); // writes always use direct

        let p1 = self.format_operand(self.pc + 1, mode1);
        let p2 = self.format_operand(self.pc + 2, mode2);
        let p3 = self.format_operand(self.pc + 3, 1);
        println!("lt\t{p1}, {p2} -> {p3}");

        if o1 < o2 {
            self.store(usize::try_from(o3).unwrap(), 1);
        } else {
            self.store(usize::try_from(o3).unwrap(), 0);
        }

        self.pc += 4;

        return true;
    }

    fn eq(&mut self, mode1: i64, mode2: i64)  -> bool {
        let o1 = self.load(self.pc + 1, mode1);
        let o2 = self.load(self.pc + 2, mode2);
        let o3 = self.load(self.pc + 3, 1); // writes always use direct

        let p1 = self.format_operand(self.pc + 1, mode1);
        let p2 = self.format_operand(self.pc + 2, mode2);
        let p3 = self.format_operand(self.pc + 3, 1);
        println!("eq\t{p1}, {p2} -> {p3}");

        if o1 == o2 {
            self.store(usize::try_from(o3).unwrap(), 1);
        } else {
            self.store(usize::try_from(o3).unwrap(), 0);
        }

        self.pc += 4;
        return true;
    }

    fn end(&mut self) -> bool {
        println!("end\n");

        self.pc = self.memory.len();
        return true;
    }

    fn unknown(&self, op: i64) -> bool {
        println!("{op}\tunknown operation");

        return false;
    }

    fn is_terminated(&self) -> bool {
        return self.pc >= self.memory.len();
    }

    fn step(&mut self) -> bool {
        if self.is_terminated() {
            return false;
        }

        let instruction = self.memory[self.pc];
        let op = instruction % 100;
        let m1 = (instruction / 100) % 10;
        let m2 = (instruction / 1000) % 10;
        // let m3 = (instruction / 10000) % 10;

        let pc = self.pc;
        print!("{pc:04X}:\t");

        return match op {
            1 => self.add(m1, m2),
            2 => self.mul(m1, m2),
            3 => self.inp(),
            4 => self.out(m1),
            5 => self.jit(m1, m2),
            6 => self.jif(m1, m2),
            7 => self.lt(m1, m2),
            8 => self.eq(m1, m2),
            99 => self.end(),
            n => self.unknown(n),
        };
    }

    fn run(&mut self) {
        // self.show();
        while self.step() {
            // self.show();
        }
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
    fn format_operand(&self, addr: usize, mode: i64) -> String {
        let s_mode = match mode {
            1   => "*",
            _   => "",
        };
        let value = self.load(addr, mode);

        format!("{s_mode}{addr}={value}")
    }

    #[allow(dead_code)]
    fn show_output(&self) {
        for (i, v) in self.output.iter().enumerate() {
            println!("test {i} ==> {v}");
        }
    }

    #[allow(dead_code)]
    fn push_input(&mut self, data: i64) {
        self.input.push_back(data);
    }

    #[allow(dead_code)]
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

fn permutations(range: Range<i64>) -> Vec<Vec<i64>> {
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

    let mut phases: Vec<i64> = range.collect();
    let mut permutations = Vec::new();
    heaps_permute(phases.len(), &mut phases, &mut permutations);
	return permutations;
}

fn part1(program: &Vec<i64>) -> i64 {
    fn run(program: &Vec<i64>, input: &Vec<i64>) -> i64 {
        let mut cpu = build_cpu(&program, input);
        cpu.run();
        return cpu.pop_output();
    }
    
	let result = permutations(0..5)
		.iter()
		.map(|input| input.iter()	// for each permutation, compute it's output power
		                .fold(0, |power, &phase| run(&program, &vec![phase, power])))
		.max().unwrap();

    return result;
}


fn part2(_program: &Vec<i64>) -> i64 {
    // for configuration in phase_permutations() {
    //     let mut systems = vec![
    //         build_cpu(&program, &vec![configuration[0], 0]),
    //         build_cpu(&program, &vec![configuration[1]]),
    //         build_cpu(&program, &vec![configuration[2]]),
    //         build_cpu(&program, &vec![configuration[3]])
    //     ];

    //     for (i, cpu) in systems.iter().enumerate() {
    //         cpu.run();
    //         systems[cpu + 1 % systems.len()]
    //     }
    // }

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
