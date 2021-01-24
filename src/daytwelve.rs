use std::fs;

//struct with the instruction: the direction and the action
// then implement the move on N,S,E,W; rotate on L,R; move N,S,E,W on F
// the problem with direction facing is that the byte values aren't 1:1 mapped to compass angles
// So N = 0, E = 90, S = 180, W = 270 rotating right
//    N = 0, W = 90, S = 180, E = 270 rotating left
// but that's if you're facing N, is it not?

struct Ship {
    x: i32,
    y: i32,
    facing: u8,
}

struct Instruction {
    direction: u8, //This is a char
    distance: i32,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            facing: b'E', //this is problematic
        }
    }
    //Oh my god I love this. RECURSION! YEAH! :D
    fn move_direction(self: &mut Self, inst: Instruction) {
        match inst.direction {
            b'N' => self.x += inst.distance,
            b'S' => self.x -= inst.distance,
            b'E' => self.y += inst.distance,
            b'W' => self.y -= inst.distance,
            b'L' => self.rotate(inst.distance),
            b'R' => self.rotate(inst.distance),
            b'F' => self.move_direction(Instruction::new(self.facing, inst.distance)),
            _ => panic!("Hoh baby, something went wrong in move_direciton"),
        }
    }
    fn rotate(self: &mut Self, rotation: i32) {
        match rotation {
            _ => {}
        }
    }
}

impl Instruction {
    fn new(direction: u8, distance: i32) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

pub fn execute_daytwelve() -> i32 {
    let path = "./input/day12test.txt";
    let working = prepare_input(path);
    let destination = process_movement(working);
    destination.x + destination.y
}

fn process_movement(instuct: Vec<Instruction>) -> Ship {
    let mut boat = Ship::new();
    for x in instuct {
        println!(" Go {} for {}", x.direction, x.distance);
        boat.move_direction(x);
    }
    println!("Arrived at {},{}", boat.x, boat.y);
    boat
}

fn prepare_input(filepath: &str) -> Vec<Instruction> {
    let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");
    let ret: Vec<Instruction> = list
        .as_str()
        .split('\n')
        .map(|x| {
            let mut dist = String::from(x);
            let direc = dist.split_off(1);
            Instruction::new(dist.bytes().next().unwrap(), direc.parse::<i32>().unwrap())
        })
        .collect();
    ret
}
