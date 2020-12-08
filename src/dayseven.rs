use std::fs;
use std::collections::HashMap;

pub fn execute_dayseven() {
  let path = "./input/day7test.txt";
  let target_bag = "shiny gold bag".to_string();
  let working = prepare_input(path);
  let structured: HashMap<String, HashMap<String, i32>> = parse_ruleset(working);
  let number_of_larger_bags = find_bigger_bags(structured.clone(), target_bag.clone());
  let number_of_smaller_bags = find_smaller_bags(structured, target_bag.clone());
  println!("Your {:?} will fit in {} other bags", &target_bag, &number_of_larger_bags);
  println!("Your {:?} will fit {} other bags in it", &target_bag, &number_of_smaller_bags);
}

fn find_smaller_bags(bag_map: HashMap<String, HashMap<String, i32>>, target_bag: String) -> i32{

  /*
    Yikes, ok. So we have the whole map with submaps and we have our starting point.
    For each bag, we need to get all the inner bags and their inner bags...

  */

  let mut hooks_in = true;
  let mut focus_bag = target_bag;
  let mut working_list: Vec<String> = Vec::new();
  let mut maybe_maybe = 0;
  let mut count = 0;

  while hooks_in {

    let (host_bag, inner_bags) = bag_map.clone().get_key_value(&focus_bag).unwrap();
    
    for (bag, must_fit) in bag_map.clone() {
      if bag.as_str().eq(focus_bag.as_str()){
        working_list.push(bag.clone());
        maybe_maybe += count;
        println!("{:?}", bag,);
        } 
    }

    if working_list.len() != 0 {
      focus_bag = working_list.pop().unwrap();
    } else {
      hooks_in = false;
    }
  }
  return maybe_maybe;
}

fn find_bigger_bags(bag_map: HashMap<String, HashMap<String, i32>>, target_bag: String) -> i32{
  let mut hooks_in = true;
  let mut focus_bag = target_bag;
  let mut full_list: Vec<String> = Vec::new();
  let mut working_list: Vec<String> = Vec::new();
  while hooks_in {

    for (bag, can_fit) in bag_map.clone() {
      for (ibag, _count) in can_fit {
        if ibag.as_str().eq(focus_bag.as_str()){
          working_list.push(bag.clone());
          full_list.push(bag.clone());
        } 
      }
    }

    if working_list.len() != 0 {
      focus_bag = working_list.pop().unwrap();
    } else {
      hooks_in = false;
    }
  }

  //Can you search by value?
    //if so, get all they keys which list shiny gold bag as value
    //build a list/set for immediately larger
    // build a list/set for everything (that has a len())
    //return that
    full_list.sort();
    full_list.dedup();

    return full_list.len() as i32;
}

fn parse_ruleset(raw_set: Vec<String>) -> HashMap<String, HashMap<String, i32>> {
  //HashMap<ContainingBag, HashMap<InnerBag, count>>
  let mut parsed_set: HashMap<String, HashMap<String, i32>> = HashMap::new();
  //parse and take left of "contain"
  for line in raw_set {
    let mut inners: HashMap<String, i32> = HashMap::new();
    let mut working = line.clone();
    let mut right_side = working.split_off(line.find(" contain ").unwrap());
    working = working.replace("bags", "bag");
    right_side.replace_range(0.." contain ".len(), "");
  // parse right at commas
   let contents: Vec<String> = right_side.split(',').map(|s| s.to_string()).collect();
    for content in contents.clone() {
      //trim and remove the s from the end, if it exists
      let mut con = content.trim().to_string();
      con.retain(|c| c != '.');
      //of these, parse at first space
        //left is number of bags
      let mut desc = con.split_off(con.find(" ").unwrap()).trim().to_string();
      //trim and remove the final character if it's 's' or '.'
      desc = desc.replace("bags", "bag");
      // no is the same as 0 and I want an i32 for later
      if con == "no".to_string() {
        con = "0".to_string();
      }
      inners.insert(desc, con.parse::<i32>().unwrap());
    }
    parsed_set.insert(working, inners);
  }
  //HashMap<ContainingBag, HashMap<InnerBag, count>>
  return parsed_set;
}

fn prepare_input (filepath: &str) -> Vec<String> {
  let mut ret: Vec<String> = Vec::new();
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");

  for lin in list.lines() {
    ret.push(lin.to_string().clone());
  }
  return ret;
}