use std::collections::HashMap;
use std::iter::FromIterator;
use std::{fs, str::Split};
fn main() {
    let contents =
        fs::read_to_string("resources/14.txt").expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let mut poly = Poly::new(split);
    for _ in 0..10 {
        poly.iterate();
    }
    println!("countone is {:#?}", poly);
    println!("\nmax_min is {:#?}\n", poly.max_min());
    for _ in 0..30 {
        poly.iterate();
    }
    println!("countone is {:#?}", poly);
    println!("\nmax_min is {:#?}\n", poly.max_min());
}

#[derive(Debug)]
struct Poly {
    count: HashMap<String, u64>,
    countone: HashMap<char, u64>,
    model: HashMap<String, char>,
    sum: u64,
}

impl Poly {
    fn new(mut split: Split<&str>) -> Poly {
        let init: Vec<char> = String::from(split.next().unwrap()).chars().collect();
        let mut model: HashMap<String, char> = HashMap::new();
        split.next();
        for line in split {
            let mut pair: Vec<String> = line.split(" -> ").map(|s| s.parse().unwrap()).collect();
            model.insert(pair.remove(0), pair.pop().unwrap().chars().nth(0).unwrap());
        }
        let mut count: HashMap<String, u64> = HashMap::new();
        for i in 0..init.len() - 1 {
            let key: String = String::from_iter(vec![init[i], init[i + 1]]);
            *count.entry(key).or_default() += 1;
        }
        let mut countone: HashMap<char, u64> = HashMap::new();
        let mut sum: u64 = 0;
        for i in init {
            *countone.entry(i).or_default() += 1;
            sum += 1;
        }
        Poly {
            count,
            countone,
            model,
            sum,
        }
    }
    fn iterate(&mut self) {
        let mut count: HashMap<String, u64> = HashMap::new();
        for (key, value) in self.count.clone().into_iter() {
            let middle = self.model.get(&key).unwrap();
            let left = String::from_iter(vec![key.chars().nth(0).unwrap(), middle.clone()]);
            let right = String::from_iter(vec![middle.clone(), key.chars().nth(1).unwrap()]);
            *count.entry(left).or_default() += value;
            *count.entry(right).or_default() += value;
            *self.countone.entry(*middle).or_default() += value;
            self.sum += value;
        }
        self.count = count;
    }
    fn max_min(&mut self) -> u64 {
        let mut max: u64 = 0;
        let mut min = u64::MAX;
        for (_, value) in self.countone.clone().into_iter() {
            max = u64::max(max, value);
            min = u64::min(min, value);
        }

        return max - min;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let contents =
            fs::read_to_string("resources/14t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut poly = Poly::new(split);
        println!("countone is {:#?}", poly.model);
        for _ in 0..10 {
            poly.iterate();
        }
        assert_eq!(poly.sum, 3073);
        assert_eq!(poly.countone[&'B'], 1749);
        assert_eq!(poly.countone[&'C'], 298);
        assert_eq!(poly.countone[&'H'], 161);
        assert_eq!(poly.max_min(), 1588);
    }
    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("resources/14t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut poly = Poly::new(split);
        println!("countone is {:#?}", poly.model);
        for _ in 0..40 {
            poly.iterate();
        }
        assert_eq!(poly.countone[&'B'], 2192039569602);
        assert_eq!(poly.countone[&'H'], 3849876073);
        assert_eq!(poly.max_min(), 2188189693529);
    }
}
