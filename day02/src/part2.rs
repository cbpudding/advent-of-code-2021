use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = File::open(&args[1]).unwrap();
        let reader = BufReader::new(input);
        let (mut x, mut y) = (0, 0); // Horizontal position and depth
        let mut aim = 0;
        for c in reader.lines() {
            let cmd = c.unwrap();
            let comp: Vec<&str> = cmd.split(" ").collect();
            let direction = comp[0];
            let amount = comp[1].parse::<usize>().unwrap();
            match direction {
                "forward" => {
                    x += amount;
                    y += aim * amount;
                },
                "down" => aim += amount,
                "up" => aim -= amount,
                _ => ()
            }
        }
        println!("Final coordinates are ({}, {}) resulting in a product of {}!", x, y, x * y);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
