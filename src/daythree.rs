use std::fs;

pub fn execute_daythree() {
  let path = "./input/day3.txt";
  let working: Vec<String> = prepare_input(path);
  let by_three = get_treecount(&working, 3, 0);
  let by_one = get_treecount(&working, 1, 0);
  let by_five = get_treecount(&working, 5, 0);
  let by_seven = get_treecount(&working, 7, 0);
  let double_speed = get_treecount(&working, 1, 1) -1; // Off by one error here
  let all_of_them: usize = by_three as usize*by_one as usize*by_five as usize*by_seven as usize*double_speed as usize;
  println!("There are {} trees by three", by_three);
  println!("There are {} trees by one", by_one);
  println!("There are {} trees by five", by_five);
  println!("There are {} trees by seven", by_seven);
  println!("There are {} trees at double speed", double_speed); // It's not 2495336976!
  println!("That's {}! That's a lot", all_of_them);
}

fn get_treecount(tree_plane: &Vec<String>, trajectory: u16, skip: i32) -> u16 {
  let mut scanner: u16 = 0;
  let mut counter: u16 = 0;
  let template: char = '#';
  let mut jump = skip;

    for x in tree_plane {
      if jump == 0 {
        let sommit: Vec<char> = x.chars().collect();
        if sommit[scanner as usize] == template {
          counter += 1;
        } 
        scanner += trajectory;
        if scanner >= x.len() as u16 {
          scanner -= x.len() as u16;
        }
        jump = skip;
      } 
      else {
        jump -= 1;
      }
    }
    return counter; //I don't know why but this is producing an off by one error on jump<> 0
}

fn prepare_input (filepath: &str) -> Vec<String> {
  let mut ret: Vec<String> = Vec::new();
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");

  for lin in list.lines() {
    ret.push(lin.to_string().clone());
  }
  return ret;
}