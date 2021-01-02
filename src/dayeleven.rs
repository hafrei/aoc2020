use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, LineWriter};
use std::ops::RangeInclusive;

pub fn execute_dayeleven() { //180 is too low
    let path = "./input/day11.txt";
    let working = prepare_input(path);
    let expected_occupied_seats = find_stable_occupied(working.clone());
    println!(
        "There are {} occupied seats at stability",
        expected_occupied_seats
    );
    let actual_occupied_seats = find_real_stable_occupied(working);
    println!(
        "Nope, I was wrong. There's actually {} occupied seats",
        actual_occupied_seats
    );
}

/*
  . Unoccupied; never changes
   L if not occupied and no visible occupied seats
   # occupied. If 5 seats are occupied in any of the 8 directions, it becomes unoccupied

   Apply these rules until no more changes
   Count the number of occupied seats
*/
fn find_real_stable_occupied(layout: Vec<Vec<u8>>) -> i32 {
    let mut active_layout = layout; // Used as iterator 2D Vector for each round
    let threshold: i32 = 5; // A seat becomes available if 5 or more visible seats are occupied
    let mut occupied = 0; // Counts number of occupied seats for this round
    let mut prev_occ = 0; // Holds the number of occupied seats from last round
    let mut total_occupied: i32 = 0; // Return value, recieves sum from active_layout's occupied seat count
    let mut counter = 1; // Used for creating the output file

    loop {
        let mut new_layout = active_layout.clone();
        //ena = Enumerator A, counter of the whole grid
        for ena in 0..active_layout.len() {
            let mut total_available = 0;
            let area_length = active_layout.len() - 1;

            //enb = enumerator b, Index of current row
            for enb in 0..active_layout[ena].len() {
                let row_length = active_layout[ena].len() - 1;
                let ta_reset = total_available; //Why not make total_available 0 here?
                let mut counted = 0;
                let mut looking_for = 0; //0 is value idk, 1 is value of open, 2 is the value of occupied

                //What is current seat? let's find out what mode we're in
                looking_for = check_seat(active_layout[ena][enb], looking_for) as u8;

                // Don't bother running everything for hallway space
                if looking_for.eq(&3) {
                    continue;
                } else if looking_for.eq(&1) {
                    looking_for = 4; //change modes to explicitly empty only
                }

                flag(&ena, &enb, &counter);

                //Look down the current row for occupied seats to the left
                if enb.ne(&0) {
                    let found_maybe = look_left(enb, &active_layout[ena], looking_for);
                    counted += found_maybe;
                    total_available += 1;
                }
                // and then to the right
                if enb.ne(&row_length) {
                    let found_maybe = look_right(enb, &active_layout[ena], looking_for, row_length);
                    counted += found_maybe;
                    total_available += 1;
                }

                if ena.ne(&0) {
                    //Look directly above current seat
                    let found_maybe =
                        look_diag(enb, &active_layout, looking_for, 0, ena, threshold as u8, 0);
                    counted += found_maybe;
                    total_available += 1;

                    // then above left.
                    if enb.ne(&0) {
                        counted += look_diag(
                            enb - 1,
                            &active_layout,
                            looking_for,
                            0,
                            ena,
                            threshold as u8,
                            2,
                        );
                        total_available += 1;
                    }
                    // then above right
                    if enb.ne(&row_length) {
                        counted += look_diag(
                            enb + 1,
                            &active_layout,
                            looking_for,
                            0,
                            ena,
                            threshold as u8,
                            1,
                        );
                        total_available += 1;
                    }
                }
                // Check the row below (higher number) for occupied seats
                if ena.ne(&area_length) {
                    counted += look_diag(
                        enb,
                        &active_layout,
                        looking_for,
                        ena + 1,
                        area_length,
                        threshold as u8,
                        0,
                    );
                    total_available += 1;

                    // then below left
                    if enb.ne(&0) {
                        counted += look_diag(
                            enb - 1,
                            &active_layout,
                            looking_for,
                            ena + 1,
                            area_length,
                            threshold as u8,
                            2,
                        );
                        total_available += 1;
                    }
                    // then below right
                    if enb.ne(&row_length) {
                        counted += look_diag(
                            enb + 1,
                            &active_layout,
                            looking_for,
                            ena + 1,
                            row_length,
                            threshold as u8,
                            1,
                        );
                        total_available += 1;
                    }
                }

                //If we are currently counting up open seats and enough are vacant, sit down
                //If we are currently counting up occupied seats and enough are full, get up
                //Currently, if we get all the way through checks for empty seats and finding no occupied seats
                // Then we don't count that seat
                if looking_for.eq(&4) && counted.eq(&(total_available as i32)) {
                    new_layout[ena][enb] = b'#';
                    occupied += 1;
                } else if looking_for.eq(&2) && counted.ge(&threshold) {
                    new_layout[ena][enb] = b'L';
                    occupied -= 1;
                }

                total_available = ta_reset;
            }
        }
        active_layout = new_layout;

        //Write current grid to file
        write_iter(active_layout.clone(), counter);
        counter += 1; // Since we'll be writing every iteration

        //Finally, if there have been no changes from this round and last round
        // then count the number of occupied seats and break out of the loop
        if prev_occ.eq(&occupied) {
            total_occupied += active_layout
                .iter()
                .flatten()
                .map(|x| if x.eq(&b'#') { 1 } else { 0 })
                .sum::<i32>();
            break;
        } else {
            prev_occ = occupied;
        }
    }
    total_occupied
}

