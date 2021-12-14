use std::{fmt, fs, str::Split};
fn main() {
    let contents =
        fs::read_to_string("resources/11.txt").expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let mut sea = Sea::new(split);
    let mut cycle: u16 = 0;
    for _ in 1..101 {
        cycle += 1;
        sea.cycle();
        println!("After step {}:\n{}", cycle, sea);
    }
    println!("count is:\n{}", sea.count);
    while sea.flashing < 100 {
        cycle += 1;
        sea.cycle();
        println!("After step {}:\n{}", cycle, sea);
    }
}

#[derive(PartialEq, Debug)]
struct Octopus {
    counter: u8,
    light: bool,
}

impl fmt::Display for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.counter)
    }
}
impl Octopus {
    fn new(counter: u8) -> Octopus {
        Octopus {
            counter: counter,
            light: false,
        }
    }
    fn add(&mut self) -> bool {
        if self.light {
            return false;
        } else {
            self.counter += 1;
            if self.counter > 9 {
                self.counter = 0;
                self.light = true;
                return true;
            }
            return false;
        }
    }
    fn reset(&mut self) {
        self.light = false;
    }
}

struct Sea {
    grid: Vec<Vec<Octopus>>,
    update: Vec<Vec<usize>>,
    count: u16,
    flashing: u16,
}

impl fmt::Display for Sea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut p_vec: Vec<String> = Vec::new();
        for row in &self.grid {
            let mut r_vec: Vec<String> = Vec::new();
            for oct in row {
                r_vec.push(format!("{}", oct.counter));
            }
            p_vec.push(r_vec.join(""));
        }
        p_vec.push(format!("\n",));
        let print = p_vec.join("\n");
        write!(f, "{}", print)
    }
}

impl Sea {
    fn new(split: Split<&str>) -> Sea {
        let mut grid: Vec<Vec<Octopus>> = Vec::new();
        for line in split {
            let mut vec: Vec<Octopus> = Vec::new();
            let vals: Vec<u8> = line
                .chars()
                .into_iter()
                .map(|s| s.to_digit(10).unwrap() as u8)
                .collect();
            for i in vals {
                vec.push(Octopus::new(i))
            }
            grid.push(vec);
        }
        Sea {
            grid: grid,
            update: Vec::new(),
            count: 0,
            flashing: 0,
        }
    }
    fn cycle(&mut self) {
        self.flashing = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                self.grid[i][j].reset();
            }
        }
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j].add() {
                    self.update.push(vec![i, j])
                }
            }
        }
        while self.update.len() > 0 {
            // println!("now \n{}",self);
            let at = self.update.pop().unwrap();
            self.spread(at);
        }
        self.count += self.flashing;
    }
    fn pass(&mut self, at: Vec<usize>) {
        if at[0] < self.grid.len() && at[1] < self.grid[0].len() {
            let oc = &mut self.grid[at[0]][at[1]];
            if oc.add() {
                self.update.push(at);
            }
        }
    }
    fn spread(&mut self, at: Vec<usize>) {
        self.flashing += 1;
        let x = at[0];
        let y = at[1];
        self.pass(vec![x + 1, y]);
        self.pass(vec![x + 1, y + 1]);
        self.pass(vec![x, y + 1]);
        if x > 0 {
            self.pass(vec![x - 1, y + 1]);
            self.pass(vec![x - 1, y]);
        }
        if y > 0 {
            self.pass(vec![x, y - 1]);
            self.pass(vec![x + 1, y - 1]);
        }
        if x > 0 && y > 0 {
            self.pass(vec![x - 1, y - 1]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let contents =
            fs::read_to_string("resources/11t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut sea = Sea::new(split);
        println!("Before any steps:\n{}", sea);
        for i in 1..11 {
            sea.cycle();
            println!("After step {}:\n{}", i, sea);
        }
        assert_eq!(204, sea.count);
        for i in 1..91 {
            sea.cycle();
            println!("After step {}:\n{}", i, sea);
        }
        assert_eq!(1656, sea.count);
    }
    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("resources/11t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut sea = Sea::new(split);
        println!("Before any steps:\n{}", sea);
        for i in 1..196 {
            sea.cycle();
            println!("After step {}:\n{}", i, sea);
        }
        assert_eq!(100, sea.flashing);
    }
}
