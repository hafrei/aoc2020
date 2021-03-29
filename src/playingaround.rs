#[derive(Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}
#[derive(Clone)]
struct Instruction {
    direction: Direction,
    distance: i32,
}

impl Instruction {
    fn new(direction: Direction, distance: i32) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }
}

impl Direction {
    fn as_vector(&self) -> (i32, i32) {
        match self {
            N => (0, -1),
            E => (1, 0),
            S => (0, 1),
            W => (-1, 0),
        }
    }
}

pub fn what_else() {
    let test_pt = Point::new(0, 0);
    let dir = N;
    let inst = Instruction::new(dir, distance);

}

pub fn entry_point(working: Vec<i32>) {
    not_silly(working.clone());
    let recursed = oh_no(working.clone());
    println!("Recusive sum for vector was {}", recursed);
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
    println!(
        "{} lived at {}, now for_really is {}",
        claim,
        lives_at,
        for_really.len()
    );
}

fn oh_no(lol: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut hahaha = lol.clone();

    if !lol.is_empty() {
        ret += hahaha.remove(0);
        ret += oh_no(hahaha);
    }

    ret
}

pub fn fizz_buzz() {
    //On 3 print fizz, on 5 print buzz, on 15 print fizzbuzz

    for x in 0..=100 {
        let bluh;
        let printer: &str = match x {
            x if x % 3 == 0 && x % 5 == 0 => "Fizzbuzz",
            x if x % 3 == 0 => "Fizz",
            x if x % 5 == 0 => "Buzz",
            _ => {
                bluh = x.to_string();
                &bluh
            }
        };
        println!("{}", printer);
    }
}
