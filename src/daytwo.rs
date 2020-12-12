use std::fs;

#[derive(Clone)]
struct PwHandle{
    pw_min: u8, //These won't get remotely close to 255
    pw_max: u8,
    c: String, //This should be a char, not a string. Get ready for the hacky workaround!
    pw: String, 
}

pub fn execute_daytwo(){
  let path= "./input/day2.txt";
  let ret: Vec<PwHandle> = prepare_input(path);
  let valid_pws: u16 = process_input(ret.clone());
  println!("{} valid passwords found", valid_pws);
  let actually_valid_pws = process_input_again(ret);
  println!("Actually there are {} valid passwords", actually_valid_pws);
}

fn process_input_again(pws_and_rules: Vec<PwHandle>) -> u16 {
  let mut valid_tally = 0;

  for pw in pws_and_rules {
      let min_op: usize = pw.pw_min as usize - 1; 
      let max_op: usize = pw.pw_max as usize - 1; 
      if (pw.pw.chars().nth(min_op) == pw.c.chars().next()) //HAHA oh wow pw.c.chars().nth(0)
          ^ (pw.pw.chars().nth(max_op)== pw.c.chars().next()) {
          valid_tally += 1;
      }
  }
  valid_tally
}

fn process_input(pws_and_rules: Vec<PwHandle>) -> u16 {
  let mut valid_tally = 0;

  for pw in pws_and_rules {
      let applicable: Vec<&str> = pw.pw.matches(&pw.c).collect();
      if applicable.len() >= pw.pw_min as usize 
        && applicable.len() <= pw.pw_max as usize {
          valid_tally += 1;
      }
  }
  valid_tally
}

fn prepare_input(filepath: &str) -> Vec<PwHandle> {
  let mut ret: Vec<PwHandle> = Vec::new();
  let list = fs::read_to_string(filepath).expect("Well that sure didn't open");
  
  //#LifetimesAbuse
  for lin in list.lines() {
      let v: Vec<&str> = lin.split(':').collect();
      let left_side: Vec<&str> = v[0].split(' ').collect();
      let specs: Vec<&str> = left_side[0].split('-').collect();
      ret.push(PwHandle {
          pw_max: specs[1].parse::<u8>().unwrap(), 
          pw_min: specs[0].parse::<u8>().unwrap(),
          c: left_side[1].to_string().clone(),
          pw: v[1].trim().to_string().clone(),
       } )
  }
  ret
}