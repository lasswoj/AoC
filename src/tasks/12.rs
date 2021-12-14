use std::collections::{HashMap, HashSet};
use std::{fs, str::Split};
fn main() {
    let contents =
        fs::read_to_string("resources/12.txt").expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let split2 = split.clone();
    let mut nodes = Nodes::new();
    nodes.load_conns(split);
    nodes.count("start", &Vec::new());
    println!("count is :\n{:#?}", nodes.counter);

    let mut nodes2 = Nodes::new();
    nodes2.load_conns(split2);
    nodes2.count_twice("start", &Vec::new(), &false, &Vec::new());
    println!("count is :\n{:#?}", nodes2.pathset.len());
    // }
}

#[derive(PartialEq, Debug)]
struct Nodes<'a> {
    map: HashMap<&'a str, Vec<&'a str>>,
    counter: u16,
    pathset: HashSet<Vec<String>>,
}

impl<'a> Nodes<'a> {
    fn create_connections(&mut self, from: &'a str, to: &'a str) {
        if from == "end" || to == "start" {
            return;
        }
        self.map.entry(from).or_default().push(to);
    }
    fn load_conns(&mut self, split: Split<'a, &str>) {
        let paths: Vec<Vec<&str>> = split.map(|line| line.split("-").collect()).collect();
        for pair in paths {
            self.create_connections(&pair[0], &pair[1]);
            self.create_connections(&pair[1], &pair[0]);
        }
    }
    fn count(&mut self, from: &str, avail: &Vec<&str>) {
        if from == "end" {
            println!("path is {:?} and now {}", avail, from);
            self.counter += 1;
            return;
        }
        let mut next_vect = avail.clone();
        if from == from.to_lowercase() {
            next_vect.push(from);
        }
        let togo = self.map[from].clone();
        for go in togo {
            if !next_vect.contains(&go) {
                self.count(go, &next_vect)
            }
        }
    }
    fn count_twice(&mut self, from: &'a str, avail: &Vec<&str>, taken: &bool, path: &Vec<String>) {
        if from == "end" {
            println!("path is {:?} ", path);
            self.pathset.insert(path.to_vec().clone());
            return;
        }
        let mut next_vect = avail.clone();
        let mut next_path = path.clone();
        next_path.push(from.to_string());
        if from == from.to_lowercase() {
            if !taken {
                let togo = self.map[from].clone();
                for go in togo {
                    if !next_vect.contains(&go) {
                        self.count_twice(go, &next_vect, &true, &next_path);
                    }
                }
            }
            next_vect.push(from);
        }
        let togo = self.map[from].clone();
        for go in togo {
            if !next_vect.contains(&go) {
                self.count_twice(go, &next_vect, &taken, &next_path)
            }
        }
    }

    fn new() -> Nodes<'a> {
        Nodes {
            map: HashMap::new(),
            counter: 0,
            pathset: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let contents =
            fs::read_to_string("resources/12t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut nodes = Nodes::new();
        nodes.load_conns(split);
        println!("After step :\n{:#?}", nodes);
        nodes.count("start", &Vec::new());
        assert_eq!(19, nodes.counter);
    }
    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("resources/12t2.txt")
            .expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut nodes = Nodes::new();
        nodes.load_conns(split);
        println!("After step :\n{:#?}", nodes);
        nodes.count("start", &Vec::new());
        assert_eq!(226, nodes.counter);
    }
    #[test]
    fn test_part3() {
        let contents = fs::read_to_string("resources/12t0.txt")
            .expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut nodes = Nodes::new();
        nodes.load_conns(split);
        println!("After step :\n{:#?}", nodes);
        nodes.count_twice("start", &Vec::new(), &false, &Vec::new());
        assert_eq!(36, nodes.pathset.len());
        // assert_eq!(36, nodes.counter);
    }
    #[test]
    fn test_part4() {
        let contents =
            fs::read_to_string("resources/12t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut nodes = Nodes::new();
        nodes.load_conns(split);
        println!("After step :\n{:#?}", nodes);
        nodes.count_twice("start", &Vec::new(), &false, &Vec::new());
        assert_eq!(103, nodes.pathset.len());
        // assert_eq!(36, nodes.counter);
    }
    #[test]
    fn test_part5() {
        let contents = fs::read_to_string("resources/12t2.txt")
            .expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut nodes = Nodes::new();
        nodes.load_conns(split);
        println!("After step :\n{:#?}", nodes);
        nodes.count_twice("start", &Vec::new(), &false, &Vec::new());
        assert_eq!(3509, nodes.pathset.len());
        // assert_eq!(36, nodes.counter);
    }
}
