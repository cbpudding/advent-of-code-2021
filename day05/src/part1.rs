use std::{
    cmp,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct Vent {
    start: (usize, usize),
    end: (usize, usize)
}

impl Vent {
    pub fn new(sx: usize, sy: usize, ex: usize, ey: usize) -> Self {
        Self {
            start: (sx, sy),
            end: (ex, ey)
        }
    }

    pub fn plot(&self, area: &mut Vec<Vec<usize>>) {
        if self.start.0 == self.end.0 {
            let sy = cmp::min(self.start.1, self.end.1);
            let ey = cmp::max(self.start.1, self.end.1);
            for y in sy..=ey {
                area[y][self.start.0] += 1;
            }
        } else if self.start.1 == self.end.1 {
            let sx = cmp::min(self.start.0, self.end.0);
            let ex = cmp::max(self.start.0, self.end.0);
            for x in sx..=ex {
                area[self.start.1][x] += 1;
            }
        } else {
            panic!("Not included in part one!");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = File::open(&args[1]).unwrap();
        let reader = BufReader::new(input);
        let data: Vec<usize> = reader
            .lines()
            .map(|l| -> Vec<usize> {
                l.unwrap()
                    .split(" ")
                    .filter(|&x| x != "->")
                    .map(|x| -> Vec<usize> {
                        x.split(",")
                            .map(|x| x.parse().unwrap())
                            .collect()
                    })
                    .flatten()
                    .collect()
            })
            .flatten()
            .collect();
        let mut height = 0;
        let mut vents = Vec::new();
        let mut width = 0;
        for i in 0..(data.len() / 4) {
            let sx = data[i * 4];
            let sy = data[(i * 4) + 1];
            let ex = data[(i * 4) + 2];
            let ey = data[(i * 4) + 3];
            if sx == ex || sy == ey {
                if sx > width || ex > width {
                    if sx > ex {
                        width = sx;
                    } else {
                        width = ex;
                    }
                }
                if sy > height || ey > height {
                    if sy > ey {
                        height = sy;
                    } else {
                        height = ey;
                    }
                }
                vents.push(Vent::new(sx, sy, ex, ey));
            }
        }
        let mut area = vec![vec![0; width + 1]; height + 1];
        for vent in vents {
            vent.plot(&mut area);
        }
        let mut dangerous = 0;
        for row in area {
            for cell in row {
                if cell > 1 {
                    dangerous += 1;
                }
            }
        }
        println!("Dangerous areas: {}", dangerous);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