fn flag(ena: &usize, enb: &usize, counter: &usize) {
    let row = 0;
    let col = 1;
    let itr: usize = 2;

    if ena.eq(&row) && enb.eq(&col) && itr.eq(&counter) {
        println!("Hooked!");
    }
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

    loop {
        let mut new_layout = active_layout.clone();

        //ena = Enumerator A, counter of the whole grid
        // r = row of grid
        for (ena, r) in active_layout.iter().enumerate() {
            let mut total_available = 0;
            let area_length = active_layout.len() - 1;

            //outer lower range
            let olr = match ena {
                0 => 0,
                _ => ena - 1,
            };

            //outer upper range
            let our = match area_length.eq(&ena) {
                true => ena,
                false => ena + 1,
            };

            //enb = enumerator b, Index of current row
            for (enb, c) in r.iter().enumerate() {
                let row_length = r.len() - 1;
                let ta_reset = total_available;
                let mut counted = 0;
                let mut looking_for = 0; //0 is value idk, 1 is value of open, 2 is the value of occupied

                //What is current space? Let's take a look so we know what we're doing
                looking_for = check_seat(*c, looking_for) as u8;

                // Don't bother running everything for hallway space
                if looking_for.eq(&3) {
                    continue;
                }

                //inner lower range
                let ilr = match enb {
                    0 => 0,
                    _ => enb - 1,
                };

                //inner upper range
                let iur = match row_length.eq(&enb) {
                    true => enb,
                    false => enb + 1,
                };

                //Check the current row for occupied seats to the left
                if enb.ne(&0) {
                    counted += check_seat(r[ilr], looking_for);
                    total_available += 1;
                }
                // and then to the right
                if enb.ne(&row_length) {
                    counted += check_seat(r[iur], looking_for);
                    total_available += 1;
                }

                // Check the row above (lower number) for occupied seats
                if ena.ne(&0) {
                    let upper_row = &active_layout[olr][ilr..=iur];
                    total_available += upper_row.len();
                    counted += &upper_row.iter().map(|x| check_seat(*x, looking_for)).sum();
                }
                // Check the row below (higher number) for occupied seats
                if ena.ne(&area_length) {
                    let lower_row = &active_layout[our][ilr..=iur];
                    total_available += lower_row.len();
                    counted += &lower_row.iter().map(|x| check_seat(*x, looking_for)).sum();
                }

                //If we are currently counting up open seats and enough are vacant, sit down
                //If we are currently counting up occupied seats and enough are full, get up
                if looking_for.eq(&1) && counted.eq(&(total_available as i32)) {
                    new_layout[ena][enb] = b'#';
                    occupied += 1;
                } else if looking_for.eq(&2) && counted.ge(&threshold) {
                    new_layout[ena][enb] = b'L';
                    occupied -= 1;
                }

                total_available = ta_reset;
            }
        }

        // Since we are no longer iterating over active layout we can update it with the changes
        active_layout = new_layout;

        // Finally, if there have been no changes from this round and last round
        // then count the number of occupied seats and break out of the loop
        if prev_occ.eq(&occupied) {
            total_occupied += active_layout
                .iter()
                .flatten()
                .map(|x| if x.eq(&b'#') { 1 } else { 0 })
                .sum::<i32>();
            break;
        } else {
            prev_occ = occupied;
        }
    }
    total_occupied
}

