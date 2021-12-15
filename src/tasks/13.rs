use std::collections::VecDeque;
use std::fmt;
use std::{fs, str::Split};
fn main() {
    let contents =
        fs::read_to_string("resources/13.txt").expect("Something went wrong reading the file");
    let mut split = contents.split("\n");
    let mut board: Board = Board::new(&mut split);
    while board.axis.len() > 0 {
        board.fold();
    }
    println!("board is \n{} \n with count {}", board, board.count());
}

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Clone, Debug)]
struct Axis {
    orientation: char,
    value: u16,
}

#[derive(Clone)]
struct Board {
    board: Vec<Vec<i8>>,
    axis: VecDeque<Axis>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut p_vec: Vec<String> = Vec::new();
        for row in &self.board {
            p_vec.push(format!("{:?}", row));
        }
        for axis in &self.axis {
            p_vec.push(format!("{:?}", axis));
        }
        let print = p_vec.join("\n");
        write!(f, "{}", print)
    }
}

impl Board {
    fn new(mut split: &mut Split<&str>) -> Board {
        let points: Vec<Point> = Board::to_vect(&mut split);
        let axies: VecDeque<Axis> = Board::to_folds(&mut split);
        let mut m_x: u16 = 0;
        let mut m_y: u16 = 0;
        for point in &points {
            if point.x > m_x {
                m_x = point.x;
            }
            if point.y > m_y {
                m_y = point.y;
            }
        }
        let mut board: Vec<Vec<i8>> = vec![vec![0; (m_x + 1) as usize]; (m_y + 1) as usize];
        Board::draw(&mut board, points);
        let board = Board {
            board: board,
            axis: axies,
        };
        board
    }

    fn draw(board: &mut Vec<Vec<i8>>, points: Vec<Point>) {
        for Point { x, y } in points {
            board[y as usize][x as usize] = 1;
        }
    }

    fn to_folds(split: &mut Split<&str>) -> VecDeque<Axis> {
        let mut axies: VecDeque<Axis> = VecDeque::new();
        for line in split {
            let pair: Vec<&str> = line.split(" ").collect::<Vec<&str>>()[2]
                .split("=")
                .collect();
            axies.push_back(Axis {
                orientation: pair[0].parse().unwrap(),
                value: pair[1].parse().unwrap(),
            })
        }
        axies
    }
    fn to_vect(split: &mut Split<&str>) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        for line in split {
            if line == "" {
                break;
            }
            let pair: Vec<u16> = line.split(",").map(|pair| pair.parse().unwrap()).collect();
            points.push(Point {
                x: pair[0],
                y: pair[1],
            });
        }
        points
    }
    fn count(&self) -> u32 {
        let mut count: u32 = 0;
        for i in &self.board {
            for j in i {
                count += *j as u32;
            }
        }
        count
    }
    fn fold(&mut self) {
        let ax = self.axis.pop_front();
        match ax {
            Some(Axis {
                orientation: 'y',
                value: y,
            }) => {
                let mylen = (y * 2 + 1) as usize;
                let boardlen = self.board.len();
                self.board
                    .append(&mut vec![vec![0; self.board[0].len()]; mylen - boardlen]);
                let mut newboard: Vec<Vec<i8>> = Vec::new();
                for i in 0..y {
                    let mut tempvec: Vec<i8> = vec![0; self.board[0].len()];
                    for j in 0..self.board[0].len() {
                        tempvec[j] = (self.board[i as usize][j] != 0
                            || self.board[self.board.len() - 1 - i as usize][j] != 0)
                            as i8;
                    }
                    newboard.push(tempvec);
                }
                self.board = newboard;
            }
            Some(Axis {
                orientation: 'x',
                value: x,
            }) => {
                if x * 2 + 1 != (self.board[0].len()) as u16 {
                    println!("error while x= {}", x)
                }
                let mut newboard: Vec<Vec<i8>> = Vec::new();
                for i in 0..self.board.len() {
                    let mut tempvec: Vec<i8> = vec![0; x as usize];
                    for j in 0..x {
                        tempvec[j as usize] = (self.board[i][j as usize] != 0
                            || self.board[i][self.board[0].len() - 1 - j as usize] != 0)
                            as i8;
                    }
                    newboard.push(tempvec);
                }
                self.board = newboard;
            }
            Some(_) => todo!(),
            None => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let contents =
            fs::read_to_string("resources/13t.txt").expect("Something went wrong reading the file");
        let mut split = contents.split("\n");
        let mut board: Board = Board::new(&mut split);
        println!("board is \n{} \n with count {}", board, board.count());
        board.fold();
        println!("board is \n{} \n with count {}", board, board.count());
        board.fold();
        println!("board is \n{} \n with count {}", board, board.count());
    }
    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("resources/13t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
    }
}
