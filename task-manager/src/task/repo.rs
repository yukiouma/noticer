use sqlx::{MySql, MySqlPool, QueryBuilder};

use super::entity::{Task, TaskDAO};

pub struct TaskRepo {
    pool: MySqlPool,
}

impl TaskRepo {
    pub fn new(pool: MySqlPool) -> TaskRepo {
        TaskRepo { pool }
    }
    pub async fn list_tasks(&self) -> anyhow::Result<Vec<Task>> {
        Ok(sqlx::query_as::<_, TaskDAO>(
            r#"
SELECT 
    `id`,
    `name`, 
    `description`, 
    `expect_times`, 
    `month`, 
    `day`, 
    `weekday`, 
    `timepoint`, 
    `time_gap`,
    `duration_start`, 
    `duration_end`, 
    `execute_times`, 
    `last_executed_at`
FROM `task`"#,
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|dao| dao.into())
        .collect::<Vec<Task>>())
    }
    pub async fn create_task(&self, task: &Task) -> anyhow::Result<()> {
        let mut query = QueryBuilder::<MySql>::new(
            r#"
INSERT INTO `task` (
    `name`, 
    `description`, 
    `expect_times`, 
    `month`, 
    `day`, 
    `weekday`, 
    `timepoint`, 
    `time_gap`,
    `duration_start`, 
    `duration_end`, 
    `execute_times`, 
    `last_executed_at`
)"#,
        );
        let task: TaskDAO = task.clone().into();
        query.push_values(vec![task], |mut builder, task| {
            builder
                .push_bind(task.name)
                .push_bind(task.description)
                .push_bind(task.expect_times)
                .push_bind(task.month)
                .push_bind(task.day)
                .push_bind(task.weekday)
                .push_bind(task.timepoint)
                .push_bind(task.time_gap)
                .push_bind(task.duration_start)
                .push_bind(task.duration_end)
                .push_bind(task.execute_times)
                .push_bind(task.last_executed_at);
        });
        query.build().execute(&self.pool).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Local};

    use super::*;
    #[tokio::test]
    async fn repo_test() -> anyhow::Result<()> {
        let database_url = "mysql://root:000000@localhost:3306/noticer?parseTime=True";
        let pool = MySqlPool::connect(&database_url).await?;
        let repo = TaskRepo::new(pool);
        let mut new_task = Task::new("demo");
        new_task
            .set_description("demo description")
            .set_weekday(1)
            .set_weekday(2)
            .set_weekday(3)
            .set_weekday(4)
            .set_weekday(5);
        repo.create_task(&new_task).await?;
        let tasks = repo.list_tasks().await?;
        assert_eq!(tasks.len(), 1);
        let task = tasks.first().unwrap();
        let now = Local::now();
        let weekday = now.weekday().num_days_from_monday() + 1;
        assert!(task.match_weekday(weekday.try_into().unwrap()));
        Ok(())
    }
}
