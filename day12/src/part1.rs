use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Cave {
    connected: Vec<String>,
    revisitable: bool
}

impl Cave {
    pub fn add(&mut self, key: String) {
        self.connected.push(key);
    }

    pub fn connected(&self) -> &Vec<String> {
        &self.connected
    }

    pub fn new(name: &String) -> Self {
        let upper = name.to_ascii_uppercase();
        Self {
            connected: Vec::new(),
            revisitable: *name == upper
        }
    }

    pub fn revisitable(&self) -> bool {
        self.revisitable
    }
}

fn travel(caves: &HashMap<String, Cave>, cave: &Cave, hstry: Vec<String>) -> usize {
    let connected = cave.connected();
    let mut paths = 0;
    for c in connected {
        let mut history = hstry.clone();
        if c == "end" {
            history.push(c.clone());
            paths += 1;
        } else {
            let subcave = caves.get(c).unwrap();
            if !history.contains(c) || subcave.revisitable() {
                history.push(c.clone());
                paths += travel(caves, subcave, history.clone());
            }
        }
    }
    paths
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = File::open(&args[1]).unwrap();
        let reader = BufReader::new(input);
        let mut caves = HashMap::new();
        for line in reader.lines() {
            let path: Vec<String> = line.unwrap().split("-").map(|s| s.to_string()).collect();
            if !caves.contains_key(&path[0]) {
                caves.insert(path[0].clone(), Cave::new(&path[0]));
            }
            if !caves.contains_key(&path[1]) {
                caves.insert(path[1].clone(), Cave::new(&path[1]));
            }
            if let Some(cave) = caves.get_mut(&path[0]) {
                cave.add(path[1].clone());
            }
            if let Some(cave) = caves.get_mut(&path[1]) {
                cave.add(path[0].clone());
            }
        }
        let start = caves.get("start").unwrap();
        let paths = travel(&caves, start, vec![String::from("start")]);
        println!("Number of paths: {}", paths);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
