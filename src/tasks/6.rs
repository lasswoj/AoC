use std::{fmt, fs};
use std::collections::VecDeque;

fn main() {
    let contents =
        fs::read_to_string("resources/6.txt").expect("Something went wrong reading the file");
    let split = contents.split(",");
    let vals: Vec<u8> = split.map(|s| s.parse().unwrap()).collect();
    let mut sea = Sea::new(vals.clone());
    for _ in 0..80{
        sea.cycle();
    }
    // println!("there are :{} lanternfishin the sea",sea.lanternfish.len());
    println!("there are : {} lanternfishin the sea",sea.lanternfish.len());
    println!("there are : {} lanternfishin the sea after {} days",cycle_count(&vals,256), 256)


}


#[derive(Clone,Debug)]
struct Lanternfish{
    cycle: u8
}

impl fmt::Display for Lanternfish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cycle)
    }
}

impl Lanternfish{
    fn new(init: u8) -> Lanternfish{
        Lanternfish{cycle:init}
    }
    fn decr(&mut self) -> Option<Lanternfish>{
        match self.cycle {
            0 => {
                self.cycle=6;
                Some(Lanternfish::new(8))
            },
            _ => { self.cycle-=1; None }
        } 
    }
}


#[derive(Clone,Debug)]
struct Sea{
    lanternfish: Vec<Lanternfish>,
    day:u8
}

impl fmt::Display for Sea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut p_vec: Vec<String> = Vec::new();
        for fish in &self.lanternfish {
            p_vec.push(format!("{}", fish));
        }
        let print = p_vec.join(", ");
        write!(f, "After  {} days:  {}", self.day, print)
    }
}

impl Sea{
    fn new(vec: Vec<u8>)-> Sea{
        let mut sea: Vec<Lanternfish> = Vec::new();
        for i in vec{
            sea.push(Lanternfish::new(i))
        };
        Sea{lanternfish:sea, day:0}
    }
    fn cycle(&mut self){
        self.day+=1;
        let n_parents = self.lanternfish.len();
        for i in 0..n_parents{
            if let Some(i)=self.lanternfish[i].decr(){
                self.lanternfish.push(i);
            }
        }
        // println!("there are fish in cycle s: {}", self)
    }
}

fn cycle_count(init_vect: &Vec<u8>, iterations:u16)->u64{
    let mut deque: VecDeque<u64> = VecDeque::from(vec![0;9]);
    for i in init_vect{
        deque[*i as usize]+=1;
    }
    for _ in 1..iterations+1{
        let val = deque.pop_front().unwrap();
        deque[6]+=val;
        deque.push_back(val);
        // println!("day {}\t and {:?}",i,vec)
    }
    let sum: u64=  deque.iter().sum();
    sum
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let contents =
            fs::read_to_string("resources/6t.txt").expect("Something went wrong reading the file");
        let split = contents.split(",");
        let vals: Vec<u8> = split.map(|s| s.parse().unwrap()).collect();
        let mut sea = Sea::new(vals);
        for _ in 0..18{
            sea.cycle();
        }
        assert_eq!(26, sea.lanternfish.len());
        
        // let pairs = destruct(split);
        // let mut board = Board::new(&pairs);
        // board.draw_board(&pairs);
    }
    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("resources/6t.txt").expect("Something went wrong reading the file");
        let split = contents.split(",");
        let vals: Vec<u8> = split.map(|s| s.parse().unwrap()).collect();
        let mut sea = Sea::new(vals);
        for _ in 0..80{
            sea.cycle();
        }
        assert_eq!(5934, sea.lanternfish.len());
    }

    #[test]
    fn test_part3() {
        let contents =
            fs::read_to_string("resources/6t.txt").expect("Something went wrong reading the file");
        let split = contents.split(",");
        let vals: Vec<u8> = split.map(|s| s.parse().unwrap()).collect();
        assert_eq!(26, cycle_count(&vals,18));
        assert_eq!(5934, cycle_count(&vals,80));
        assert_eq!(26984457539, cycle_count(&vals,256));
    }
}
