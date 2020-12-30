use std::fs;

pub fn execute_dayone() {
    //344ms
    let path = "./input/day1.txt";
    let base: Vec<i32> = read_and_parse_file(path);
    let pair: Vec<i32> = get_pair(base.clone());
    let triple: Vec<i32> = get_triple(base);

    println!("Pooped out a {} for pair", pair_multi(pair[0], pair[1]));
    println!(
        "Pooped out a {} for multi",
        trip_multi(triple[0], triple[1], triple[2])
    );
}

fn get_triple(worklist: Vec<i32>) -> Vec<i32> {
    let mut breaker = false;
    let working: Vec<i32> = worklist.clone();
    let mut the_triple: Vec<i32> = Vec::new();

    for x in worklist {
        for y in &working {
            let target: i32 = 2020 - x - y;

            if working.contains(&target) {
                the_triple.push(x);
                the_triple.push(*y);
                the_triple.push(target);
                breaker = true;
                break;
            }
        }
        if breaker {
            break;
        }
    }

    the_triple
}

fn get_pair(worklist: Vec<i32>) -> Vec<i32> {
    let working: Vec<i32> = worklist.clone();
    let mut the_pair: Vec<i32> = Vec::new();

    for x in worklist {
        let target = 2020 - x;
        if working.contains(&target) {
            the_pair.push(x);
            the_pair.push(target);
            break;
        };
    }
    the_pair
}

fn read_and_parse_file(filepath: &str) -> Vec<i32> {
    let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");
    let ret = list
        .as_str()
        .split('\n')
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect();
    ret
}

fn pair_multi(x: i32, y: i32) -> i32 {
    x * y
}

fn trip_multi(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}
