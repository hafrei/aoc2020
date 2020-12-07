use std::fs;
use std::collections::HashMap;

pub fn execute_daysix() { 
  let path = "./input/day6.txt";
  let mut count = get_sum(get_everyones_answers(path));
  println!("Sum is {}", count);
  count = get_second_sum(get_group_answers(path));
  println!("Actually sum is {}", count);
}

fn get_sum(list: Vec<String>) -> i32 {
  let mut count: i32 = 0;

  for a in list {
    count += a.len() as i32;
  }
  return count;
}

fn get_second_sum(list: Vec<HashMap<char, i32>>) -> i32 {
  let mut count: i32 = 0;
  let pnd = "#".chars().next().unwrap();

  for a in list.into_iter() {
    let gimmie_max = a.get(&pnd).unwrap().clone();
    for (b, c) in a {
      if c.eq(&gimmie_max) && !b.eq(&pnd) {
        count += 1;
      }
    }
  }
  return count;
}

fn get_group_answers (filepath: &str) -> Vec<HashMap<char, i32>> {
  let mut ret = Vec::new();
  let mut hm = HashMap::new();
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");
  let mut builder: Vec<String> = Vec::new();

  for lin in list.lines() {
    if lin.len() == 0 {
      for derp in builder.clone() {
        for ch in derp.chars(){
          let what = hm
                              .entry(ch)
                              .and_modify(|e| { *e += 1 })
                              .or_insert(1);
        }

      }
      let leng = "#".chars().next().unwrap();
      let what2 = builder.len() as i32;
      hm.entry(leng).or_insert(what2);
      hm.retain(|_, v| *v == what2);
      ret.push(hm);
      hm = HashMap::new();
      builder = Vec::new();
    } else {
      builder.push(lin.to_string());
    }
  }

  if builder.len() != 0 {
    for derp in builder.clone() {
      for ch in derp.chars(){
        let what = hm
                            .entry(ch)
                            .and_modify(|e| { *e += 1 })
                            .or_insert(1);
      }

    }
    let leng = "#".chars().next().unwrap();
    let what2 = builder.len() as i32;
    hm.entry(leng).or_insert(what2);
    hm.retain(|_, v| *v == what2);
    ret.push(hm);
  }
  return ret;
}

fn get_everyones_answers (filepath: &str) -> Vec<String> {
  let mut ret: Vec<String> = Vec::new();
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");
  let mut builder = String::new();

  for lin in list.lines() {
    if lin.len() == 0 {
      let mut what: Vec<char> = builder.chars().collect();
      what.sort();
      what.dedup();
      ret.push(what.into_iter().collect());
      builder = String::new();
    } else {
      builder.push_str(lin);
    }
  }

  if builder.len() != 0{
    let mut what: Vec<char> = builder.chars().collect();
    what.sort();
    what.dedup();
    ret.push(what.into_iter().collect());
  }
  return ret;
}