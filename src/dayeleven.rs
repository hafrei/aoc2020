use std::fs;

pub fn execute_dayeleven(){
  let path = "./input/day11test.txt";
  let working= prepare_input(path);
  let occupied_seats = find_stable_occupied(working);
  println!("Waiting");
}

/* 
  . Unoccupied; never changes
   L if not occupied and no adjacent occupied seats
   # occupied. If 4 adjacent seats are occupied, it becomes unoccupied

   Apply these rules until no more changes
   Count the number of occupied seats
*/

fn find_stable_occupied(layout: Vec<Vec<u8>>) -> i32 {
  let mut active_layout = layout.clone();
  let mut occupied = 0;
  let mut iterated = 0;

  loop {
    let new_layout = active_layout.clone();
    let prev_occ = 0;

    for (ena, r) in active_layout.clone().iter().enumerate() {
      let row_length = r.len() -1;
      let olr = match ena { //inner lower range
        0 => 0,
        _ => ena - 1
      };

      let our = match row_length.eq(&ena) { //inner upper range
        true => ena,
        false => ena + 1
      };

      for (enb, c) in r.iter().enumerate() {
        let mut counted = 0;
        let mut looking_for = 0; //0 is value idk, 1 is value of open, 2 is the value of occupied
        // what is current seat?
        looking_for = check_seat(*c, looking_for) as u8;

        if looking_for.eq(&3) {
          continue;
        }

        let ilr = match enb { //inner lower range
          0 => 0,
          _ => enb - 1
        };

        let iur = match row_length.eq(&enb) { //inner upper range
          true => enb,
          false => enb + 1
        };

        // Check the current row for occupied seats
        if !enb.eq(&0) {
          counted += check_seat(r[ilr], looking_for);
        }

        if !enb.eq(&row_length) {
          counted += check_seat(r[iur], looking_for);
        }

        // Check the row above for occupied seats
        if !ena.eq(&0) {
          let upper_row = &active_layout[olr][ilr..=iur];
          counted += &upper_row.iter()
                        .map(|x| check_seat(*x, looking_for))
                        .sum();
        }
        // Check the row below for occupied seats
        if !ena.eq(&row_length) {
          let upper_row = &active_layout[our][ilr..=iur];
          counted += &upper_row.iter()
                        .map(|x| check_seat(*x, looking_for))
                        .sum();
        }
        println!("{}", counted);
      }
    }

    if prev_occ.eq(&occupied) {
      break;
    }
  }
  occupied
}
/**
 * Returns 1 if seat matches L, #, or other
 * l_f = 0 will return 1 for L, 2 for #, and 3 for other
 * l_f = 1 means L will return 1
 * l_f = 2 means # will return 1
 * All other char will return 0
*/
fn check_seat(seat: u8, l_f: u8) -> i32 {
  let mut ret = 0;
  if l_f.eq(&0) {
    ret = match seat {
      b'L' => 1,
      b'#' => 2,
      _ => 3 //Should match open floor, which we don't care about
    };
  } else if l_f.eq(&1){
    ret = match seat {
      b'L' => 1,
      b'#' => 0,
      _ => 1 // Hallways are always open
    };
  } else if l_f.eq(&2){
    ret = match seat {
      b'L' => 0,
      b'#' => 1,
      _ => 0 //Hallways are never occupied
    }
  } 
  ret
}

/**
 * Will attempt to read the whole file at filepath
 * Transforms the file at filepath into a 2d char vector
 */
fn prepare_input (filepath: &str) -> Vec<Vec<u8>> {
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");
  let ret: Vec<Vec<u8>>= list
                      .as_str()
                      .split('\n')
                      .map(|x| x.as_bytes().to_vec()) //OH MY GOD IT WORKED
                      .collect();
  ret
}