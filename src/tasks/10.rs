use std::{collections::HashMap, fs};

fn closers(c: &char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '<' => Some('>'),
        '{' => Some('}'),
        _ => None,
    }
}

fn main() {
    let contents =
        fs::read_to_string("resources/10.txt").expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let split2 = split.clone();
    let mut vecer: Vec<u64> = Vec::new();
    for i in split2 {
        let val = checker2(i);
        if val > 0 {
            vecer.push(val);
        }
    }
    vecer.sort();
    let counter: u64 = vecer[vecer.len() / 2];

    println!("the value is : {}", counter);
}

fn checker(str: &str) -> u32 {
    let oppeners: Vec<char> = Vec::from(['(', '[', '{', '<']);

    let points: HashMap<char, u32> = HashMap::from(
        [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
            .iter()
            .cloned()
            .collect(),
    );
    let mut queue: Vec<char> = Vec::new();
    for i in str.chars() {
        if oppeners.contains(&i) {
            queue.push(i)
        } else {
            let last = queue.last().unwrap();
            if i == closers(last).unwrap() {
                let _ = queue.pop();
            } else {
                return points[&i];
            }
        }
    }
    return 0;
}

fn checker2(str: &str) -> u64 {
    let mut counter: u64 = 0;

    let oppeners: Vec<char> = Vec::from(['(', '[', '{', '<']);

    let points: HashMap<char, u64> = HashMap::from(
        [('(', 1), ('[', 2), ('{', 3), ('<', 4)]
            .iter()
            .cloned()
            .collect(),
    );
    let mut queue: Vec<char> = Vec::new();
    for i in str.chars() {
        if oppeners.contains(&i) {
            queue.push(i)
        } else {
            let last = queue.last().unwrap();
            if i == closers(last).unwrap() {
                let _ = queue.pop();
            } else {
                return 0;
            }
        }
    }
    while queue.len() > 0 {
        counter *= 5;
        let ch = queue.pop().unwrap();
        counter += points[&ch];
    }
    return counter;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let contents =
            fs::read_to_string("resources/10t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut counter: u32 = 0;
        for i in split {
            counter += checker(i)
        }
        assert_eq!(26397, counter);
    }
    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("resources/10t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut vecer: Vec<u64> = Vec::new();
        for i in split {
            let val = checker2(i);
            if val > 0 {
                vecer.push(val);
            }
        }
        vecer.sort();
        let counter: u64 = vecer[vecer.len() / 2];
        assert_eq!(288957, counter);
    }
}
