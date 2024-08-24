use chrono::{DateTime, Local};

const ONE_DAY_MINUTE: u32 = 1440;

#[derive(Debug, Default)]
pub struct Task {
    id: u64,
    // task basic info
    name: String,
    description: String,

    // task schedule parameters

    // expect times less than 0 stands for unlimited execute times
    expect_times: i32,
    month: u16,
    day: u32,
    weekday: u8,
    timepoint: u32,
    time_gap: u32,
    duration_start: u32,
    duration_end: u32,
    execute_times: i32,
    last_execute_at: DateTime<Local>,
}

impl Task {
    pub fn new(name: &str) -> Task {
        Task {
            name: name.into(),
            expect_times: -1,
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
    pub fn set_month(&mut self, month: usize) -> &mut Self {
        if month.le(&12) {
            self.month = self.month | (1 << (month - 1));
        }
        self
    }
    pub fn month(&self) -> Vec<usize> {
        let mut months = vec![];
        for i in 0..12 {
            if (self.month | !(1 << i)).eq(&!0) {
                months.push(i + 1)
            }
        }
        months
    }
    pub fn set_weekday(&mut self, weekday: usize) -> &mut Self {
        if weekday.le(&7) {
            self.weekday = self.weekday | (1 << (weekday - 1));
        }
        self
    }
    pub fn weekday(&self) -> Vec<usize> {
        let mut weekdays = vec![];
        for i in 0..7 {
            if (self.weekday | !(1 << i)).eq(&!0) {
                weekdays.push(i + 1)
            }
        }
        weekdays
    }
    pub fn set_day(&mut self, day: usize) -> &mut Self {
        if day.le(&31) {
            self.day = self.day | (1 << (day - 1));
        }
        self
    }
    pub fn day(&self) -> Vec<usize> {
        let mut days = vec![];
        for i in 0..31 {
            if (self.day | !(1 << i)).eq(&!0) {
                days.push(i + 1)
            }
        }
        days
    }
    pub fn set_timepoint(&mut self, hour: u32, minute: u32) -> &mut Self {
        self.timepoint = hour * 60 + minute;
        self
    }
    pub fn timepoint(&self) -> (u32, u32) {
        let hour = self.timepoint / 60;
        let minute = self.timepoint % 60;
        (hour, minute)
    }
    pub fn set_time_gap(&mut self, time_gap: u32) -> &mut Self {
        if time_gap.le(&ONE_DAY_MINUTE) {
            self.time_gap = time_gap;
        }
        self
    }
    pub fn set_duration_start(&mut self, hour: u32, minute: u32) -> &mut Self {
        let timepoint = hour * 60 + minute;
        if timepoint.le(&ONE_DAY_MINUTE) {
            self.duration_start = timepoint;
        }
        self
    }

    pub fn set_duration_end(&mut self, hour: u32, minute: u32) -> &mut Self {
        let timepoint = hour * 60 + minute;
        if timepoint.le(&ONE_DAY_MINUTE) {
            self.duration_end = timepoint;
        }
        self
    }

    pub fn duration(&self) -> Option<((u32, u32), (u32, u32))> {
        if self.duration_end.lt(&self.duration_start) {
            None
        } else {
            Some((
                (self.duration_start / 60, self.duration_start % 60),
                (self.duration_end / 60, self.duration_end % 60),
            ))
        }
    }

    pub fn set_expect_times(&mut self, times: i32) -> &mut Self {
        self.execute_times = 0;
        self.expect_times = times;
        self
    }

    pub fn execute<F>(&mut self, execute_task: F) -> &mut Self
    where
        F: FnOnce(),
    {
        //  limited execute times
        if self.expect_times.gt(&0) && self.execute_times.ge(&self.expect_times) {
            return self;
        }

        //

        // execute and update task status
        execute_task();
        self.execute_times += 1;
        self.last_execute_at = Local::now();
        self
    }
    pub fn execute_times(&self) -> i32 {
        self.execute_times
    }
    pub fn last_execute_at(&self) -> &DateTime<Local> {
        &self.last_execute_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task_test() {
        let task = Task::default();
    }
    #[test]
    fn test_datetime() {
        let task = || println!("do task");
        let mut task = Task::new("demo");
        task.set_month(1).set_month(6).set_month(14);
        assert_eq!(vec![1, 6], task.month());
        task.set_weekday(1).set_weekday(5).set_weekday(9);
        assert_eq!(vec![1, 5], task.weekday());
        task.set_day(1).set_day(30).set_day(32);
        assert_eq!(vec![1, 30], task.day());
        task.set_timepoint(8, 30);
        assert_eq!((8, 30), task.timepoint());
        task.set_duration_start(8, 0).set_duration_end(17, 0);
        assert_eq!(Some(((8, 0), (17, 0))), task.duration());
        task.set_duration_start(8, 0).set_duration_end(7, 0);
        assert_eq!(None, task.duration());
        // task.execute(task).execute().execute();
        // assert_eq!(3, task.execute_times());
        // task.set_expect_times(1).execute().execute().execute();
        // assert_eq!(1, task.execute_times());
    }
}
