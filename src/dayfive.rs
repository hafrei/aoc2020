use std::fs;

pub fn execute_dayfive() {
    let path = "./input/day5.txt";
    let boarding_passes = prepare_input(path);
    let coords: Vec<(i32, i32)> = get_coords(boarding_passes);
    let (the_greatest, seat) = get_greatest(coords);
    println!("Greatest seat should be {}", the_greatest);
    println!("Your seat ID is {}", seat);
}

fn get_greatest(coords: Vec<(i32, i32)>) -> (i32, i32) {
    let mut ids: Vec<i32> = Vec::new();
    let mut the_greatest = 0;
    let mut your_seat = 0;

    for (c, d) in coords {
        let x = c * 8 + d;
        if x > the_greatest {
            the_greatest = x;
        }
        ids.push(x);
    }

    ids.sort_unstable();
    let mut starter = ids[0];

    for id in ids {
        if id != starter {
            your_seat = starter;
            break;
        }
        starter += 1;
    }

    (the_greatest, your_seat)
}

fn get_coords(passes: Vec<String>) -> Vec<(i32, i32)> {
    let mut ret: Vec<(i32, i32)> = Vec::new();
    for c in passes {
        let mut max_row = 127;
        let mut max_col = 7;
        let mut mid_row;
        let mut mid_col;
        let mut min_row = 0;
        let mut min_col = 0;
        let mut ret_row = 0;
        let mut ret_col = 0;
        for i in c.clone().bytes() {
            if i == 70 {
                // F lower row
                mid_row = (max_row - min_row) / 2;
                max_row = max_row - mid_row - (mid_row % 2);
                ret_row = min_row;
            } else if i == 66 {
                //B upper row
                mid_row = (max_row - min_row) / 2;
                min_row = max_row - mid_row;
                ret_row = max_row;
            } else if i == 76 {
                // L lower col
                mid_col = (max_col - min_col) / 2;
                max_col = max_col - mid_col - (mid_col % 2);
                ret_col = min_col;
            } else if i == 82 {
                //R upper col
                mid_col = (max_col - min_col) / 2;
                min_col = max_col - mid_col;
                ret_col = max_col;
            }
        }
        ret.push((ret_row, ret_col));
    }
    ret
}

fn prepare_input(filepath: &str) -> Vec<String> {
    let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");
    let ret = list
        .as_str()
        .split('\n')
        .map(str::parse::<String>)
        .map(Result::unwrap)
        .collect();
    ret
}
