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
        let mut score = 0;
        for line in reader.lines() {
            let mut stack = VecDeque::<BracketPair>::new();
            'line: for c in line.unwrap().chars() {
                if is_bracket(c) {
                    if is_closing(c) {
                        let pair = stack.pop_front().unwrap();
                        if !pair.matches(c) {
                            println!("Line {}: Expected '{}', but found '{}' instead.", lnum, pair.closing(), c);
                            match c {
                                ')' => score += 3,
                                ']' => score += 57,
                                '}' => score += 1197,
                                '>' => score += 25137,
                                _ => ()
                            }
                            break 'line;
                        }
                    } else {
                        if let Some(pair) = BracketPair::new(c) {
                            stack.push_front(pair);
                        }
                    }
                }
            }
            lnum += 1;
        }
        println!("Syntax error score: {}", score);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
