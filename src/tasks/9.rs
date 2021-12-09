use std::{fmt, fs, str::Split, vec};

fn main() {
    let contents =
        fs::read_to_string("resources/9.txt").expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let split2 = split.clone();

    println!("min sum is : {}", finder(split));
    let mut board = Board::new(split2);
    board.segmentize();
    let ret = board.segments[0] as u32 * board.segments[1] as u32 * board.segments[2] as u32;

    println!("the value is : {}", ret);
}

// thought i am gonna outsmart them... LOL
fn finder(split: Split<&str>) -> usize {
    let board = Board::new(split);
    let mat = board.board;
    let mut mins: Vec<Vec<usize>> = Vec::new();
    for i in 1..mat.len() - 1 {
        for j in 1..mat[0].len() - 1 {
            if mat[i][j] < mat[i + 1][j]
                && mat[i][j] < mat[i][j + 1]
                && mat[i][j] < mat[i - 1][j]
                && mat[i][j] < mat[i][j - 1]
            {
                mins.push(vec![i, j]);
            }
        }
    }
    let mut counter: usize = 0;
    for i in mins {
        counter += mat[i[0]][i[1]] as usize + 1;
    }
    counter
    // mins.len()
}

#[derive(PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

struct Segment {
    count: u16,
    next: Vec<Point>,
}

impl Segment {
    fn new(seed: Point) -> Segment {
        Segment {
            count: 0,
            next: Vec::from(vec![seed]),
        }
    }

    fn append(&mut self, point: Point) {
        if !self.next.contains(&point) {
            self.next.push(point)
        }
    }

    fn segment(&mut self, board: &mut Vec<Vec<u8>>) -> u16 {
        while self.next.len() > 0 {
            self.count += 1;
            let c = self.next.pop().unwrap();
            board[c.x][c.y] = 9;
            if board[c.x + 1][c.y] < 9 {
                self.append(Point { x: c.x + 1, y: c.y })
            }
            if board[c.x][c.y + 1] < 9 {
                self.append(Point { x: c.x, y: c.y + 1 })
            }
            if board[c.x - 1][c.y] < 9 {
                self.append(Point { x: c.x - 1, y: c.y })
            }
            if board[c.x][c.y - 1] < 9 {
                self.append(Point { x: c.x, y: c.y - 1 })
            }
        }
        self.count
    }
}

struct Board {
    board: Vec<Vec<u8>>,
    segments: Vec<u16>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut p_vec: Vec<String> = Vec::new();
        for row in &self.board {
            p_vec.push(format!("{:?}", row));
        }
        p_vec.push(format!("\n",));
        let print = p_vec.join("\n");
        write!(f, "{}", print)
    }
}

impl Board {
    fn to_row(line: &str) -> Vec<u8> {
        let mut vec: Vec<u8> = vec![9];
        vec.append(
            &mut line
                .chars()
                .into_iter()
                .map(|s| s.to_digit(10).unwrap() as u8)
                .collect(),
        );
        vec.push(9);
        vec
    }
    fn new(mut spl: Split<&str>) -> Board {
        let mut board: Vec<Vec<u8>> = Vec::new();
        let first = spl.nth(0).unwrap();
        let width = first.len() + 2;
        // adding border
        board.push(vec![9; width]);
        board.push(Board::to_row(first));
        for line in spl {
            board.push(Board::to_row(line));
        }
        board.push(vec![9; width]);

        Board {
            board: board,
            segments: Vec::new(),
        }
    }

    fn segmentize(&mut self) {
        for i in 1..self.board.len() - 1 {
            for j in 1..self.board[0].len() - 1 {
                if self.board[i][j] < 9 {
                    let mut seg = Segment::new(Point { x: i, y: j });
                    self.segments.push(seg.segment(&mut self.board));
                    // println!("{}",self);
                }
            }
        }
        self.segments.sort();
        self.segments.reverse();
        // println!("{:?}",self.segments);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let contents =
            fs::read_to_string("resources/9t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        assert_eq!(15, finder(split));
    }
    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("resources/9t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut board = Board::new(split);
        board.segmentize();
        let ret = board.segments[0] as u32 * board.segments[1] as u32 * board.segments[2] as u32;
        // let ret = board.
        assert_eq!(1134, ret);
    }
}
