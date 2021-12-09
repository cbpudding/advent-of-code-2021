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
            .map(|l| {
                l.unwrap()
                    .split(" ")
                    .filter(|&c| c != "|")
                    .map(|c| c.chars().map(|d| 1 << (d as u8 - 97)).sum())
                    .collect()
            })
            .collect();
        let mut outputs = Vec::new();
        for line in data {
            // We know these will always exist and be useful so we'll give them dedicated names
            let four = *line.iter().filter(|&d| d.count_ones() == 4).next().unwrap();
            let seven = *line.iter().filter(|&d| d.count_ones() == 3).next().unwrap();
            // We'll put all the digits we know about here
            let mut digits = [None; 10];
            // One, four, seven, and eight.
            digits[1] = line
                .iter()
                .filter(|&d| d.count_ones() == 2)
                .map(|&d| d)
                .next();
            digits[4] = Some(four);
            digits[7] = Some(seven);
            digits[8] = line
                .iter()
                .filter(|&d| d.count_ones() == 7)
                .map(|&d| d)
                .next();
            // Collect all the five segment digits(Two, three, and five)
            let fives: Vec<u8> = line
                .iter()
                .filter(|&d| d.count_ones() == 5)
                .map(|&d| d)
                .collect();
            digits[2] = fives
                .iter()
                .filter(|&d| (d & four).count_ones() == 2)
                .map(|&d| d)
                .next();
            digits[3] = fives
                .iter()
                .filter(|&d| d & seven == seven)
                .map(|&d| d)
                .next();
            digits[5] = fives
                .iter()
                .filter(|&d| (d & four).count_ones() == 3)
                .filter(|&d| d & seven != seven)
                .map(|&d| d)
                .next();
            // Collect all the six segment digits(Zero, six, and nine)
            let sixes: Vec<u8> = line
                .iter()
                .filter(|&d| d.count_ones() == 6)
                .map(|&d| d)
                .collect();
            digits[0] = sixes
                .iter()
                .filter(|&d| d & seven == seven)
                .filter(|&d| d & four != four)
                .map(|&d| d)
                .next();
            digits[6] = sixes
                .iter()
                .filter(|&d| d & seven != seven)
                .filter(|&d| d & four != four)
                .map(|&d| d)
                .next();
            digits[9] = sixes
                .iter()
                .filter(|&d| d & four == four)
                .map(|&d| d)
                .next();
            // Finally, decode the output values
            let chunk = &line[10..];
            let output = chunk.iter().fold(0, |mut a, d| {
                a = (a * 10) + digits.iter().position(|&x| x == Some(*d)).unwrap();
                a
            });
            outputs.push(output);
        }
        println!("Output sum: {}", outputs.iter().sum::<usize>());
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
