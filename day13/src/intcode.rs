use std::fs;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct CPU {
    pc: usize,        // program counter
    memory: Vec<i64>, // memory
    base: i64,        // relative base (day 9)
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    debug: bool,
}

impl CPU {    
    pub fn load_program(program: &Vec<i64>) -> CPU {
        return CPU {
            pc: 0,
            memory: program.clone(),
            base: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            debug: false,
        };
    }
    
    pub fn set_memory(&mut self, address: usize, value: i64) {
        self.memory[address] = value;
    }

    fn load_direct(&self, address: usize) -> i64 {
        if self.memory.len() <= address {
            return 0;
        }

        return self.memory[address];
    }

    fn store(&mut self, address: usize, mode: i64, value: i64) {
        let operand = self.load_direct(address);

        let mut position_mode = |operand, value| {
            let address = usize::try_from(operand).expect("Bad memory address");
            if self.memory.len() <= address {
                self.memory.resize(address+1, 0);
            }

            // println!("WRITE: {address}: {value}");
            self.memory[address] = value;
        };

        let mut relative_mode = |operand, value| {
            position_mode(self.base + operand, value);
        };

        match mode {
            0 => { position_mode(operand, value) },
            1 => {  },
            2 => { relative_mode(operand, value) },
            _ => {  }
        }
    }

    fn load(&self, address: usize, mode: i64) -> i64 {
        // println!("load {address}, {mode}");
        let operand = self.load_direct(address);

        let position_mode = |operand| {
            let address = usize::try_from(operand).expect("Bad memory address");
            return self.load_direct(address);
        };

        let relative_mode = |operand| {
            return position_mode(self.base + operand);
        };

        match mode {
            0 => { position_mode(operand) },
            1 => { operand },
            2 => { relative_mode(operand) },
            _ => { 0 }
        }
    }

    fn add(&mut self, mode: &Vec<i64>) -> bool {
        assert!(mode.len() >= 2);
        let o1 = self.load(self.pc + 1, mode[0]);
        let o2 = self.load(self.pc + 2, mode[1]);
        // let o3 = self.load(self.pc + 3, 1); // writes always use direct

        if self.debug {
            let p1 = self.format_operand(self.pc + 1, mode[0]);
            let p2 = self.format_operand(self.pc + 2, mode[1]);
            let p3 = self.format_operand(self.pc + 3, mode[2]);
            println!("add\t{p1}, {p2}, {p3}");
        }

        self.store(self.pc + 3, mode[2], o1 + o2);
        self.pc += 4;

        return true;
    }

    fn mul(&mut self, mode: &Vec<i64>) -> bool {
        assert!(mode.len() >= 2);
        let o1 = self.load(self.pc + 1, mode[0]);
        let o2 = self.load(self.pc + 2, mode[1]);
        // let o3 = self.load(self.pc + 3, 1); // writes always use direct

        if self.debug {
            let p1 = self.format_operand(self.pc + 1, mode[0]);
            let p2 = self.format_operand(self.pc + 2, mode[1]);
            let p3 = self.format_operand(self.pc + 3, mode[2]);
            println!("mul\t{p1}, {p2}, {p3}");
        }

        // self.store(usize::try_from(o3).unwrap(), o1 * o2);
        self.store(self.pc + 3, mode[2], o1 * o2);
        self.pc += 4;

        return true;
    }

    fn inp(&mut self, mode: &Vec<i64>) -> bool {
        // assert!(mode.len() >= 0);
        // let o1 = self.load(self.pc + 1, mode[0]); // writes always use direct

        if self.input.len() >= 1 {
            let value = self.input.pop_front().unwrap();

            if self.debug {
                let p1 = self.format_operand(self.pc + 1, mode[0]);
                println!("inp\t{p1} <== {value}");
            }
            
            // let address = usize::try_from(o1).unwrap();
            // self.store(address, value);
            self.store(self.pc + 1, mode[0], value);
            self.pc += 2;

            return true;
        }

        if self.debug {
            let p1 = self.format_operand(self.pc + 1, mode[0]);
            println!("inp\t{p1} << No input available (pause)\n");
        }
        
        return false;
    }

    fn out(&mut self, mode: &Vec<i64>) -> bool {
        assert!(mode.len() >= 1);
        let o1 = self.load(self.pc + 1, mode[0]);

        if self.debug {
            let p1 = self.format_operand(self.pc + 1, mode[0]);
            println!("out\t{p1} ==> {o1}");
        }

        self.output.push_back(o1);
        self.pc += 2;

        return true;
    }

    fn jit(&mut self, mode: &Vec<i64>) -> bool {
        assert!(mode.len() >= 2);
        let o1 = self.load(self.pc + 1, mode[0]);
        let o2 = self.load(self.pc + 2, mode[1]);

        if self.debug {
            let p1 = self.format_operand(self.pc + 1, mode[0]);
            let p2 = self.format_operand(self.pc + 2, mode[1]);
            println!("jit\t{p1}, {p2}");
        }
        
        if o1 != 0 {
            self.pc = usize::try_from(o2).unwrap();
        } else {
            self.pc += 3;
        }

        return true;
    }

    fn jif(&mut self, mode: &Vec<i64>) -> bool {
        assert!(mode.len() >= 2);
        let o1 = self.load(self.pc + 1, mode[0]);
        let o2 = self.load(self.pc + 2, mode[1]);

        if self.debug {
            let p1 = self.format_operand(self.pc + 1, mode[0]);
            let p2 = self.format_operand(self.pc + 2, mode[1]);
            println!("jif\t{p1}, {p2}");
        }

        if o1 == 0 {
            self.pc = usize::try_from(o2).unwrap();
        } else {
            self.pc += 3;
        }

        return true;
    }

