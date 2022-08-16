use std::fmt::Display;

const HOURS_IN_A_DAY: i32 = 24;
const MINUTES_IN_A_HOUR: i32 = 60;
const MINUTES_IN_A_DAY: i32 = HOURS_IN_A_DAY * MINUTES_IN_A_HOUR;

#[derive(Eq, PartialOrd, Ord, PartialEq, Debug)]
pub struct Clock(i32);

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02}:{:02}", self.0 / 60, self.0 % 60))
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = (hours * MINUTES_IN_A_HOUR + minutes).rem_euclid(MINUTES_IN_A_DAY);
        Clock(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock((self.0 + minutes).rem_euclid(MINUTES_IN_A_DAY))
    }
}
