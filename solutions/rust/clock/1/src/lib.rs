use core::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_min_in_a_day = 60 * 24;
        let hr_in_min = hours * 60;
        let total_min = hr_in_min + minutes;

        let mod_minutes =
            ((total_min % total_min_in_a_day) + total_min_in_a_day) % total_min_in_a_day;

        let hour = mod_minutes / 60;
        let min = mod_minutes % 60;

        Self {
            hours: hour,
            minutes: min,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hour, min) = self.calc(self.hours, minutes);
        Self {
            hours: hour,
            minutes: min,
        }
    }

    fn calc(&self, hours: i32, minutes: i32) -> (i32, i32) {
        let total_min_in_a_day = 60 * 24;
        let hr_in_min = hours * 60;
        let total_min = hr_in_min + self.minutes + minutes;

        let mod_minutes =
            ((total_min % total_min_in_a_day) + total_min_in_a_day) % total_min_in_a_day;

        let hour = mod_minutes / 60;
        let min = mod_minutes % 60;
        (hour, min)
    }
}
