use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn flash(grid: &mut Vec<Vec<u8>>, flashed: &mut Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut flashes = 0;
    if grid[y][x] > 9 && !flashed[y][x] {
        flashes += 1;
        flashed[y][x] = true;
        grid[y][x] = 0;
        if x > 0 {
            if !flashed[y][x - 1] {
                grid[y][x - 1] += 1;
                flashes += flash(grid, flashed, x - 1, y);
            }
            if y > 0 && !flashed[y - 1][x - 1] {
                grid[y - 1][x - 1] += 1;
                flashes += flash(grid, flashed, x - 1, y - 1);
            }
        }
        if x < 9 {
            if !flashed[y][x + 1] {
                grid[y][x + 1] += 1;
                flashes += flash(grid, flashed, x + 1, y);
            }
            if y < 9 && !flashed[y + 1][x + 1] {
                grid[y + 1][x + 1] += 1;
                flashes += flash(grid, flashed, x + 1, y + 1);
            }
        }
        if y > 0 {
            if !flashed[y - 1][x] {
                grid[y - 1][x] += 1;
                flashes += flash(grid, flashed, x, y - 1);
            }
            if x < 9 && !flashed[y - 1][x + 1] {
                grid[y - 1][x + 1] += 1;
                flashes += flash(grid, flashed, x + 1, y - 1);
            }
        }
        if y < 9 {
            if !flashed[y + 1][x] {
                grid[y + 1][x] += 1;
                flashes += flash(grid, flashed, x, y + 1);
            }
            if x > 0 && !flashed[y + 1][x - 1] {
                grid[y + 1][x - 1] += 1;
                flashes += flash(grid, flashed, x - 1, y + 1);
            }
        }
    }
    flashes
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = File::open(&args[1]).unwrap();
        let reader = BufReader::new(input);
        let mut grid: Vec<Vec<u8>> = reader
            .lines()
            .map(|l| l.unwrap().chars().map(|c| c as u8 - 48).collect())
            .collect();
        let mut flashes = 0;
        for _step in 0..100 {
            let mut flashed = vec![vec![false; 10]; 10];
            for y in 0..10 {
                for x in 0..10 {
                    grid[y][x] += 1;
                }
            }
            for y in 0..10 {
                for x in 0..10 {
                    flashes += flash(&mut grid, &mut flashed, x, y);
                }
            }
        }
        println!("Flashes: {}", flashes);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
