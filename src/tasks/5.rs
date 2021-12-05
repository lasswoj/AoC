use std::{fmt, fs, str::Split};

fn main() {
    let contents =
        fs::read_to_string("resources/5.txt").expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let pairs = destruct(split);
    let mut board = Board::new(&pairs);
    // no time to
    let mut board2 = board.clone();
    board.draw_board(&pairs);
    println!("Result is {}\n", board.count());
    board2.draw_board_diag(&pairs);
    println!("Result2 is {}\n", board2.count());
    // assert_eq!(5,board.count())
}

#[derive(Debug)]
struct Point {
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct Vector {
    path: Vec<Point>,
}

impl Vector {
    fn xvect(p_from: &Point, mut d_x: i16) -> Vector {
        let mut path: Vec<Point> = Vec::new();
        let mut mux_x: i16 = 1;
        if d_x < 0 {
            d_x = -d_x;
            mux_x = -1;
        }
        for i in 0..d_x + 1 {
            path.push(Point {
                y: p_from.y,
                x: p_from.x + (i * mux_x),
            })
        }
        Vector { path }
    }
    fn yvect(p_from: &Point, mut d_y: i16) -> Vector {
        let mut path: Vec<Point> = Vec::new();
        let mut mux_y: i16 = 1;
        if d_y < 0 {
            d_y = -d_y;
            mux_y = -1;
        }
        for i in 0..d_y + 1 {
            path.push(Point {
                x: p_from.x,
                y: p_from.y + (i * mux_y),
            })
        }
        Vector { path }
    }

    fn divect(p_from: &Point, mut d_y: i16, d_x: i16) -> Vector {
        let mut path: Vec<Point> = Vec::new();
        let mut mux_y: i16 = 1;
        let mut mux_x: i16 = 1;
        if d_x < 0 {
            mux_x = -1;
        }
        if d_y < 0 {
            d_y = -d_y;
            mux_y = -1
        }
        for i in 0..d_y + 1 {
            path.push(Point {
                x: p_from.x + (i * mux_x),
                y: p_from.y + (i * mux_y),
            })
        }
        Vector { path }
    }

    fn from_points(p_from: &Point, p_to: &Point) -> Vector {
        let d_x = p_to.x - p_from.x;
        let d_y = p_to.y - p_from.y;
        if d_x != 0 && d_y != 0 {
            return Vector { path: Vec::new() };
        }
        if d_x != 0 {
            return Vector::xvect(p_from, d_x);
        }
        if d_y != 0 {
            return Vector::yvect(p_from, d_y);
        }
        Vector { path: Vec::new() }
    }

    fn from_points_diag(p_from: &Point, p_to: &Point) -> Vector {
        let d_x = p_to.x - p_from.x;
        let d_y = p_to.y - p_from.y;
        if d_x != 0 && d_y != 0 {
            if d_x == d_y || d_x == -d_y {
                return Vector::divect(p_from, d_y, d_x);
            } else {
                return Vector { path: Vec::new() };
            }
        }
        if d_x != 0 {
            return Vector::xvect(p_from, d_x);
        }
        if d_y != 0 {
            return Vector::yvect(p_from, d_y);
        }
        Vector { path: Vec::new() }
    }
}

fn destruct(split: Split<&str>) -> Vec<Vec<Point>> {
    let mut points: Vec<Vec<Point>> = Vec::new();
    for i in split {
        let mut pair: Vec<Point> = Vec::new();
        let mypair = i.split(" -> ");
        for j in mypair {
            let p_vals: Vec<i16> = j.split(",").map(|s| s.parse().unwrap()).collect();
            let point = Point {
                x: p_vals[0],
                y: p_vals[1],
            };
            pair.push(point)
        }
        points.push(pair)
    }
    points
}

#[derive(Clone)]
struct Board {
    board: Vec<Vec<i8>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut p_vec: Vec<String> = Vec::new();
        for row in &self.board {
            p_vec.push(format!("{:?}", row));
        }
        let print = p_vec.join("\n");
        write!(f, "{}", print)
    }
}

impl Board {
    fn new(points: &Vec<Vec<Point>>) -> Board {
        let mut m_x: i16 = 0;
        let mut m_y: i16 = 0;
        for pair in points {
            for point in pair {
                if point.x > m_x {
                    m_x = point.x;
                }
                if point.y > m_y {
                    m_y = point.y;
                }
            }
        }
        Board {
            board: vec![vec![0; (m_x + 1) as usize]; (m_y + 1) as usize],
        }
    }

    fn draw_board(&mut self, pairs: &Vec<Vec<Point>>) {
        for pair in pairs {
            let line = Vector::from_points(&pair[0], &pair[1]);
            // println!("\n{:?}",pair);
            if line.path.len() == 0 {
                continue;
            }
            for i in &line.path {
                self.board[i.y as usize][i.x as usize] += 1;
            }
            // println!("{:?}\n",line);
            // println!("the board is \n{}",self);
        }
        // println!("{:?}\n",line);
        // println!("the board is \n{}",self);
    }

    fn draw_board_diag(&mut self, pairs: &Vec<Vec<Point>>) {
        for pair in pairs {
            let line = Vector::from_points_diag(&pair[0], &pair[1]);
            // println!("\n{:?}",pair);
            if line.path.len() == 0 {
                continue;
            }
            for i in &line.path {
                self.board[i.y as usize][i.x as usize] += 1;
            }
        }
        // println!("{:?}\n",line);
        // println!("the board is \n{}",self);
    }
    fn count(&self) -> i16 {
        let mut counter: i16 = 0;
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] > 1 {
                    counter += 1;
                }
            }
        }
        counter
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let contents =
            fs::read_to_string("resources/5t.txt").expect("Something went wrong reading the file");
        let mut split = contents.split("\n");
        let pairs = destruct(split);
        let mut board = Board::new(&pairs);
        board.draw_board(&pairs);
        assert_eq!(5, board.count())
    }

    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("resources/5t.txt").expect("Something went wrong reading the file");
        let mut split = contents.split("\n");
        let pairs = destruct(split);
        let mut board = Board::new(&pairs);
        board.draw_board_diag(&pairs);
        assert_eq!(12, board.count())
    }
}
