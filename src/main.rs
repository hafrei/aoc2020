mod dayone;
mod daytwo;
mod daythree;
mod dayfour;
mod dayfive;
mod daysix;
mod dayseven;
mod dayeight;
mod daynine;
mod dayten;
mod dayeleven;
use {
    dayone::execute_dayone,
    daytwo::execute_daytwo,
    daythree::execute_daythree,
    dayfour::execute_dayfour,
    dayfive::execute_dayfive,
    daysix::execute_daysix,
    dayseven::execute_dayseven,
    dayeight::execute_dayeight,
    daynine::execute_daynine,
    dayten::execute_dayten,
    dayeleven::execute_dayeleven
    };
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    execute_dayone();
    runtime(start);
    let start = SystemTime::now();
    execute_daytwo();
    runtime(start);
    let start = SystemTime::now();
    execute_daythree();
    runtime(start);
    let start = SystemTime::now();
    execute_dayfour();
    runtime(start);
    let start = SystemTime::now();
    execute_dayfive();
    runtime(start);
    let start = SystemTime::now();
    execute_daysix();
    runtime(start);
    let start = SystemTime::now();
    execute_dayseven();
    runtime(start);
    let start = SystemTime::now();
    execute_dayeight();
    runtime(start);
    let start = SystemTime::now();
    execute_daynine();
    runtime(start);
    let start = SystemTime::now();
    execute_dayten();
    runtime(start);
    let start = SystemTime::now();
    execute_dayeleven();
    runtime(start);
}

fn runtime(start: SystemTime) {
    let new_sys_time = SystemTime::now();
    let difference = new_sys_time.duration_since(start)
    .expect("Clock may have gone backwards");
    println!("Runtime was {:?}; resetting timer.\n", difference);
}