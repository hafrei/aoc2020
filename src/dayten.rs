use std::fs;

pub fn execute_dayten() {
  let path = "./input/day10.txt";
  let working = prepare_input(path);
  not_silly(working.clone());
  let haha_what = oh_no(working);
  println!("If this isn't 0 then it worked: {}", haha_what);
}

/*
  Each number can be itself or up to -3 
  Find the item on the list with the highest number and +3
  Start at 0
  At each level, check for all numbers within +3 (filter / drain)
    for each option
 */

 fn not_silly(really: Vec<i32>) {

  let mut for_really = really.clone();
  println!("for_really is {} long", for_really.len());
  // This magic line gets the maximum value from an array.
  // If there are multiples it will start from the end of the array
  let iunno = *really.iter().max_by(|x, y| x.cmp(y)).unwrap();
  // we have the number, but where does it live in the array? Go over the array and match against 
  let lives_at = really.iter().position(|&x| x == iunno).unwrap();
  // And now for_really is one shorter
  let claim = for_really.remove(lives_at);
  println!("{} lived at {}, now for_really is {}", claim, lives_at, for_really.len());

 }

fn oh_no(lol: Vec<i32>) -> i32 {
  let mut ret = 0;
  let mut hahaha = lol.clone();

  if lol.len() != 0 {
    ret += hahaha.remove(0);
    ret += oh_no(hahaha);
  }

  return ret;
}

fn prepare_input (filepath: &str) -> Vec<i32> {
  let mut ret: Vec<i32> = Vec::new();
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");

  for lin in list.lines() {
    ret.push(lin.parse::<i32>().unwrap());
  }
  return ret;
}