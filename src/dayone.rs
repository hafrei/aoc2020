use std::fs;

pub fn execute_dayone() {
    //344ms
    let path = "./input/day1.txt";
    let base: Vec<i32> = read_and_parse_file(path);
    let pair: Vec<i32> = get_pair(base.clone());
    let triple: Vec<i32> = get_triple(base);

    println!("Pooped out a {} for pair", pair.iter().product::<i32>());
    println!("Pooped out a {} for multi", triple.iter().product::<i32>());
}

fn get_triple(worklist: Vec<i32>) -> Vec<i32> {
    let working: Vec<i32> = worklist.clone();
    let mut the_triple: Vec<i32> = Vec::new();

    'test: for x in worklist {
        for y in &working {
            let target: i32 = 2020 - x - y;

            if working.contains(&target) {
                the_triple.push(x);
                the_triple.push(*y);
                the_triple.push(target);
                break 'test;
            }
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
