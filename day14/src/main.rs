use std::env;
use std::process;
use std::fs;
use std::time::Instant; // 0.8.2
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Reaction {
    input: Vec<(String, u64)>,
    output: String,
    output_count: u64,
}

fn read_component(text: &str) -> (String, u64) {
    let parts: Vec<&str> = text.split_whitespace().collect();

    (parts[1].to_string(), parts[0].parse().unwrap())
}

fn read_reaction(line: &str) -> Reaction {
    let parts: Vec<&str> = line.split("=>").collect();

    let input = parts[0].split(',')
            .map(|text| read_component(text))
            .collect();

    let output = read_component(parts[1]);

    return Reaction {
        input: input,
        output: output.0,
        output_count: output.1
    };
}

fn read_data(filename: &str) -> HashMap<String, Reaction> {
    // create vector of points and their value
    let rules = fs::read_to_string(filename)
        .expect("Expected input file to exist")
        .split('\n')
        .filter(|text| text.len() >= 1)
        .map(|text| read_reaction(text))
        .map(|rule| (rule.output.clone(), rule))
        .collect();

    return rules;
}

// how many `base` to produce a `produce`
fn satisfy(rules: &HashMap<String, Reaction>, element: &String, count: u64, base: &String) -> u64 {
    let mut surplus: HashMap<String, u64> = HashMap::new();
    let mut queue: VecDeque<(String, u64)> = VecDeque::new();
    queue.push_back((element.clone(), count));

    let mut base_count: u64 = 0;
    while let Some((element, amount)) = queue.pop_front() {
        //println!("Produce {} {}", amount, element);
        let available = surplus.get(&element).copied().unwrap_or(0);

        if amount <= available {
            // we have enough surplus to satisfy this
            surplus.insert(element, available - amount);
            continue;
        }

        let deficit = amount - available;
        let reaction = rules.get(&element).expect("Element not found");

        // Produce enough to satisfy deficit
        // ceiling division
        let batches = (deficit + reaction.output_count - 1) / reaction.output_count;

        let produced = batches * reaction.output_count;
        let new_surplus = produced - deficit;
        surplus.insert(element, new_surplus);

        // add new items to queue
        for (input_element, input_count) in &reaction.input {
            let total_needed = input_count * batches;
            if input_element == base {
                // handle ore, don't add to queue
                base_count += total_needed;
            } else {
                queue.push_back((input_element.clone(), total_needed));
            }
        }
    }

    return base_count;
}

fn part1(rules: &HashMap<String, Reaction>) -> u64 {
    satisfy(rules, &"FUEL".to_string(), 1, &"ORE".to_string())
}

fn part2(rules: &HashMap<String, Reaction>) -> u64 {
    let trillion: u64 = 1000000000000;
    let ore_per_fuel = satisfy(rules, &"FUEL".to_string(), 1, &"ORE".to_string());

    // track the highest amount of FUEL created (less than 1 trillion)
    let mut best: u64 = 0;

    // binary search for amount of FUEL created by 1000000000000 ORE
    let mut lo: u64 = trillion / ore_per_fuel; 
    let mut hi: u64 = trillion / (ore_per_fuel / 2);

    while lo <= hi {
        let fuel = (lo + hi) / 2;
        let ore_used = satisfy(rules, &"FUEL".to_string(), fuel, &"ORE".to_string());
        //println!("lo={}, fuel={}, hi={}, ore_used={}", lo, fuel, hi, ore_used);

        if ore_used <= trillion {
            lo = fuel + 1;
            best = fuel;
        } else {
            hi = fuel - 1;
        }
    }

    return best;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("ERROR: No input file specified");
        process::exit(1);
    }

    let start = Instant::now();

    let data = read_data(&args[1]);
    println!("          parse ({:9.9} ns)", Instant::now().duration_since(start).as_nanos());

    let p1_start = Instant::now();
    let p1 = part1(&data);
    println!("{p1:>15} ({:9.9} ns)", Instant::now().duration_since(p1_start).as_nanos());

    let p2_start = Instant::now();
    let p2 = part2(&data);

    println!("{p2:>15} ({:9.9} ns)", Instant::now().duration_since(p2_start).as_nanos());
    println!("          total ({:9.9} ns)", Instant::now().duration_since(start).as_nanos());
}
