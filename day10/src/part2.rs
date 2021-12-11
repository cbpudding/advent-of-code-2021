use std::{
    collections::VecDeque,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy)]
enum BracketPair {
    Normal, // ()
    Square, // []
    Curly, // {}
    Angle // <>
}

impl BracketPair {
    pub fn closing(&self) -> char {
        match *self {
            Self::Normal => ')',
            Self::Square => ']',
            Self::Curly => '}',
            Self::Angle => '>'
        }
    }

    pub fn matches(&self, c: char) -> bool {
        match (*self, c) {
            (Self::Normal, '(' | ')') => true,
            (Self::Square, '[' | ']') => true,
            (Self::Curly, '{' | '}') => true,
            (Self::Angle, '<' | '>') => true,
            _ => false
        }
    }

    pub fn new(c: char) -> Option<Self> {
        match c {
            '(' | ')' => Some(Self::Normal),
            '[' | ']' => Some(Self::Square),
            '{' | '}' => Some(Self::Curly),
            '<' | '>' => Some(Self::Angle),
            _ => None
        }
    }
}

fn is_bracket(c: char) -> bool {
    match c {
        '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' => true,
        _ => false
    }
}

fn is_closing(c: char) -> bool {
    match c {
        ')' | ']' | '}' | '>' => true,
        _ => false
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = File::open(&args[1]).unwrap();
        let reader = BufReader::new(input);
        let mut lnum = 1;
        let mut scores = Vec::new();
        'line: for line in reader.lines() {
            let mut stack = VecDeque::<BracketPair>::new();
            for c in line.unwrap().chars() {
                if is_bracket(c) {
                    if is_closing(c) {
                        let pair = stack.pop_front().unwrap();
                        if !pair.matches(c) {
                            println!("Line {}: Expected '{}', but found '{}' instead.", lnum, pair.closing(), c);
                            continue 'line;
                        }
                    } else {
                        if let Some(pair) = BracketPair::new(c) {
                            stack.push_front(pair);
                        }
                    }
                }
            }
            let mut ending = String::new();
            let mut score: usize = 0;
            while let Some(pair) = stack.pop_front() {
                score *= 5;
                ending.push(pair.closing());
                score += match pair {
                    BracketPair::Normal => 1,
                    BracketPair::Square => 2,
                    BracketPair::Curly => 3,
                    BracketPair::Angle => 4
                };
            }
            if score > 0 {
                println!("Line {}: Complete by adding '{}'.", lnum, ending);
                scores.push(score);
            }
            lnum += 1;
        }
        scores.sort();
        println!("Autocomplete score: {}", scores[scores.len() / 2]);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
