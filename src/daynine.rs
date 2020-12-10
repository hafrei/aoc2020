use std::fs;

#[derive(Clone)]
struct XmasStream {
  value: i64,
  origi: usize,
  validated: i32,
}

impl XmasStream {
  pub fn new(value: i64, origi: usize) -> Self {
    Self {
      value,
      origi,
      validated: 0,
    }
  }
}

pub fn execute_daynine() { 
  let path = "./input/day9.txt";
  let preamble: usize = 25;
  let working = prepare_input(path);
  let weak_point = find_weakness(working.clone(), preamble);
  println!("Use {} to attack for massive damage!", weak_point);
  let exploit_sum = exploit_weakness(working, preamble, weak_point);
  println!("The end is at {}", exploit_sum);
}

fn exploit_weakness(working: Vec<XmasStream>, preamble: usize, weak_point: i64) -> i64 {
  let mut exploit_sum = 0;
  let mut preamble_start: usize = 0;
  let mut preamble_end: usize = preamble;
  let mut _w_copy = working;
  let mut _p_work: Vec<XmasStream> = Vec::new();

  while exploit_sum == 0 {
    _p_work = _w_copy[preamble_start..preamble_end].to_vec().clone();

    for i in 0.._p_work.len() {
      let mut weak_combo: Vec<usize> = Vec::new();
      let mut work_sum = _p_work[i].value;
      for j in 0.._p_work.len() {
        if work_sum < weak_point {
          work_sum += _p_work[j].value;
          weak_combo.push(_p_work[j].value as usize);
        } else if work_sum > weak_point {
          work_sum = 0;
          weak_combo = Vec::new();
          break;
        } else if work_sum == weak_point {
          weak_combo.sort();
          exploit_sum = weak_combo[0] as i64 + weak_combo[weak_combo.len()-1] as i64;
          break;
        }
      }
      if exploit_sum > 0 || work_sum > weak_point {
        break;
      }
    }
    preamble_start += 1;
    preamble_end += 1;
  }

  return exploit_sum;
}

fn find_weakness(working: Vec<XmasStream>, preamble: usize) -> i64{
  let mut weak_point = 0;
  let mut preamble_start: usize = 0;
  let mut preamble_end: usize = preamble;
  let mut w_copy = working;
  let mut _p_work: Vec<XmasStream> = Vec::new();
  let mut _target: i64 = 0;

  while weak_point == 0 {
    _p_work = w_copy[preamble_start..preamble_end].to_vec().clone();
    let target_index = preamble_end;
    _target = w_copy[target_index].value;

    for i in 0.._p_work.len() {
      for j in 0.._p_work.len() {
        if i != j && _p_work[i].value + _p_work[j].value == _target {
            w_copy[preamble_end+1].validated +=1;
        } 
      }
    }
    if w_copy[preamble_end+1].validated == 0 {
      weak_point = _target;
    }
    /*
     * Hoooh boy, ok
     * You need the preamble and only the VERY NEXT number
     * for all combinations of preamble numbers
     *  count the total times summed (if you want but you don't need to)
     *  return all target numbers   
    */

    preamble_start += 1;
    preamble_end += 1;
  }

  return weak_point;
}

fn prepare_input (filepath: &str) -> Vec<XmasStream> {
  let mut ret: Vec<XmasStream> = Vec::new();
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");

  for lin in list.lines() {
    ret.push(XmasStream::new(lin.parse::<i64>().unwrap(), ret.len()));
  }
  return ret;
}