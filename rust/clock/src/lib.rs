// current implementation requires nightly rust
// cargo +nightly test
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const HOURS_PER_DAY: i32 = 24;
const MINS_PER_HOUR: i32 = 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let trunc_mins = minutes.wrapping_rem_euclid(MINS_PER_HOUR);
        let total_hrs = minutes.div_euclid(MINS_PER_HOUR) + hours;
        let trunc_hrs = total_hrs.wrapping_rem_euclid(HOURS_PER_DAY);
        Clock {
            hours: trunc_hrs,
            minutes: trunc_mins,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
