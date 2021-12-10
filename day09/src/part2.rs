use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn find_basin(data: &Vec<Vec<u8>>, basin: &mut Vec<(usize, usize)>, (x, y): (usize, usize)) {
    let height = data.len();
    let width = data[0].len();
    basin.push((x, y));
    if x > 0 {
        if data[y][x - 1] != 9 {
            if !basin.contains(&(x - 1, y)) {
                find_basin(data, basin, (x - 1, y));
            }
        }
    }
    if x < width - 1 {
        if data[y][x + 1] != 9 {
            if !basin.contains(&(x + 1, y)) {
                find_basin(data, basin, (x + 1, y));
            }
        }
    }
    if y > 0 {
        if data[y - 1][x] != 9 {
            if !basin.contains(&(x, y - 1)) {
                find_basin(data, basin, (x, y - 1));
            }
        }
    }
    if y < height - 1 {
        if data[y + 1][x] != 9 {
            if !basin.contains(&(x, y + 1)) {
                find_basin(data, basin, (x, y + 1));
            }
        }
    }
}

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
        let mut basins = Vec::new();
        for point in risky {
            let mut basin = Vec::new();
            find_basin(&data, &mut basin, point);
            basins.push(basin);
        }
        basins.sort_by(|a, b| b.len().cmp(&a.len()));
        let answer: usize = basins.iter().take(3).map(|x| x.len()).product();
        println!("Answer: {}", answer);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
