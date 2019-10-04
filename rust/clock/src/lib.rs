#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut c = Clock {
            hours: 0,
            minutes: 0,
        };
        c.add_hours(hours);
        c.add_minutes(minutes)
    }
    pub fn add_hours(&mut self, hours: i32) {
        self.hours += hours;

        self.hours %= 24;

        if self.hours < 0 {
            self.hours += 24;
        }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.add_hours(minutes / 60);
        self.minutes += minutes % 60;

        if self.minutes > 60 {
            self.minutes = minutes % 60;
            self.add_hours(1);
        }

        if self.minutes < 0 {
            self.minutes += 60;
            self.add_hours(-1)
        }

        self
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
