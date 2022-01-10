use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = File::open(&args[1]).unwrap();
        let reader = BufReader::new(input);
        let mut lines = reader.lines();
        let mut elements: HashMap<char, usize> = HashMap::new();
        let mut test_tube: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
        let mut combinations = HashMap::new();
        lines.next(); // Discard blank line
        for c in lines {
            let combo = c.unwrap();
            let parts: Vec<&str> = combo.split(" -> ").collect();
            let parents: Vec<char> = parts[0].chars().collect();
            let child = parts[1].chars().next().unwrap();
            combinations.insert((parents[0], parents[1]), child);
        }
        for e in &test_tube {
            let value = if elements.contains_key(&e) {
                *elements.get(&e).unwrap()
            } else {
                0
            };
            elements.insert(*e, value + 1);
        }
        for _step in 0..40 {
            let mut reaction = Vec::new();
            for i in 1..test_tube.len() {
                let pair = (test_tube[i - 1], test_tube[i]);
                if combinations.contains_key(&pair) {
                    let result = *combinations.get(&pair).unwrap();
                    let old_value = if elements.contains_key(&result) {
                        *elements.get(&result).unwrap()
                    } else {
                        0
                    };
                    elements.insert(result, old_value + 1);
                    reaction.push(result);
                } else {
                    panic!("Unknown combination: {:?}", pair);
                }
            }
            for i in 0..reaction.len() {
                test_tube.insert((i * 2) + 1, reaction[i]);
            }
        }
        let mut least = (0 as char, usize::MAX);
        let mut most = (0 as char, 0);
        for e in elements.keys() {
            let count = *elements.get(&e).unwrap();
            if count > most.1 {
                most.0 = *e;
                most.1 = count;
            }
            if count < least.1 {
                least.0 = *e;
                least.1 = count;
            }
        }
        println!("The least common element is {}", least.0);
        println!("The most common element is {}", most.0);
        println!("The difference is {}", most.1 - least.1);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
