pub mod dayeight;
pub mod dayeleven;
pub mod dayfive;
pub mod dayfour;
pub mod daynine;
pub mod dayone;
pub mod dayseven;
pub mod daysix;
pub mod dayten;
pub mod daythree;
pub mod daytwelve;
pub mod daytwo;

#[cfg(test)]
mod tests {
    use crate::daytwelve;
    #[test] //This only works if the test file is day12test.txt
    fn test_daytwelve() {
        let (part_one, part_two) = daytwelve::execute_daytwelve();
        assert_eq!(25, part_one);
        assert_eq!(286, part_two);
    }
}
