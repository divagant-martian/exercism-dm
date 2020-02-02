use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // real number of minutes
        let rm = 60 * hours + minutes;
        Clock {
            hours: rm.div_euclid(60).rem_euclid(24),
            minutes: rm.rem_euclid(60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let m = self.minutes + minutes;
        Clock {
            hours: (60 * self.hours + m).div_euclid(60).rem_euclid(24),
            minutes: m.rem_euclid(60),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:02}:{:02}", self.hours, self.minutes);
        Ok(())
    }
}
