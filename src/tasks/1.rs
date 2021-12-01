
use std::fs;
use std::collections::VecDeque;

struct FTask{
   prev : i32,
   counter: i32
}

impl FTask {
   fn new(s_prev: &str) -> FTask {
      FTask { prev: s_prev.parse::<i32>().unwrap(), counter: 0 }
  }

   fn count(&mut self, now :&str ){
      let my_int = now.parse::<i32>().unwrap();
      if my_int >self.prev {
         self.counter+=1;

      }
      self.prev = my_int;

   }
}

struct STask{
   prev : VecDeque<i32>,
   prevsum: i32,
   counter: i32
}

impl STask {
   fn new(prev1: &str, prev2: &str, prev3: &str,) -> STask {
      let deque = VecDeque::from([prev1.parse::<i32>().unwrap(), 
                        prev2.parse::<i32>().unwrap(), 
                        prev3.parse::<i32>().unwrap()]
                        .to_vec());
      let sum: i32 = deque.iter().sum();

      STask { 
         prev: VecDeque::from(deque),
         prevsum: sum,
         counter: 0, 
      }
  }

   fn count(&mut self, now :&str ){
      let my_int = now.parse::<i32>().unwrap();
      self.prev.pop_front();
      self.prev.push_back(my_int);
      let sum = self.prev.iter().sum();

      if sum >self.prevsum {
         self.counter+=1;

      }
      self.prevsum = sum;

   }
}


fn main() {
    let contents = fs::read_to_string("resources/1.txt")
        .expect("Something went wrong reading the file");
         let mut split = contents.split_whitespace();

         let i1 = split.next().unwrap();
         let i2 = split.next().unwrap();
         let i3 = split.next().unwrap();

         let mut f = FTask::new(&i1);
         f.count(&i2);
         f.count(&i3);

         let mut f2 = STask::new(&i1, &i2, &i3);
         
         for  i in split{
            f.count(&i);
            f2.count(&i);
         }
         print!("count 1 :\n{}\ncount 2 :\n{}\n", f.counter, f2.counter);
}