use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn oxygen_generator(diag: &Vec<usize>, width: usize) -> usize {
    let mut bit = 0;
    let mut data = diag.clone();
    while data.len() > 1 {
        let mut count = 0;
        let mask = 1 << (width - bit) - 1;
        for x in &data {
            if x & mask != 0 {
                count += 1;
            }
        }
        let iter = data.iter();
        if count >= data.len() - count {
            data = iter.filter(|x| **x & mask != 0).map(|x| *x).collect();
        } else {
            data = iter.filter(|x| **x & mask == 0).map(|x| *x).collect();
        }
        bit += 1;
    }
    data[0]
}

fn co2_scrubber(diag: &Vec<usize>, width: usize) -> usize {
    let mut bit = 0;
    let mut data = diag.clone();
    while data.len() > 1 {
        let mut count = 0;
        let mask = 1 << (width - bit) - 1;
        for x in &data {
            if x & mask != 0 {
                count += 1;
            }
        }
        let iter = data.iter();
        if count < data.len() - count {
            data = iter.filter(|x| **x & mask != 0).map(|x| *x).collect();
        } else {
            data = iter.filter(|x| **x & mask == 0).map(|x| *x).collect();
        }
        bit += 1;
    }
    data[0]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = File::open(&args[1]).unwrap();
        let reader = BufReader::new(input);
        let mut total = 0;
        let mut count = Vec::new();
        let mut diag = Vec::new();
        for raw in reader.lines() {
            let line = raw.unwrap();
            let data = usize::from_str_radix(&line, 2).unwrap();
            diag.push(data);
            if count.len() == 0 {
                count = vec![0; line.len()];
            }
            for bit in 0..count.len() {
                let mask = 1 << bit;
                if data & mask != 0 {
                    count[bit] += 1;
                }
            }
            total += 1;
        }
        let mut gamma = 0;
        for bit in 0..count.len() {
            if count[bit] > total - count[bit] {
                gamma |= 1 << bit;
            }
        }
        let epsilon = !gamma & (1 << count.len()) - 1;
        println!("Gamma: {}", gamma);
        println!("Epsilon: {}", epsilon);
        println!("Power Consumption: {}", gamma * epsilon);
        let oxygen_rating = oxygen_generator(&diag, count.len());
        let scrubber_rating = co2_scrubber(&diag, count.len());
        let support_rating = oxygen_rating * scrubber_rating;
        println!("Oxygen Generator Rating: {}", oxygen_rating);
        println!("CO2 Scrubber Rating: {}", scrubber_rating);
        println!("Life Support Rating: {}", support_rating);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
