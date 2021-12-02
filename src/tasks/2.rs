
use std::fs;
use std::collections::VecDeque;

struct FTask{
    horisontal : i32,
    depth: i32
 }
 
 impl FTask {
    fn new() -> FTask {
       FTask { horisontal: 0, depth: 0 }
   }
 
    fn count(&mut self, direction :&str , value :&str ){
       let int_val = value.parse::<i32>().unwrap();
 
         let s = String::from(direction);
       match s.as_str() {
         "forward" => {
             self.horisontal+=int_val;
          },
          "up" => {
             self.depth-=int_val;
           },
           "down" => {
             self.depth+=int_val;
            },
          _ => {
              println!("No");
          }
     }
       
 
    }
 }
 
 struct STask{
    horisontal : i32,
    depth: i32,
    aim: i32
 }
 
 impl STask {
    fn new() -> STask {
        STask { horisontal: 0, depth: 0,  aim: 0}
   }
 
    fn count(&mut self, direction :&str , value :&str ){
       let int_val = value.parse::<i32>().unwrap();
 
         let s = String::from(direction);
       match s.as_str() {
         "forward" => {
            self.horisontal+=int_val;
            self.depth+=int_val*self.aim;
          },
          "up" => {
            self.aim-=int_val;
           },
           "down" => {
            self.aim+=int_val;
            },
          _ => {
              println!("No");
          }
     }
       
 
    }
 }
 
  
fn main() {
    let contents = fs::read_to_string("resources/2.txt")
        .expect("Something went wrong reading the file");
         let mut split = contents.split("\n");

        let mut f = FTask::new();
        let mut f2 = STask::new();
         
         for  i in split{
             let mut spi = i.split_whitespace();
             let dir = spi.next().unwrap();
             let val = spi.next().unwrap();

             f.count(&dir,&val);
             f2.count(&dir,&val);
         }
         print!("count 1 :\n{}\n", f.horisontal*f.depth);
         print!("count 2 :\n{}\n", f2.horisontal*f2.depth);
}