// Find the first occurance of the seat you're looking for on the right
fn look_right(enb: usize, r: &Vec<u8>, looking_for: u8, row_length: usize) -> i32 {
    let look_right = &r[enb + 1..=row_length];
    let look_len = look_right.len();
    let mut ret = 0;

    for (e, lr) in look_right.into_iter().enumerate() {
        if check_seat(*lr, looking_for) == 1 {
            ret = 1;
            break;
        } else if e.eq(&look_len) && looking_for.eq(&4) {
            ret = 1; // we have gone all the way through and found nothing
        }
    }

    ret
}

// Find the first occurance of the seat you're looking for on the left
fn look_left(enb: usize, r: &Vec<u8>, looking_for: u8) -> i32 {
    let mut look_left: Vec<usize> = (0..enb).collect();
    let look_len = look_left.len();
    let mut ret = 0;

    look_left.reverse();
    for ll in look_left {
        if check_seat(r[ll], looking_for) == 1 {
            ret = 1;
            break;
        } else if ll.eq(&look_len) && looking_for.eq(&4) {
            ret = 1; // we have gone all the way through and found nothing
        }
    }

    ret
}

/* ii = inner index, the starting point for the enb adjusting value
   lf = looking_for, used in check_seat
   lb = left bound, the beggining (or end) bound for the for loop
   lb = left bound, the end (or beginning) bound for the for loop
   al = active_layout
   thresh = threshold value, it's used in check_seat so pass it as a u8
*/
fn look_diag(
    ii: usize,
    al: &Vec<Vec<u8>>,
    lf: u8,
    lb: usize,
    rb: usize,
    thresh: u8,
    inc: i32,
) -> i32 {
    let mut dec = false;
    let mut ret = 0;
    let rng = if lb.gt(&rb) {
        dec = true;
        RangeInclusive::new(rb, lb)
    } else {
        RangeInclusive::new(lb, rb)
    };
    let mut angle = ii;
    for i in rng {
        let cur_seat = al[i][angle].clone();
        if check_seat(cur_seat, lf) == 1 {
            ret += 1;
            break;
        }
        // using threshold here (which is 5)
        // because check_seat with an LF of 5 looks for L and #
        if check_seat(cur_seat, thresh) == 1 
        || (dec && i.eq(&0) || angle.eq(&0))
        || (dec == false && i.eq(&(al.len() - 1) ) || angle.eq(&(al.len() - 1))) {
            break;
        }
        if inc.eq(&1) && angle.ne(&lb) && angle.ne(&rb) {
            angle += 1;
        } else if inc.eq(&2) && angle.ne(&0) && angle.ne(&0) {
            angle -= 1;
        }
    }
    ret
}

/**
 * Returns 1 if seat matches L, #, or other
 * l_f = 0 will return 1 for L, 2 for #, and 3 for other
 * l_f = 1 means L and _ will return 1
 * l_f = 2 means # will return 1
 * l_f = 4 is L only
 * l_f = 5 is L or #
 * All other char will return 0
*/
fn check_seat(seat: u8, l_f: u8) -> i32 {
    let mut ret = 0;
    if l_f.eq(&0) {
        ret = match seat {
            b'L' => 1,
            b'#' => 2,
            _ => 3, //Should match open floor, which we don't care about
        };
    } else if l_f.eq(&1) {
        ret = match seat {
            b'L' => 1,
            b'#' => 0,
            _ => 1, // Hallways are always open
        };
    } else if l_f.eq(&2) {
        ret = match seat {
            b'L' => 0,
            b'#' => 1,
            _ => 0, //Hallways are never occupied
        }
    } else if l_f.eq(&4) {
        //I need an explicit open only
        ret = match seat {
            b'L' => 1, //76
            b'#' => 0, //35
            _ => 0, //46
        }
    } else if l_f.eq(&5) {
        //I need only seats
        ret = match seat {
            b'L' => 1,
            b'#' => 1,
            _ => 0,
        }
    }
    ret
}

/**
 * Write the grid to file. Is formatted into expected characters for free!
 * Files are written into ./output as day11_<iter>.txt
 * Currently loaded with warnings because I'm not checking for Errors
 */
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
fn prepare_input(filepath: &str) -> Vec<Vec<u8>> {
    let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");
    let ret: Vec<Vec<u8>> = list
        .as_str()
        .split('\n')
        .map(|x| x.as_bytes().to_vec()) //OH MY GOD IT WORKED
        .collect();
    ret
}
