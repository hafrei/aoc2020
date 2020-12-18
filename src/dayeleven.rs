use std::fs;

pub fn execute_dayeleven(){
  let path = "./input/day11.txt";
  let working= prepare_input(path);
}

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