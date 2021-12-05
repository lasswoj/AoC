use std::collections::VecDeque;
use std::iter::FromIterator;
use std::{fs, str::Split};

struct PUsage {
    gamma: u32,
    epsilon: u32,
}

impl PUsage {
    fn new() -> PUsage {
        PUsage {
            gamma: 0,
            epsilon: 0,
        }
    }
    fn shift(&mut self) {
        self.gamma <<= 1;
        self.epsilon <<= 1;
    }
    fn count(&mut self, split: Split<&str>) {
        let vect: Vec<&str> = split.collect();
        for i in 0..vect[0].len() {
            let mut loc_g = 0;
            let mut loc_e = 0;
            for j in 0..vect.len() {
                if vect[j].chars().nth(i).unwrap() == '1' {
                    loc_g += 1;
                } else {
                    loc_e += 1;
                }
            }
            self.shift();
            if loc_g > loc_e {
                self.gamma += 1;
            } else {
                self.epsilon += 1
            }
        }
    }
}

struct OUsage {
}

impl OUsage {
    fn new() -> OUsage {
        OUsage {}
    }
    fn decode(code: &str) -> u16 {
        println!("code is {}", code);
        let mut vec = VecDeque::from_iter(code.chars());
        let mut decoded: u16 = 0;
        while vec.len() > 0 {
            decoded = decoded << 1;
            let char = vec.pop_front().unwrap() as u8 - '0' as u8;
            decoded += char as u16
        }
        decoded
    }
    fn count_ox(&mut self, split: Split<&str>) -> u16 {
        let mut vect: Vec<&str> = split.collect();
        for i in 0..vect[0].len() {
            let mut vec0: Vec<&str> = Vec::with_capacity(vect.len());
            let mut vec1: Vec<&str> = Vec::with_capacity(vect.len());
            while vect.len() > 0 {
                let val = vect.pop().unwrap();
                if val.chars().nth(i).unwrap() == '1' {
                    vec1.push(val);
                } else {
                    vec0.push(val);
                }
            }
            if vec1.len() >= vec0.len() {
                vect = vec1;
            } else {
                vect = vec0;
            }
        }
        let decoded = OUsage::decode(vect.pop().unwrap());
        decoded
    }
    fn count_co(&mut self, split: Split<&str>) -> u16 {
        let mut vect: Vec<&str> = split.collect();
        for i in 0..vect[0].len() {
            if vect.len() == 1 {
                break;
            };
            let mut vec0: Vec<&str> = Vec::with_capacity(vect.len());
            let mut vec1: Vec<&str> = Vec::with_capacity(vect.len());
            while vect.len() > 0 {
                let val = vect.pop().unwrap();
                if val.chars().nth(i).unwrap() == '1' {
                    vec1.push(val);
                } else {
                    vec0.push(val);
                }
            }
            if vec1.len() >= vec0.len() {
                vect = vec0;
            } else {
                vect = vec1;
            }
        }
        let decoded = OUsage::decode(vect.pop().unwrap());
        decoded
    }
}

fn main() {
    let contents =
        fs::read_to_string("resources/3.txt").expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let split2 = split.clone();
    let split3 = split.clone();
    let mut pusage = PUsage::new();
    pusage.count(split);
    print!(
        "count power consumption :\n{}\n",
        pusage.gamma * pusage.epsilon
    );
    let mut ous = OUsage::new();
    let ox = ous.count_ox(split2);
    let co = ous.count_co(split3);
    print!("count 2 :\n{}\n{}\n{}", ox, co, ox as u32 * co as u32);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let contents =
            fs::read_to_string("resources/3t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut pusage = PUsage::new();
        pusage.count(split);
        assert_eq!(22, pusage.gamma);
        assert_eq!(9, pusage.epsilon);
    }
    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("resources/3t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let split2 = split.clone();
        let mut ous = OUsage::new();
        let ox = ous.count_ox(split);
        let co = ous.count_co(split2);
        assert_eq!(23, ox);
        assert_eq!(10, co);
    }
}
