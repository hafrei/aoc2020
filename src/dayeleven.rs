use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::LineWriter;


pub fn execute_dayeleven(){
  let path = "./input/day11.txt";
  let working= prepare_input(path);
  let occupied_seats = find_stable_occupied(working); // 2164 is too low
  println!("There are {} occupied seats at stability", occupied_seats);
}

/* 
  . Unoccupied; never changes
   L if not occupied and no adjacent occupied seats
   # occupied. If 4 adjacent seats are occupied, it becomes unoccupied

   Apply these rules until no more changes
   Count the number of occupied seats
*/

fn find_stable_occupied(layout: Vec<Vec<u8>>) -> i32 {
  let mut active_layout = layout; // Used as iterator 2D Vector for each round
  let threshold = 4; // A seat becomes available if 4 or more adjacent seats are occupied
  let mut occupied = 0; // Counts number of occupied seats for this round
  let mut prev_occ = 0; // Holds the number of occupied seats from last round
  let mut total_occupied: i32 = 0; // Return value, recieves sum from active_layout's occupied seat count
  let mut counter = 0; // Used for creating the output file

  loop {
    let mut new_layout = active_layout.clone(); 

    for (ena, r) in active_layout.iter().enumerate() {
      let mut total_available = 0;
      let area_length = active_layout.len() -1;
      let row_length = r.len() -1;

      let olr = match ena { //outer lower range
        0 => 0,
        _ => ena - 1
      };

      let our = match area_length.eq(&ena) { //outer upper range
        true => ena,
        false => ena + 1
      };

      for (enb, c) in r.iter().enumerate() {
        let ta_reset = total_available;
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

        //Check the current row for occupied seats to the right
        if !enb.eq(&0) {
          counted += check_seat(r[ilr], looking_for);
          total_available += 1;
        }
        // and then to the left
        if !enb.eq(&row_length) {
          counted += check_seat(r[iur], looking_for);
          total_available += 1;
        }

        // Check the row above for occupied seats
        if !ena.eq(&0) {
          let upper_row = &active_layout[olr][ilr..=iur];
          total_available += upper_row.len();
          counted += &upper_row.iter()
                        .map(|x| check_seat(*x, looking_for))
                        .sum();
        }
        // Check the row below for occupied seats
        if !ena.eq(&row_length) {
          let lower_row = &active_layout[our][ilr..=iur];
          total_available += lower_row.len();
          counted += &lower_row.iter()
                        .map(|x| check_seat(*x, looking_for))
                        .sum();
        }
        // If we are currently counting up open seats and enough are vacant, sit down
        //If we are currently counting up occupied seats and enough are full
        if looking_for.eq(&1) && counted.eq(&(total_available as i32)) {
          new_layout[ena][enb] = b'#';
          occupied +=1;
        } else if looking_for.eq(&2) && counted.ge(&threshold) {
          new_layout[ena][enb] = b'L';
          occupied -=1;
        }

        total_available = ta_reset;
      }
    }
    // Since we are no longer iterating over active layout we can update it with the changes
    write_iter(new_layout.clone(), counter);
    counter += 1; // Since we'll be writing every iteration
    active_layout = new_layout;
    //Finally, if there have been no changes from this round and last round
    // then count the number of occupied seats and break out of the loop
    if prev_occ.eq(&occupied) { //This no longer works properly
      //println!("{:?}", active_layout);
      total_occupied += active_layout.iter()
                              .flatten()
                              .map(|x| if x.eq(&b'#') {1} else {0})
                              .sum::<i32>();
      break;
    } else {
      prev_occ = occupied;
      //occupied = 0; Occupied now decrements so I shouldn't need this anymore
    }
  }
  total_occupied
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

fn write_iter(cm: Vec<Vec<u8>>, iter: usize) -> Result<(), Error> {
  let mut iterwrite = "./output/day11_".to_string();
  iterwrite.push_str(iter.to_string().as_str());
  iterwrite.push_str(".txt");

  let file = File::create(iterwrite)?;
  let mut buffer = LineWriter::new(file);

  for c in cm {
    buffer.write_all(&c);
    buffer.write_all(b"\n");
  }
  buffer.flush();
  Ok(())
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