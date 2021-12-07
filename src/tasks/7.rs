use std::collections::HashMap;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("resources/7.txt").expect("Something went wrong reading the file");
    let split = contents.split(",");
    let vals: Vec<u16> = split.map(|s| s.parse().unwrap()).collect();
    println!("min cost is : {} ", move_cost(&vals));
    println!("propper min cost is : {} ", move_cost_propper(&vals));
}

fn move_cost(vals: &Vec<u16>) -> u32 {
    let mut map: HashMap<u16, u32> = HashMap::new();
    let mut minval = std::u32::MAX;
    for i in vals {
        if let Some(_) = map.get(i) {
            continue;
        } else {
            let mut locval: u32 = 0;
            for j in vals {
                locval += (*i as i32 - *j as i32).abs() as u32;
            }
            if minval > locval {
                minval = locval;
            }
            map.insert(*i, locval);
        }
    }
    minval
}

fn move_cost_propper(vals: &Vec<u16>) -> u32 {
    let mut minval = std::u32::MAX;
    for i in 0..vals.len() {
        let mut locval: u32 = 0;
        for j in vals {
            let reach = (i as i32 - *j as i32).abs() as u32;
            let mut val_up: u32 = 0;
            for i in 0..(reach + 1) {
                val_up += i;
            }
            locval += val_up;
        }
        if minval > locval {
            minval = locval;
        }
    }
    minval
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let contents =
            fs::read_to_string("resources/7t.txt").expect("Something went wrong reading the file");
        let split = contents.split(",");
        let vals: Vec<u16> = split.map(|s| s.parse().unwrap()).collect();
        assert_eq!(37, move_cost(&vals));
    }
    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("resources/7t.txt").expect("Something went wrong reading the file");
        let split = contents.split(",");
        let vals: Vec<u16> = split.map(|s| s.parse().unwrap()).collect();
        assert_eq!(168, move_cost_propper(&vals));
    }
}