    fn lt(&mut self, mode: &Vec<i64>) -> bool {
        assert!(mode.len() >= 2);
        let o1 = self.load(self.pc + 1, mode[0]);
        let o2 = self.load(self.pc + 2, mode[1]);
        // let o3 = self.load(self.pc + 3, 1); // writes always use direct

        if self.debug {
            let p1 = self.format_operand(self.pc + 1, mode[0]);
            let p2 = self.format_operand(self.pc + 2, mode[1]);
            let p3 = self.format_operand(self.pc + 3, mode[2]);
            println!("lt\t{p1}, {p2}, {p3}");
        }

        if o1 < o2 {
            // self.store(usize::try_from(o3).unwrap(), 1);
            self.store(self.pc + 3, mode[2], 1);
        } else {
            // self.store(usize::try_from(o3).unwrap(), 0);
            self.store(self.pc + 3, mode[2], 0);
        }

        self.pc += 4;

        return true;
    }

    fn eq(&mut self, mode: &Vec<i64>) -> bool {
        assert!(mode.len() >= 2);
        let o1 = self.load(self.pc + 1, mode[0]);
        let o2 = self.load(self.pc + 2, mode[1]);
        // let o3 = self.load(self.pc + 3, 1); // writes always use direct

        if self.debug {
            let p1 = self.format_operand(self.pc + 1, mode[0]);
            let p2 = self.format_operand(self.pc + 2, mode[1]);
            let p3 = self.format_operand(self.pc + 3, mode[2]);
            println!("eq\t{p1}, {p2}, {p3}");
        }

        if o1 == o2 {
            // self.store(usize::try_from(o3).unwrap(), 1);
            self.store(self.pc + 3, mode[2], 1);
        } else {
            // self.store(usize::try_from(o3).unwrap(), 0);
            self.store(self.pc + 3, mode[2], 0);
        }

        self.pc += 4;
        return true;
    }

    fn base(&mut self, mode: &Vec<i64>) -> bool {
        assert!(mode.len() >= 1);
        let o1 = self.load(self.pc + 1, mode[0]);

        if self.debug {
            let p1 = self.format_operand(self.pc + 1, mode[0]);
            println!("bas\t{p1}");
        }

        self.base += o1;

        self.pc += 2;
        return true;
    }

    fn end(&mut self, _mode: &Vec<i64>) -> bool {
        // assert!(mode.len() >= 0);

        if self.debug {
            println!("end\n");
        }

        self.pc = self.memory.len();
        return true;
    }

    fn unknown(&self, op: i64) -> bool {
        println!("{op}\tunknown operation");

        return false;
    }

    pub fn is_terminated(&self) -> bool {
        return self.pc >= self.memory.len();
    }

    pub fn step(&mut self) -> bool {
        if self.is_terminated() {
            return false;
        }

        let instruction = self.memory[self.pc];
        let op = instruction % 100;
        let m1 = (instruction / 100) % 10;
        let m2 = (instruction / 1000) % 10;
        let m3 = (instruction / 10000) % 10;
        let mode = vec![m1, m2, m3];

        let pc = self.pc;
        if self.debug {
            print!("{pc:04X}:\t");
        }

        return match op {
            1 => self.add(&mode),
            2 => self.mul(&mode),
            3 => self.inp(&mode),
            4 => self.out(&mode),
            5 => self.jit(&mode),
            6 => self.jif(&mode),
            7 => self.lt(&mode),
            8 => self.eq(&mode),
            9 => self.base(&mode),
            99 => self.end(&mode),
            n => self.unknown(n),
        };
    }

    pub fn run(&mut self) {
        // self.show();
        while self.step() {
            // self.show();
        }
    }

    #[allow(dead_code)]
    pub fn show(&self) {
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
    fn format_operand(&self, address: usize, mode: i64) -> String {
        // let operand = self.memory[address];
        let operand = self.load_direct(address);

        let format_position = |operand| {
            format!("{operand}")
        };

        let format_immediate = |operand| {
            format!("${operand}")
        };

        let format_relative = |operand| {
            format!("+{operand}")
        };

        match mode {
            0 => { format_position(operand) },
            1 => { format_immediate(operand) },
            2 => { format_relative(operand) },
            _ => { format!("Error") }
        }
    }

    #[allow(dead_code)]
    pub fn show_output(&self) {
        println!("{:?}", self.output);
    }

    #[allow(dead_code)]
    pub fn push_input(&mut self, data: i64) {
        self.input.push_back(data);
    }

    #[allow(dead_code)]
    pub fn output_len(&mut self) -> usize {
        return self.output.len();
    }

    #[allow(dead_code)]
    pub fn output(&mut self) -> Vec<i64> {
        return self.output.drain(..).collect();
        // return self.output.clone().into();
    }

    #[allow(dead_code)]
    pub fn pop_output(&mut self) -> Option<i64> {
        return self.output.pop_front();
        // return match self.output.pop_front() {
        //     Some(out) => out,
        //     None => 0
        // };
    }
}

pub fn read_program(filename: &str) -> Vec<i64> {
    fs::read_to_string(filename)
        .expect("Expected input file to exist")
        .replace("\n", "")
        .split(',')
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}
