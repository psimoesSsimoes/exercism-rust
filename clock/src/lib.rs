use std::fmt;
pub struct Clock{
	hours: i32,
	minutes:i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
		let mut h = (hours + minutes / 60) % 24;
        let m = minutes % 60;
        if m < 0 {
            h -= 1;
        }

        Clock {
            hours: if h < 0 { h + 24 } else { h },
            minutes: if m < 0 { m + 60 } else { m },
		}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
		 Clock::new(self.hours, self.minutes + minutes)
    }

	pub fn to_string(&self) -> String{
		format!("{:02}:{:02}",self.hours,self.minutes)
	}
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}",self.hours,self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
		(self.hours == other.hours) &&
			(self.minutes==other.minutes)
    }
}
