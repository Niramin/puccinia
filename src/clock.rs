use std::fmt::{self, Debug};


#[derive(Debug)]
pub struct Clock
{
    hours:i32,
    minutes: i32

}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{hours,minutes}
        
    }


    pub fn add_minutes(&self, minutes: i32) -> Self {
        self
        .add_minutes_raw(minutes)
        .normalize()
    }

    fn normalize(&self) -> Self {
        self
        .normalize_negative_minutes()
        .normalize_overflow_minutes()
        .normalize_overflow_hours()
        .normalize_negative_hours()
    }

    fn add_minutes_raw(&self, minutes: i32) -> Self
    {
        let hours = self.hours;
        let new_minutes = self.minutes + minutes;
        Clock{hours,minutes:new_minutes}
    }

    fn normalize_negative_minutes(&self) -> Self
    {
        if self.are_minutes_negative()
        {
            Clock { hours:self. hours -1, minutes: self.minutes+60 }.normalize_negative_minutes()
        }
        else {
            Clock { hours: self.hours, minutes: self.minutes }
        }

    }

    fn normalize_negative_hours(&self) -> Self
    {
        if self.are_hours_negative()
        {
            Clock { hours:self. hours +24, minutes: self.minutes }.normalize_negative_hours()
        }
        else {
            Clock { hours: self.hours, minutes: self.minutes }
        }
        

    }

    fn are_minutes_negative(&self) -> bool
    {
        self.minutes < 0
    }

    fn are_hours_negative(&self) -> bool
    {
        self.hours < 0
    }

    fn normalize_overflow_hours(&self) -> Self
    {
        let mut clock = Clock { hours: self.hours, minutes: self.minutes };
        if self.hours>=24
        {
            clock = Clock{hours : self.hours -24, minutes : self.minutes}.normalize_overflow_hours()
        }
        clock

    }

    fn normalize_overflow_minutes(&self) -> Self
    {
        let mut clock = Clock { hours: self.hours, minutes: self.minutes };
        if self.minutes>=60
        {
            clock = Clock{hours : self.hours +1, minutes : self.minutes-60}.normalize_overflow_minutes()
        }
        clock

    }
}


impl fmt::Display for Clock
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let normalized_clock = self.normalize();
        write!(f, "{:02}:{:02}", normalized_clock.hours, normalized_clock.minutes)
    }
}


impl PartialEq for Clock
{
    fn eq(&self, other: &Self) -> bool {
        let clock1 = self.normalize();
        let clock2 = other.normalize();
        println!("{} {}", clock1, clock2);
        clock1.minutes == clock2.minutes && clock1.hours == clock2.hours
    }
}