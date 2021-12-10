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
        let data: Vec<Vec<u8>> = reader
            .lines()
            .map(|l| l.unwrap().chars().map(|n| n as u8 - 48).collect())
            .collect();
        let height = data.len();
        let width = data[0].len();
        let mut risky = Vec::new();
        for y in 0..height {
            for x in 0..width {
                if x > 0 {
                    if data[y][x] >= data[y][x - 1] {
                        continue;
                    }
                }
                if x < width - 1 {
                    if data[y][x] >= data[y][x + 1] {
                        continue;
                    }
                }
                if y > 0 {
                    if data[y][x] >= data[y - 1][x] {
                        continue;
                    }
                }
                if y < height - 1 {
                    if data[y][x] >= data[y + 1][x] {
                        continue;
                    }
                }
                risky.push((x, y));
            }
        }
        let total: usize = risky.iter().map(|&(x, y)| data[y][x] as usize + 1).sum();
        println!("Total risk: {}", total);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
