use std::fs;

//struct with the instruction: the direction and the action
// then implement the move on N,S,E,W; rotate on L,R; move N,S,E,W on F
// the problem with direction facing is that the byte values aren't 1:1 mapped to compass angles
// So N = 0, E = 90, S = 180, W = 270 rotating right
//    N = 0, W = 90, S = 180, E = 270 rotating left
// but that's if you're facing N, is it not?

enum Direction {
    N,
    S,
    E,
    W,
}

struct Waypoint {
    x: i32, //Positive X = E, Negative X = W
    y: i32, //Positive Y = N, Negative Y = S
}

struct Ship {
    x: i32,
    y: i32,
    facing: u8,
}

#[derive(Clone)]
struct Instruction {
    direction: u8, //This is a char
    distance: i32,
}

impl Instruction {
    fn new(direction: u8, distance: i32) -> Self {
        Self {
            direction,
            distance,
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

impl Waypoint {
    fn new() -> Self {
        Self {
            x: 10, //Positive X = E, Negative X = W
            y: 1,  //Positive Y = N, Negative Y = S
        }
    }
    fn rotate_left(&mut self, degree: i32, ship_facing: &mut u8) {
        let adjust_to = degree / 90;
        match ship_facing {
            // N (-x, y) -> W (-x, -y) -> S (x, -y) -> E (x, y)
            b'N' => match adjust_to {
                // I may need to specifically use
                1 => {
                    let ty = self.y;
                    self.y = -self.x;
                    self.x = -ty;
                    *ship_facing = b'W';
                }
                2 => {
                    self.x = self.x.abs();
                    self.y = -self.y;
                    *ship_facing = b'S';
                }
                3 => {
                    let ty = self.y;
                    self.y = self.x.abs();
                    self.x = ty;
                    *ship_facing = b'E';
                }
                _ => {}
            },
            b'S' => match adjust_to {
                1 => {
                    let tx = self.x;
                    let ty = self.y;
                    self.y = tx.abs();
                    self.x = ty.abs();
                    *ship_facing = b'E';
                }
                2 => {
                    self.y = self.y.abs();
                    self.x = -self.x;
                    *ship_facing = b'N';
                }
                3 => {
                    let tx = self.x;
                    let ty = self.y;
                    self.y = -tx;
                    self.x = -ty;
                    *ship_facing = b'W';
                }
                _ => {}
            },
            b'E' => match adjust_to {
                1 => {
                    //turn left 90 degrees
                    let tx = self.x;
                    let ty = self.y;
                    self.y = tx.abs();
                    self.x = -ty;
                    *ship_facing = b'N';
                }
                2 => {
                    self.x = -self.x;
                    self.y = -self.y;
                    *ship_facing = b'W';
                }
                3 => {
                    let tx = self.x;
                    let ty = self.y;
                    self.y = -tx;
                    self.x = ty.abs();
                    *ship_facing = b'S';
                }
                _ => {}
            },
            b'W' => match adjust_to {
                1 => {
                    let tx = self.x;
                    self.x = self.y.abs();
                    self.y = -tx;
                    *ship_facing = b'S';
                }
                2 => {
                    self.x = self.x.abs();
                    self.y = self.y.abs();
                    *ship_facing = b'E';
                }
                3 => {
                    let tx = self.x;
                    self.x = -self.y;
                    self.y = tx.abs();
                    *ship_facing = b'N';
                }
                _ => {}
            },
            _ => {}
        }
    }
    fn rotate_right(&mut self, degree: i32, ship_facing: &mut u8) {
        let adjust_to = degree / 90;
        match ship_facing {
            b'N' => match adjust_to {
                1 => {
                    let ty = self.y;
                    self.y = self.x.abs();
                    self.x = ty;
                    *ship_facing = b'E';
                }
                2 => {
                    self.x = self.x.abs();
                    self.y = -self.y;
                    *ship_facing = b'S';
                }
                3 => {
                    let ty = self.y;
                    self.y = -self.x;
                    self.x = -ty;
                    *ship_facing = b'W';
                }
                _ => {}
            },
            b'S' => match adjust_to {
                1 => {
                    let tx = self.x;
                    let ty = self.y;
                    self.y = -tx;
                    self.x = -ty;
                    *ship_facing = b'W';
                }
                2 => {
                    self.y = self.y.abs();
                    self.x = -self.x;
                    *ship_facing = b'N';
                }
                3 => {
                    let tx = self.x;
                    let ty = self.y;
                    self.y = tx.abs();
                    self.x = ty.abs();
                    *ship_facing = b'E';
                }
                _ => {}
            },
            b'E' => match adjust_to {
                1 => {
                    //turn right 90 degrees
                    let tx = self.x;
                    let ty = self.y;
                    self.y = -tx;
                    self.x = ty.abs();
                    *ship_facing = b'S';
                }
                2 => {
                    self.x = -self.x;
                    self.y = -self.y;
                    *ship_facing = b'W';
                }
                3 => {
                    let tx = self.x;
                    let ty = self.y;
                    self.y = tx.abs();
                    self.x = -ty;
                    *ship_facing = b'N';
                }
                _ => {}
            },
            b'W' => match adjust_to {
                1 => {
                    let tx = self.x;
                    self.x = -self.y;
                    self.y = tx.abs();
                    *ship_facing = b'N';
                }
                2 => {
                    self.x = self.x.abs();
                    self.y = self.y.abs();
                    *ship_facing = b'E';
                }
                3 => {
                    let tx = self.x;
                    self.x = self.y.abs();
                    self.y = -tx;
                    *ship_facing = b'S';
                }
                _ => {}
            },
            _ => {}
        }
    }
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            facing: b'E',
        }
    }

    fn move_to_waypoint(&mut self, wp: &mut Waypoint, inst: Instruction) {
        match inst.direction {
            b'N' => wp.y += inst.distance,
            b'S' => wp.y -= inst.distance,
            b'E' => wp.x += inst.distance,
            b'W' => wp.x -= inst.distance,
            b'L' => wp.rotate_left(inst.distance, &mut self.facing),
            b'R' => wp.rotate_right(inst.distance, &mut self.facing),
            b'F' => {
                for _x in 0..inst.distance {
                    if wp.y > 0 {
                        self.move_direction(Instruction::new(b'N', wp.y.abs()))
                    } else if wp.y < 0 {
                        self.move_direction(Instruction::new(b'S', wp.y.abs()))
                    }
                    if wp.x > 0 {
                        self.move_direction(Instruction::new(b'E', wp.x.abs()))
                    } else if wp.x < 0 {
                        self.move_direction(Instruction::new(b'W', wp.x.abs()))
                    }
                }
            }
            _ => panic!("Hoh baby, something went wrong in move_to_waypoint"),
        }
    }

    //Oh my god I love this. RECURSION! YEAH! :D
    fn move_direction(&mut self, inst: Instruction) {
        match inst.direction {
            b'N' => self.y += inst.distance,
            b'S' => self.y -= inst.distance,
            b'E' => self.x += inst.distance,
            b'W' => self.x -= inst.distance,
            b'L' => self.rotate_left(inst.distance),
            b'R' => self.rotate_right(inst.distance),
            b'F' => self.move_direction(Instruction::new(self.facing, inst.distance)),
            _ => panic!("Hoh baby, something went wrong in move_direciton"),
        }
    }
    fn rotate_left(&mut self, degree: i32) {
        let adjust_to = degree / 90;
        match self.facing {
            b'N' => match adjust_to {
                1 => self.facing = b'W',
                2 => self.facing = b'S',
                3 => self.facing = b'E',
                _ => {}
            },
            b'S' => match adjust_to {
                1 => self.facing = b'E',
                2 => self.facing = b'N',
                3 => self.facing = b'W',
                _ => {}
            },
            b'E' => match adjust_to {
                1 => self.facing = b'N',
                2 => self.facing = b'W',
                3 => self.facing = b'S',
                _ => {}
            },
            b'W' => match adjust_to {
                1 => self.facing = b'S',
                2 => self.facing = b'E',
                3 => self.facing = b'N',
                _ => {}
            },
            _ => {}
        }
    }
    fn rotate_right(&mut self, degree: i32) {
        let adjust_to = degree / 90;
        match self.facing {
            b'N' => match adjust_to {
                1 => self.facing = b'E',
                2 => self.facing = b'S',
                3 => self.facing = b'W',
                _ => {}
            },
            b'S' => match adjust_to {
                1 => self.facing = b'W',
                2 => self.facing = b'N',
                3 => self.facing = b'E',
                _ => {}
            },
            b'E' => match adjust_to {
                1 => self.facing = b'S',
                2 => self.facing = b'W',
                3 => self.facing = b'N',
                _ => {}
            },
            b'W' => match adjust_to {
                1 => self.facing = b'N',
                2 => self.facing = b'E',
                3 => self.facing = b'S',
                _ => {}
            },
            _ => {}
        }
    }
}

// Entry point; 88531 is too high
pub fn execute_daytwelve() -> (i32, i32) {
    let path = "./input/day12.txt";
    let working = prepare_input(path);
    let destination = process_part1(working.clone());
    let manhattan_dist = destination.x.abs() + destination.y.abs();
    println!("Manhattan distance is {}", manhattan_dist);
    let waypoint_dist = process_part2(working);
    let taxicab_waypoint = waypoint_dist.x.abs() + waypoint_dist.y.abs();
    println!("Manhattan distance using waypoints is {}", taxicab_waypoint);
    (manhattan_dist, taxicab_waypoint)
}

fn process_part2(instuct: Vec<Instruction>) -> Ship {
    let mut boat = Ship::new();
    let mut waypoint = Waypoint::new();
    for x in instuct {
        boat.move_to_waypoint(&mut waypoint, x);
    }
    boat
}

fn process_part1(instuct: Vec<Instruction>) -> Ship {
    let mut boat = Ship::new();
    for x in instuct {
        boat.move_direction(x);
    }
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
