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
        let data: Vec<usize> = reader
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        let mut fishes = data.clone();
        for _ in 0..80 {
            let mut newfish = Vec::new();
            for fish in &mut fishes {
                if *fish == 0 {
                    *fish = 6;
                    newfish.push(8);
                } else {
                    *fish -= 1;
                }
            }
            fishes.append(&mut newfish);
        }
        println!("Lanternfish: {}", fishes.len());
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
