pub fn entry_point(working: Vec<i32>){
  not_silly(working.clone());
  oh_no(working.clone());
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