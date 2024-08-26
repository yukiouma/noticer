use chrono::{DateTime, Datelike, Local, Timelike};

const ONE_DAY_MINUTE: u32 = 1440;

#[derive(Debug, Default)]
pub struct Task {
    id: u64,
    // task basic info
    name: String,
    description: String,

    // task schedule parameters
    expect_times: Option<i32>,
    month: Option<u16>,
    day: Option<u32>,
    weekday: Option<u8>,
    timepoint: Option<u32>,
    time_gap: Option<u32>,
    duration_start: Option<u32>,
    duration_end: Option<u32>,
    execute_times: i32,
    last_execute_at: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(name: &str) -> Task {
        Task {
            name: name.into(),
            ..Default::default()
        }
    }
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.into();
        self
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = description.into();
        self
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn set_month(&mut self, month: u32) -> &mut Self {
        let mut m = match self.month {
            Some(m) => m,
            None => 0,
        };
        if month.le(&12) {
            m = m | (1 << (month - 1));
            self.month = Some(m);
        }
        self
    }
    pub fn month(&self) -> Option<Vec<u32>> {
        let month = self.month?;
        let mut months = vec![];
        for i in 0..12 {
            if (month | !(1 << i)).eq(&!0) {
                months.push(i + 1)
            }
        }
        Some(months)
    }
    pub fn match_month(&self, month: u32) -> bool {
        match self.month {
            Some(m) => {
                (m | !(1 << month-1)).eq(&!0)
            },
            None => true,
        }
    }
    pub fn set_weekday(&mut self, weekday: u32) -> &mut Self {
        let mut w = match self.weekday {
            Some(w) => w,
            None => 0,
        };
        if weekday.le(&7) {
            w = w | (1 << (weekday - 1));
            self.weekday = Some(w);
        }
        self
    }
    pub fn weekday(&self) -> Option<Vec<u32>> {
        let weekday = self.weekday?;
        let mut weekdays = vec![];
        for i in 0..7 {
            if (weekday | !(1 << i)).eq(&!0) {
                weekdays.push(i + 1)
            }
        }
        Some(weekdays)
    }
    pub fn match_weekday(&self, weekday: u32) -> bool {
        match self.weekday {
            Some(w) => {
                (w | !(1 << weekday-1)).eq(&!0)
            },
            None => true,
        }
    }
    pub fn set_day(&mut self, day: u32) -> &mut Self {
        let mut d = match self.day {
            Some(d) => {
                d
            },
            None => 0,
        };
        if day.le(&31) {
            d = d | (1 << (day - 1));
            self.day = Some(d);
        }
        self
    }
    pub fn day(&self) -> Option<Vec<u32>> {
        let day = self.day?;
        let mut days = vec![];
        for i in 0..31 {
            if (day | !(1 << i)).eq(&!0) {
                days.push(i + 1)
            }
        }
        Some(days)
    }
    pub fn match_day(&self, day: u32) -> bool {
        match self.day {
            Some(d) => {
                (d | !(1 << day-1)).eq(&!0)
            },
            None => true,
        }
    }
    pub fn set_timepoint(&mut self, hour: u32, minute: u32) -> &mut Self {
        self.timepoint = Some(hour * 60 + minute);
        self
    }
    pub fn timepoint(&self) -> Option<(u32, u32)> {
        let timepoint = self.timepoint?;
        let hour = timepoint / 60;
        let minute = timepoint % 60;
        Some((hour, minute))
    }
    pub fn set_time_gap(&mut self, time_gap: u32) -> &mut Self {
        if time_gap.le(&ONE_DAY_MINUTE) {
            self.time_gap = Some(time_gap);
        }
        self
    }
    pub fn set_duration_start(&mut self, hour: u32, minute: u32) -> &mut Self {
        let timepoint = hour * 60 + minute;
        if timepoint.le(&ONE_DAY_MINUTE) {
            self.duration_start = Some(timepoint);
        }
        self
    }

    pub fn set_duration_end(&mut self, hour: u32, minute: u32) -> &mut Self {
        let timepoint = hour * 60 + minute;
        if timepoint.le(&ONE_DAY_MINUTE) {
            self.duration_end = Some(timepoint);
        }
        self
    }

    pub fn duration(&self) -> Option<((u32, u32), (u32, u32))> {
        let duration_start = self.duration_start?;
        let duration_end = self.duration_end?;
        if duration_end.lt(&duration_start) {
            None
        } else {
            Some((
                (duration_start / 60, duration_start % 60),
                (duration_end / 60, duration_end % 60),
            ))
        }
    }

    pub fn set_expect_times(&mut self, times: i32) -> &mut Self {
        self.execute_times = 0;
        self.expect_times = Some(times);
        self
    }

    pub fn execute<F>(&mut self, execute_task: F) -> &mut Self
    where
        F: FnOnce(),
    {
        //  limited execute times
        if let Some(expect_times) = self.expect_times {
            if expect_times.gt(&0) && self.execute_times.ge(&expect_times) {
                return self;
            }
        }
        
        // get current time
        let now = Local::now();
        let month = now.month();
        let day = now.day();
        let weekday = now.weekday();
        let hour = now.hour();
        let miunte = now.minute();
        let gap = if let Some(last_execute_at) = self.last_execute_at {
            Some(now.signed_duration_since(&last_execute_at))
        } else {
            None
        };

        if !self.match_month(month) {
            return self;
        }

        if !self.match_day(day) {
            return self;
        }

        if !self.match_weekday(weekday.num_days_from_monday() + 1) {
            return self;
        }

        // execute and update task status
        execute_task();
        self.execute_times += 1;
        self.last_execute_at = Some(Local::now());
        self
    }
    pub fn execute_times(&self) -> i32 {
        self.execute_times
    }
    pub fn last_execute_at(&self) -> Option<DateTime<Local>> {
        self.last_execute_at.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_datetime() {
        let t = || {};
        let mut task = Task::new("demo");
        task.set_month(1).set_month(6).set_month(14);
        assert_eq!(Some(vec![1, 6]), task.month());
        assert!(task.match_month(1));
        assert!(task.match_month(6));
        assert!(!task.match_month(5));
        task.set_weekday(1).set_weekday(5).set_weekday(9);
        assert_eq!(Some(vec![1, 5]), task.weekday());
        assert!(task.match_weekday(1));
        assert!(task.match_weekday(5));
        assert!(!task.match_weekday(7));
        task.set_day(1).set_day(30).set_day(32);
        assert_eq!(Some(vec![1, 30]), task.day());
        assert!(task.match_day(1));
        assert!(task.match_day(30));
        assert!(!task.match_day(7));
        task.set_timepoint(8, 30);
        assert_eq!(Some((8, 30)), task.timepoint());
        task.set_duration_start(8, 0).set_duration_end(17, 0);
        assert_eq!(Some(((8, 0), (17, 0))), task.duration());
        task.set_duration_start(8, 0).set_duration_end(7, 0);
        assert_eq!(None, task.duration());
    }

    #[test]
    fn test_execute_task() {
        let t = || {};
        let mut task = Task::new("demo");
    }
}
