use std::fs;

pub fn execute_dayeleven(){
  let path = "./input/day11.txt";
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

fn find_stable_occupied(layout: Vec<String>) -> i32 {
  let mut active_layout = layout.clone();
  let mut occupied = 0;
  let mut iterated = 0;

  loop {
    let new_layout = active_layout.clone();
    let prev_occ = 0;

    for (ina, r) in active_layout.clone().iter().enumerate() {
      for (inb, c) in r.char_indices() {
        match c {
          '.' => println!("Floor"),
          'L' => println!("Open"),
          '#' => println!("Occupied"),
          _ => println!("Not sure what happened here")
        }
      }
    }

    if prev_occ.eq(&occupied) {
      break;
    }
  }
  occupied
}


fn prepare_input (filepath: &str) -> Vec<String> {
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");
  let ret= list
                      .as_str()
                      .split('\n')
                      .map(str::parse::<String>)
                      .map(Result::unwrap)
                      .collect();

    
  ret
}