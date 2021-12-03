use std::{fs, str::Split};

trait Task {
    fn count(&mut self, direction: &str, value: &str);

    fn run(&mut self, split: Split<&str>) {
        for i in split {
            let mut spi = i.split_whitespace();
            let dir = spi.next().unwrap();
            let val = spi.next().unwrap();
            self.count(&dir, &val);
        }
    }
}

struct FTask {
    horisontal: i32,
    depth: i32,
}

impl Task for FTask {
    fn count(&mut self, direction: &str, value: &str) {
        let int_val = value.parse::<i32>().unwrap();

        let s = String::from(direction);
        match s.as_str() {
            "forward" => {
                self.horisontal += int_val;
            }
            "up" => {
                self.depth -= int_val;
            }
            "down" => {
                self.depth += int_val;
            }
            _ => {
                println!("No");
            }
        }
    }
}
impl FTask {
    fn new() -> FTask {
        FTask {
            horisontal: 0,
            depth: 0,
        }
    }
}

struct STask {
    horisontal: i32,
    depth: i32,
    aim: i32,
}

impl STask {
    fn new() -> STask {
        STask {
            horisontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}
impl Task for STask {
    fn count(&mut self, direction: &str, value: &str) {
        let int_val = value.parse::<i32>().unwrap();

        let s = String::from(direction);
        match s.as_str() {
            "forward" => {
                self.horisontal += int_val;
                self.depth += int_val * self.aim;
            }
            "up" => self.aim -= int_val,
            "down" => self.aim += int_val,
            _ => {
                println!("No");
            }
        }
    }
}
fn main() {
    let contents =
        fs::read_to_string("resources/2.txt").expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let split2 = split.clone();

    let mut f = FTask::new();
    f.run(split);
    print!("count 1 :\n{}\n", f.horisontal * f.depth);
    let mut f2 = STask::new();
    f2.run(split2);

    print!("count 2 :\n{}\n", f2.horisontal * f2.depth);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let contents =
            fs::read_to_string("resources/2t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut f = FTask::new();
        f.run(split);
        assert_eq!(150, f.horisontal * f.depth);
    }

    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("resources/2t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut f = STask::new();
        f.run(split);
        assert_eq!(900, f.horisontal * f.depth);
    }
}
