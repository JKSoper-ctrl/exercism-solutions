use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{ hours, minutes }.correct_overflow()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }.correct_overflow()
    }

    pub fn correct_overflow(&self) -> Self {
        let mut total_minutes = self.hours * 60 + self.minutes;
        let one_day_in_minutes = 24 * 60;
        
        total_minutes %= one_day_in_minutes;

        if total_minutes < 0 {
            total_minutes += one_day_in_minutes;
        }
        
        Clock{
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}