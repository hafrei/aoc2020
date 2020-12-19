use std::fs;

pub fn execute_dayeleven(){
  let path = "./input/day11.txt";
  let working= prepare_input(path);
}

/* Look into traits? 
  . Unoccupied; never changes
   L if not occupied and no adjacent occupied seats
   # occupied. If 4 adjacent seats are occupied, it becomes unoccupied

   Apply these rules until no more changes
   Count the number of occupied seats
*/


fn prepare_input (filepath: &str) -> Vec<String> {
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");
  let ret: Vec<String> = list
                      .as_str()
                      .split('\n')
                      .map(str::parse::<String>)
                      .map(Result::unwrap)
                      .collect();
  ret
}