use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::task::{Task, TaskDAO};

pub struct TaskRepo {
    pool: MySqlPool,
}

impl TaskRepo {
    pub fn new(pool: MySqlPool) -> TaskRepo {
        TaskRepo { pool }
    }

    //     pub async fn find_task_by_id(&self, id: i32) -> anyhow::Result<Option<Task>> {
    //         let dao = sqlx::query_as::<_, TaskDAO>(
    //             r#"
    // SELECT
    //     `id`,
    //     `name`,
    //     `description`,
    //     `expect_times`,
    //     `month`,
    //     `day`,
    //     `weekday`,
    //     `timepoint`,
    //     `time_gap`,
    //     `duration_start`,
    //     `duration_end`,
    //     `execute_times`,
    //     `last_executed_at`,
    //     `event_id`
    // FROM `task`
    // WHERE `id` = ?"#,
    //         )
    //         .bind(id)
    //         .fetch_optional(&self.pool)
    //         .await?;
    //         if let Some(dao) = dao {
    //             Ok(Some(dao.into()))
    //         } else {
    //             Ok(None)
    //         }
    //     }

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
    `last_executed_at`,
    `event_id`
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
    `last_executed_at`,
    `event_id`
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
                .push_bind(task.last_executed_at)
                .push_bind(task.event_id);
        });
        query.build().execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_task(&self, task: &Task) -> anyhow::Result<()> {
        let task: TaskDAO = task.clone().into();
        let query = sqlx::query(
            r#"
UPDATE `task` 
SET 
    `name` = ?, 
    `description` = ?, 
    `expect_times` = ?, 
    `month` = ?,
    `day` = ?,
    `weekday` = ?,
    `timepoint` = ?,
    `time_gap` = ?,
    `duration_start` = ?,
    `duration_end` = ?,
    `execute_times` = ?,
    `last_executed_at` = ?,
    `event_id` = ?
WHERE
    `id` = ?;
        "#,
        )
        .bind(task.name)
        .bind(task.description)
        .bind(task.expect_times)
        .bind(task.month)
        .bind(task.day)
        .bind(task.weekday)
        .bind(task.timepoint)
        .bind(task.time_gap)
        .bind(task.duration_start)
        .bind(task.duration_end)
        .bind(task.execute_times)
        .bind(task.last_executed_at)
        .bind(task.event_id)
        .bind(task.id);

        query.execute(&self.pool).await?;
        // query.pu
        Ok(())
    }
}
