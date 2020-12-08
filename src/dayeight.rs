use std::fs;

pub fn execute_day_eight(){
  let path = "./input/day8.txt";
  let working = prepare_input(path);
}

fn prepare_input (filepath: &str) -> Vec<String> {
  let mut ret: Vec<String> = Vec::new();
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");

  for lin in list.lines() {
    ret.push(lin.to_string().clone());
  }
  return ret;
}