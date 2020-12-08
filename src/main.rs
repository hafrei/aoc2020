mod dayone;
mod daytwo;
mod daythree;
mod dayfour;
mod dayfive;
mod daysix;
mod dayseven;
mod dayeight;
use {
    dayone::execute_dayone,
    daytwo::execute_daytwo,
    daythree::execute_daythree,
    dayfour::execute_dayfour,
    dayfive::execute_dayfive,
    daysix::execute_daysix,
    dayseven::execute_dayseven,
    dayeight::execute_day_eight
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
    execute_day_eight();
    runtime(start);
}

fn runtime(start: SystemTime) {
    let new_sys_time = SystemTime::now();
    let difference = new_sys_time.duration_since(start)
    .expect("Clock may have gone backwards");
    println!("Runtime was {:?}; resetting timer.\n", difference);
}