use chrono::{DateTime, Local};
use sqlx::prelude::FromRow;

use super::entity::Task;

#[derive(Debug, FromRow)]
pub struct TaskDAO {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub expect_times: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
    pub weekday: Option<i32>,
    pub timepoint: Option<i32>,
    pub time_gap: Option<i32>,
    pub duration_start: Option<i32>,
    pub duration_end: Option<i32>,
    pub execute_times: i32,
    pub event_id: Option<i32>,
    pub last_executed_at: Option<DateTime<Local>>,
}

impl From<TaskDAO> for Task {
    fn from(value: TaskDAO) -> Self {
        Task {
            id: value.id,
            name: value.name,
            description: value.description,
            expect_times: value.expect_times,
            month: value.month,
            day: value.day,
            weekday: value.weekday,
            timepoint: value.timepoint,
            time_gap: value.time_gap,
            duration: if let Some(start) = value.duration_start {
                if let Some(end) = value.duration_end {
                    Some((start, end))
                } else {
                    None
                }
            } else {
                None
            },
            event_id: value.event_id,
            execute_times: value.execute_times,
            last_executed_at: value.last_executed_at,
        }
    }
}

impl Into<TaskDAO> for Task {
    fn into(self) -> TaskDAO {
        TaskDAO {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            expect_times: self.expect_times,
            month: self.month,
            day: self.day,
            weekday: self.weekday,
            timepoint: self.timepoint,
            time_gap: self.time_gap,
            duration_start: if let Some(duration) = self.duration {
                Some(duration.0)
            } else {
                None
            },
            duration_end: if let Some(duration) = self.duration {
                Some(duration.1)
            } else {
                None
            },
            event_id: self.event_id,
            execute_times: self.execute_times,
            last_executed_at: self.last_executed_at,
        }
    }
}
