use sqlx::MySqlPool;

pub struct TaskRepo {
    pool: MySqlPool,
}

impl TaskRepo {
    pub async fn new(pool: MySqlPool) -> TaskRepo {
        TaskRepo { pool }
    }
    pub async fn get_task_by_name(&self, task: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
