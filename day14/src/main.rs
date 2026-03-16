use std::env;
use std::process;
use std::fs;
use std::time::Instant; // 0.8.2
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Component {
    n: i64,
    element: String,
}

#[derive(Clone, Debug)]
struct Production {
    output: Componenet,
    elements: Vec<Component>,
}

fn read_component(text: &str) -> Component {
    let parts: Vec<&str> = text.split_whitespace().collect();

    Component { 
        n: parts[0].parse().unwrap(),
        element: parts[1].to_string() }
}

fn read_production(line: &str) -> Production {
    let parts: Vec<&str> = line.split("=>").collect();

    let elements: Vec<Component> = parts[0].split(',')
            .map(|text| read_component(text))
            .collect();

    let output: read_component(parts[1]);

    return Production {
        output: output,
        elements: elements,
    };
}

fn read_data(filename: &str) -> HashMap<String, Production> {
    // create vector of points and their value
    let rules = fs::read_to_string(filename)
        .expect("Expected input file to exist")
        .split('\n')
        .filter(|text| text.len() >= 1)
        .map(|text| read_production(text))
        .map(|rule| (rule.output.element, rule))
        .collect();

    return rules;
}


fn find_rule<'a>(rules: &'a Vec<Production>, element: &String) -> &'a Production {
    for prod in rules {
        if prod.out[0].element == *element {
            return &prod;
        }
    }

    return &rules[0];
}


// how many `base` to produce a `produce`
fn satisfy(rules: &Vec<Production>) -> i64 {
    let mut current: HashMap<String, i64> = HashMap::new();
    let mut next: HashMap<String, i64> = HashMap::new();
    let mut _extra: HashMap<String, i64> = HashMap::new();

    println!("Start...");
    current.insert("FUEL".to_string(), 1);

    let mut changes = 1;
    while changes != 0 {
        changes = 0;

        for (e, n) in current {

            if e == "ORE" {
                *next.entry(e).or_insert(0) += n;
                continue;
            }

            print!("Making {}x{} needs ", n, e);

            changes += 1;
            let rule = find_rule(rules, &e);
            let produced = rule.out[0].n; // => 10 A

            for c in &rule.inp {            // 10 ORE
                let mut needed_n = c.n; // produce c.n n-times
                while needed_n > 0 {
                    print!("{}x{}={}, ", c.n, c.element, needed_n);
                    // enter that we needed to produce c.n of these
                    *next.entry(c.element.clone()).or_insert(0) += n;
                    needed_n -= produced;
                }
                print!("END{}={} ", c.element, needed_n);
                if needed_n != 0 {
                    print!("extra {}={} ", c.element, needed_n);
                }
            }

            println!("");
    for (e, n) in next.iter() {
        println!("{}:{}", n, e);
    }
            println!("");
        }

        current = next;
        next = HashMap::new();
        println!("");
    }

    for (e, n) in current.iter() {
        println!("{}:{}", n, e);
    }
    // current => vec of current needed elements, 1 FUEL
    // produced => vec of produced elementsA
    //
    // stock: HashMap<element, n>
    // for e, n in stock
    //   if e != ORE
    //      need = find_out(e)
    //      for e1, n1 in need
    //          stock[e1] += n * n1
    
    return 0;
}

fn part1(rules: &Vec<Production>) -> usize {
    for prod in rules {
        for i in &prod.inp {
            print!("{} {}, ", i.n, i.element);
        }

        print!("-> ");

        for o in &prod.out {
            print!("{} {}", o.n, o.element);
        }
        println!("");
    }

    satisfy(rules);

    return rules.len();
}

fn part2(rules: &Vec<Production>) -> usize {
    return rules.len();
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
