use std::{fs, str::Split, vec};

fn main() {
    let contents =
        fs::read_to_string("resources/8.txt").expect("Something went wrong reading the file");
    let split = contents.split("\n");
    println!("min cost is : {} ", counter(split.clone()));
    let mut ret: u32 = 0;
    for line in split {
        ret += decoder(line);
    }
    println!("value is {}", ret)
}

fn counter(split: Split<&str>) -> u16 {
    let mut counter: u16 = 0;
    for line in split {
        let mut sides = line.split(" | ");
        let _ = sides.next();
        if let Some(right) = sides.next() {
            let vals = right.split(" ");
            for val in vals {
                let tempval = val.len();
                if tempval == 2 || tempval == 3 || tempval == 4 || tempval == 7 {
                    counter += 1;
                }
            }
        }
    }
    counter
}

struct Decoder {
    encoded: Vec<String>,
}

impl Decoder {
    fn comparator(str: &String, pattern: &String) -> bool {
        for (_, c) in pattern.chars().enumerate() {
            if str.contains(c) {
            } else {
                return false;
            }
        }
        true
    }

    fn new(mut coded: Vec<String>) -> Decoder {
        let mut dec = Decoder {
            encoded: vec![String::from(""); 10],
        };
        for i in (0..coded.len()).rev() {
            let code = &coded[i];
            if code.len() == 2 {
                dec.encoded[1] = code.to_string();
                coded.remove(i);
            } else if code.len() == 3 {
                dec.encoded[7] = code.to_string();
                coded.remove(i);
            } else if code.len() == 4 {
                dec.encoded[4] = code.to_string();
                coded.remove(i);
            } else if code.len() == 7 {
                dec.encoded[8] = code.to_string();
                coded.remove(i);
            }
        }
        for i in (0..coded.len()).rev() {
            let code = &coded[i];
            if code.len() == 6 {
                if Decoder::comparator(code, &dec.encoded[4]) {
                    dec.encoded[9] = code.to_string();
                    coded.remove(i);
                } else if Decoder::comparator(code, &dec.encoded[7]) {
                    dec.encoded[0] = code.to_string();
                    coded.remove(i);
                } else {
                    dec.encoded[6] = code.to_string();
                    coded.remove(i);
                }
            }
        }
        for i in (0..coded.len()).rev() {
            let code = &coded[i];
            if code.len() == 5 {
                if Decoder::comparator(code, &dec.encoded[7]) {
                    dec.encoded[3] = code.to_string();
                    coded.remove(i);
                } else if Decoder::comparator(&dec.encoded[6], code) {
                    dec.encoded[5] = code.to_string();
                    coded.remove(i);
                } else {
                    dec.encoded[2] = code.to_string();
                    coded.remove(i);
                }
            }
        }

        dec
    }

    fn decode(&self, coded: Vec<String>) -> u32 {
        let mut ret: u32 = 0;
        for code in coded {
            ret *= 10;
            for val in 0..self.encoded.len() {
                if Decoder::comparator(&self.encoded[val], &code)
                    && self.encoded[val].len() == code.len()
                {
                    ret += val as u32;
                }
            }
        }
        ret
    }
}

fn decoder(line: &str) -> u32 {
    let mut sides = line.split(" | ");
    let decoder: Decoder;

    let left = sides.next().unwrap();
    let coded: Vec<String> = left.split(" ").map(|s| s.parse().unwrap()).collect();
    decoder = Decoder::new(coded);
    let right = sides.next().unwrap();
    let vals = right.split(" ").map(|s| s.parse().unwrap()).collect();
    decoder.decode(vals)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let contents =
            fs::read_to_string("resources/8t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        assert_eq!(26, counter(split));
    }
    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("resources/8t.txt").expect("Something went wrong reading the file");
        let split = contents.split("\n");
        let mut ret: u32 = 0;
        for line in split {
            ret += decoder(line);
        }
        println!("value is {}", ret)
    }
}
