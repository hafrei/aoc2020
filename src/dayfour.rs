use std::fs;
use regex::Regex;

pub fn execute_dayfour() {
  let path = "./input/day4.txt";
  let working: Vec<String> = prepare_input(path);
  let passports = build_passenger_info(&working);
  let working_passports = build_passports(&passports);
  let valid = validate_passports(working_passports);

  println!("{} passports validated", valid); 
}

fn validate_passports(passports: Vec<Vec<(String, String)>>) -> i32 {
  let mut valid = 0;
  let byr = "byr";
  let iyr = "iyr";
  let eyr = "eyr";
  let hgt = "hgt";
  let hcl = "hcl";
  let ecl = "ecl";
  let pid = "pid";

  let year_re = Regex::new(r"(19[2-8][0-9]|199[0-9]|200[0-2])").unwrap();
  let iyear_re = Regex::new(r"(201[0-9]|2020)").unwrap();
  let eyear_re = Regex::new(r"(202[0-9]|2030)").unwrap();
  let hgt_re = Regex::new(r"(59in|6[0-9]in|7[0-6]in)|(1[5-8][0-9]cm|19[0-3]cm)").unwrap();
  let hcl_re = Regex::new(r"^(#[0-9a-f]{6})$").unwrap();
  let ecl_re = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
  let pass_re = Regex::new(r"^(\d{9})$").unwrap(); //Stupid anchors

  for pp in passports {
    let mut entries = 0;
    let scanner: Vec<(String, String)> = pp;
    for (piece1, piece2) in scanner.clone() {
      if (piece1.as_str().eq(byr) && year_re.is_match(piece2.as_str()))
      || (piece1.as_str().eq(iyr) && iyear_re.is_match(piece2.as_str()))
      || (piece1.as_str().eq(eyr) && eyear_re.is_match(piece2.as_str()))
      || (piece1.as_str().eq(hgt) && hgt_re.is_match(piece2.as_str()))
      || (piece1.as_str().eq(hcl) && hcl_re.is_match(piece2.as_str()))
      || (piece1.as_str().eq(ecl) && ecl_re.is_match(piece2.as_str()))
      || (piece1.as_str().eq(pid) && pass_re.is_match(piece2.as_str()))
      {
        entries += 1;
      }
    }
    if entries >= 7 {
      valid += 1;
    }
  }
  valid
}

fn build_passports(passports: &[String]) -> Vec<Vec<(String, String)>> {
  let mut actually: Vec<Vec<(String, String)>> = Vec::new();

  for x in passports {
    if x.is_empty(){
      break;
    }
    let mut working_passports: Vec<(String, String)> = Vec::new();
    let mut pp_fields: Vec<String> = x.clone().split(' ').map(String::from).collect();
    pp_fields.sort();
    for y in pp_fields {
      let left_side: Vec<String> = y.clone().split(':').map(String::from).collect();
      working_passports.push((left_side[0].clone(), left_side[1].clone()));
    }
    actually.push(working_passports.clone());
  }

  actually
}

fn build_passenger_info(input: &[String]) -> Vec<String> {
  let mut builder: Vec<String> = Vec::new();
  let mut validate: String = String::new();

  for x in input{
    if x.is_empty(){
      builder.push(validate.trim().to_string());
      validate = String::new();
    } else {
      validate.push(' ');
      validate.push_str(&x);
    }
  }

  if validate.is_empty() {
    builder.push(validate.trim().to_string());
  }
  builder
}

fn prepare_input (filepath: &str) -> Vec<String> {
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");
  let ret = list
                      .as_str()
                      .split('\n')
                      .map(str::parse::<String>)
                      .map(Result::unwrap)
                      .collect();
  ret
}