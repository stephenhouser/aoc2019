use std::env;
use std::fs;
use std::process;
use std::time::Instant; // 0.8.2

fn read_data(filename: &str) -> Vec<u32> {
    fs::read_to_string(filename)
        .expect("Expected input file to exist")
        .replace("\n", "")
        .chars()
        // .map(|ch| u8::try_from(ch).unwrap())
        .map(|ch| ch.to_digit(10).expect("Not a number"))
        .collect()
}

fn digit_counts(raw: &[u32]) -> Vec<usize> {
    let mut counts = vec![0; 10];
    
    for &digit in raw {
        counts[digit as usize] += 1;
    }

    return counts;
}

fn part1(raw: &Vec<u32>) -> usize {
    let (width, height) = match raw.len() {
        12 => (3, 2),   // test.txt
        16 => (2, 2),   // test-1.txt
         _ => (25, 6),  // input.txt
    };

    let result = raw
        // break into layers based on layer size
        .chunks(width * height)
        // get vector of the number of each digit
        .map(|layer| digit_counts(layer))
        // filter for the layer with the least 0s
        .min_by_key(|x| x[0])
        // transform to the product of 1s and 2s
        .map(|l| l[1] * l[2])
        // unwrap Some() from min_by_key if collection was empty
        .unwrap();

    return result;
}

fn apply_layer(top: &Vec<u32>, bot: &[u32]) -> Vec<u32> {
    return top.clone()
        .into_iter()
        .enumerate()
        .map(|(i, s)| if s == 2 { bot[i] } else { s } )
        .collect();
}

fn show_picture(picture: &[u32], width: usize) {
    for row in picture.chunks(width) {
        for digit in row {
            match digit {
                0 => print!(" "),
                1 => print!("*"),
                _ => print!("_")
            }
        }
        println!();
    }
}

fn part2(raw: &Vec<u32>) -> usize {
    let (width, height) = match raw.len() {
        12 => (3, 2),   // test.txt
        16 => (2, 2),   // test-1.txt
         _ => (25, 6),  // input.txt
    };

    let empty: Vec<u32> = vec![2; width * height];

    let picture = raw
        // break into layers vector of vec<32>
        .chunks(width * height)
        // fold into final picture by applying layers
        .fold(empty, |pic, layer| apply_layer(&pic, layer));

    // BCYEF for input
    show_picture(&picture, width);
    return picture.len();
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
