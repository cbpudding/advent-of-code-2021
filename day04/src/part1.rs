use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct BingoCard {
    numbers: Vec<usize>,
    marked: Vec<bool>
}

impl BingoCard {
    fn bingo(marked: Vec<bool>) -> bool {
        marked.iter().fold(true, |mut r, &x| {r &= x; r})
    }

    pub fn check(&mut self, number: usize) -> bool {
        if self.numbers.contains(&number) {
            let index = self.numbers.iter().position(|&x| x == number).unwrap();
            self.marked[index] = true;
            for r in 0..5 {
                let start = r * 5;
                let end = start + 5;
                if Self::bingo(self.marked[start..end].to_vec()) {
                    return true;
                }
            }
            for c in 0..5 {
                let mut col = Vec::new();
                for o in 0..5 {
                    col.push(self.marked[(o * 5) + c]);
                }
                if Self::bingo(col) {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }

    pub fn new(numbers: &[usize]) -> Self {
        Self {
            numbers: Vec::from(numbers),
            marked: vec![false; 25]
        }
    }

    pub fn score(&self) -> usize {
        let mut score = 0;
        for i in 0..25 {
            if !self.marked[i] {
                score += self.numbers[i];
            }
        }
        score
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = File::open(&args[1]).unwrap();
        let reader = BufReader::new(input);
        let mut lines = reader.lines();
        let numbers: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        let carddata: Vec<usize> = lines
            .map(|l| -> Vec<usize> {
                l.unwrap()
                    .split(" ")
                    .map(|x| x.parse())
                    .filter(|x| x.is_ok())
                    .map(|x| x.unwrap())
                    .collect()
            })
            .flatten()
            .collect();
        let mut cards = Vec::new();
        for i in 0..(carddata.len() / 25) {
            let start = i * 25;
            let end = start + 25;
            cards.push(BingoCard::new(&carddata[start..end]));
        }
        'bingo: for num in numbers {
            for card in &mut cards {
                if card.check(num) {
                    let score = card.score();
                    println!("Score: {}", score);
                    println!("Last Number Drawn: {}", num);
                    println!("Final Score: {}", score * num);
                    break 'bingo;
                }
            }
        }
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
