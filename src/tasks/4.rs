use std::collections::VecDeque;
use std::{fs, str::Split, vec};
fn main() {
    let contents =
        fs::read_to_string("resources/4.txt").expect("Something went wrong reading the file");
    let mut split = contents.split("\n");
    let vs = split.next().unwrap().split(",");
    let mut vals: VecDeque<u16> = vs.map(|s| s.parse().unwrap()).collect();
    let mut boards = create_boards(split);
    while boards.len() > 1 {
        let res = flow(&mut boards, &mut vals);
        println!("result = {}", res)
    }
    let res = flow(&mut boards, &mut vals);
    println!("result = {}", res)
}

struct Board {
    board: Vec<Vec<u16>>,
    label: Vec<Vec<u8>>,
}

impl Board {
    fn new(vec: Vec<Vec<u16>>) -> Board {
        Board {
            board: vec,
            label: vec![vec![0 as u8; 5]; 5],
        }
    }
    fn find(&mut self, new: u16) {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == new {
                    self.label[i][j] = 1;
                }
            }
        }
    }
    fn check(&mut self) -> u16 {
        for i in 0..5 {
            if self.label[i][0] == 1
                && self.label[i][1] == 1
                && self.label[i][2] == 1
                && self.label[i][3] == 1
                && self.label[i][4] == 1
            {
                return self.calculate();
            }
        }
        for i in 0..5 {
            if self.label[0][i] == 1
                && self.label[1][i] == 1
                && self.label[2][i] == 1
                && self.label[3][i] == 1
                && self.label[4][i] == 1
            {
                return self.calculate();
            }
        }
        0
    }
    fn calculate(&self) -> u16 {
        let mut counter: u16 = 0;
        for i in 0..5 {
            for j in 0..5 {
                if self.label[i][j] == 0 {
                    counter += self.board[i][j];
                }
            }
        }
        counter
    }
}

fn create_boards(mut split: Split<&str>) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    while true {
        let line = split.next();
        match line {
            None => break,
            Some(_) => {}
        }
        let mut grid: Vec<Vec<u16>> = Vec::new();
        for _ in 0..5 {
            let str = split.next().unwrap();
            let chars = str.split_whitespace();
            let mut row: Vec<u16> = Vec::new();
            for char in chars {
                row.push(char.parse::<u16>().unwrap());
            }
            grid.push(row);
        }
        boards.push(Board::new(grid))
    }
    boards
}

fn flow(boards: &mut Vec<Board>, vals: &mut VecDeque<u16>) -> u16 {
    let mut score: u16 = 0;
    let mut win_index = 0;

    while !vals.is_empty() {
        let val = vals[0];
        for (i, item) in boards.iter_mut().enumerate() {
            item.find(val);
            let sum = item.check();
            if sum > 0 {
                win_index = i;
                score = sum * val;
            }
        }
        if score > 0 {
            boards.remove(win_index);
            return score;
        }
        vals.pop_front().unwrap();
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let contents =
            fs::read_to_string("resources/4t.txt").expect("Something went wrong reading the file");
        let mut split = contents.split("\n");
        let vs = split.next().unwrap().split(",");
        let mut vals: VecDeque<u16> = vs.map(|s| s.parse().unwrap()).collect();
        let mut boards = create_boards(split);
        let res = flow(&mut boards, &mut vals);
        assert_eq!(4512, res)
    }

    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("resources/4t.txt").expect("Something went wrong reading the file");
        let mut split = contents.split("\n");
        let vs = split.next().unwrap().split(",");
        let mut vals: VecDeque<u16> = vs.map(|s| s.parse().unwrap()).collect();
        let mut boards = create_boards(split);
        while boards.len() > 1 {
            let res = flow(&mut boards, &mut vals);
        }
        let res = flow(&mut boards, &mut vals);
    }
}
