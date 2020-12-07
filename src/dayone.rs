use std::fs;

pub fn execute_dayone() { //344ms
  let path = "./input/day1.txt";
  let base: Vec<i32> = read_and_parse_file(path);
  let pair: Vec<i32> = get_pair(base.clone());
  let triple: Vec<i32> = get_triple(base.clone());

  println!("Pooped out a {} for pair", pair_multi(pair[0], pair[1]));
  println!("Pooped out a {} for multi", trip_multi(triple[0], triple[1], triple[2]));
}

fn get_triple(worklist: Vec<i32>) -> Vec<i32> {
  let mut breaker = false;
  let working: Vec<i32> = worklist.clone();
  let mut the_triple: Vec<i32> = Vec::new();

  for x in worklist {
      for y in &working {
          for z in &working {
              if x.clone() + y.clone() + z.clone() == 2020 {
                  the_triple.push(x.clone());
                  the_triple.push(y.clone());
                  the_triple.push(z.clone());
                  breaker = true;
              }
              if breaker {
                break;
              }
          }
        if breaker {
          break;
        }
      }
    if breaker {
      break;
    }
  }

  return the_triple;
}

fn get_pair(worklist: Vec<i32>) -> Vec<i32> {
  let mut breaker = false;
  let working: Vec<i32> = worklist.clone();
  let mut the_pair: Vec<i32> = Vec::new();

  for x in worklist {
      for y in &working {
          if x.clone() + y.clone() == 2020 {
              the_pair.push(x.clone());
              the_pair.push(y.clone());
              breaker = true;
          }
        if breaker {
          break;
        }
      } 
    if breaker {
      break;
    }  
  }
  return the_pair;
}

fn read_and_parse_file(path: &str) -> Vec<i32> {
  let mut extract: Vec<i32> = Vec::new();
  //Day 1: Find the two entries that sum to 2020; what do you get if you multiply them together?
  let list = fs::read_to_string(path).expect("Well that sure didn't open");
  
  for lin in list.lines() {
      extract.push(lin.parse::<i32>().unwrap().clone());
  }

  return extract;
}

fn pair_multi(x: i32, y: i32) -> i32 {
  return x * y;
}

fn trip_multi(x: i32, y: i32, z: i32) -> i32 {
  return x * y * z;
}