use std::{
    cmp,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn paper_fold(paper: Vec<Vec<bool>>, upward: bool, fold: usize) -> Vec<Vec<bool>> {
    if upward {
        let mut folded = vec![vec![false; paper[0].len()]; fold];
        for y in 0..paper.len() {
            for x in 0..paper[0].len() {
                if y < fold {
                    folded[y][x] = paper[y][x];
                } else {
                    let axis = fold - (y - fold);
                    if axis < folded.len() {
                        folded[axis][x] |= paper[y][x];
                    }
                }
            }
        }
        folded
    } else {
        let mut folded = vec![vec![false; fold]; paper.len()];
        for y in 0..paper.len() {
            for x in 0..paper[0].len() {
                if x < fold {
                    folded[y][x] = paper[y][x];
                } else {
                    let axis = fold - (x - fold);
                    if axis < folded[0].len() {
                        folded[y][fold - (x - fold)] |= paper[y][x];
                    }
                }
            }
        }
        folded
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = File::open(&args[1]).unwrap();
        let reader = BufReader::new(input);
        let mut dots = Vec::new();
        let mut height = 0;
        let mut folds = Vec::new();
        let mut mode = 0; // 0 = dots, 1 = instructions
        let mut width = 0;
        for l in reader.lines() {
            let line = l.unwrap();
            if line == "" {
                mode += 1;
            } else if mode == 0 {
                let point: Vec<usize> = line.split(",").map(|n| n.parse().unwrap()).collect();
                width = cmp::max(width, point[0] + 1);
                height = cmp::max(height, point[1] + 1);
                dots.push(point);
            } else if mode == 1 {
                let fold: Vec<&str> = line.split(" ").last().unwrap().split("=").collect();
                let upward = fold[0] == "y";
                let coord: usize = fold[1].parse().unwrap();
                folds.push((upward, coord));
            } else {
                panic!("Invalid parsing mode: {}", mode);
            }
        }
        let mut paper = vec![vec![false; width]; height];
        for dot in dots {
            paper[dot[1]][dot[0]] = true;
        }
        for fold in folds {
            paper = paper_fold(paper, fold.0, fold.1);
        }
        for y in 0..paper.len() {
            for x in 0..paper[0].len() {
                if paper[y][x] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
