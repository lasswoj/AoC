use std::collections::VecDeque;
use std::fs;
use std::str::SplitWhitespace;

struct FTask {
    prev: i32,
    counter: i32,
}

impl FTask {
    fn new(s_prev: &str) -> FTask {
        FTask {
            prev: s_prev.parse::<i32>().unwrap(),
            counter: 0,
        }
    }

    fn count(&mut self, now: &str) {
        let my_int = now.parse::<i32>().unwrap();
        if my_int > self.prev {
            self.counter += 1;
        }
        self.prev = my_int;
    }
}

struct STask {
    prev: VecDeque<i32>,
    prevsum: i32,
    counter: i32,
}

impl STask {
    fn new(prev1: &str, prev2: &str, prev3: &str) -> STask {
        let deque = VecDeque::from(
            [
                prev1.parse::<i32>().unwrap(),
                prev2.parse::<i32>().unwrap(),
                prev3.parse::<i32>().unwrap(),
            ]
            .to_vec(),
        );
        let sum: i32 = deque.iter().sum();

        STask {
            prev: VecDeque::from(deque),
            prevsum: sum,
            counter: 0,
        }
    }

    fn count(&mut self, now: &str) {
        let my_int = now.parse::<i32>().unwrap();
        self.prev.pop_front();
        self.prev.push_back(my_int);
        let sum = self.prev.iter().sum();

        if sum > self.prevsum {
            self.counter += 1;
        }
        self.prevsum = sum;
    }
}

fn task1(mut split: SplitWhitespace) -> i32 {
    let i1 = split.next().unwrap();
    let mut f = FTask::new(&i1);
    for i in split {
        f.count(&i);
    }
    f.counter
}

fn task2(mut split: SplitWhitespace) -> i32 {
    let i1 = split.next().unwrap();
    let i2 = split.next().unwrap();
    let i3 = split.next().unwrap();
    let mut f2 = STask::new(&i1, &i2, &i3);
    for i in split {
        f2.count(&i);
    }
    f2.counter
}

fn main() {
    let contents =
        fs::read_to_string("resources/1.txt").expect("Something went wrong reading the file");
    let split = contents.split_whitespace();
    let split2 = split.clone();
    print!(
        "count 1 :\n{}\ncount 2 :\n{}\n",
        task1(split),
        task2(split2)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let contents =
            fs::read_to_string("resources/1t.txt").expect("Something went wrong reading the file");
        let split = contents.split_whitespace();
        assert_eq!(7, task1(split));
    }

    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("resources/1t.txt").expect("Something went wrong reading the file");
        let split = contents.split_whitespace();
        assert_eq!(5, task2(split));
    }
}
