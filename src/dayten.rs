use std::fs;

pub fn execute_dayten() {
  let path = "./input/day10.txt";
  let working: Vec<i32> = prepare_input(path);
  let eff_you: Vec<i32> = working.clone();
  let (one, three) = kinda_cheeky(eff_you);
  not_silly(working.clone());
  let haha_what = oh_no(working);
  println!("If this isn't 0 then it worked: {}", haha_what);
  println!("Should be {} * {} = {}", one, three, one * three );
}

/*
  Each number can be itself or up to -3 
  Find the item on the list with the highest number and +3
  Start at 0
  At each level, check for all numbers within +3 (filter / drain)
    for each option
 */

 fn kinda_cheeky(worklist: Vec<i32>) -> (i32, i32){ //2170 too low 6300 too high
   let mut workan: Vec<i32> = Vec::new();
   let mut come_on: Vec<i32> = Vec::new();
   workan = worklist;
   workan.sort_unstable();
   let mut one = 0;
   let mut three = 0;

   for (e, &x) in workan.iter().enumerate() {
     come_on = workan.clone();
     let fine: usize = e+1;
     if fine >= workan.len() {
       break;
     }
     let okay: i32 = come_on[fine];
     if okay - x == 1 {
       one +=1;
     } else if okay - x == 2 {
      println!("Sommin");
     }
     else if okay - x == 3{
      three +=1;
     } else {
       println!("a broke?");
     }
   }
   three += 1; //Cause your laptop!

   (one as i32, three as i32)
 }

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

  for_really = really.clone();
 }

fn oh_no(lol: Vec<i32>) -> i32 {
  let mut ret = 0;
  let mut hahaha = lol.clone();

  if lol.is_empty(){
    ret += hahaha.remove(0);
    ret += oh_no(hahaha);
  }

  ret
}

fn prepare_input (filepath: &str) -> Vec<i32> {
  let mut ret: Vec<i32> = Vec::new();
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");

  for lin in list.lines() {
    ret.push(lin.parse::<i32>().unwrap());
  }
  ret
}