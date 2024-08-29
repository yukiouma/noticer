use sqlx::MySqlPool;

use super::entity::{Task, TaskDAO};

pub struct TaskRepo {
    pool: MySqlPool,
}

impl TaskRepo {
    pub  fn new(pool: MySqlPool) -> TaskRepo {
        TaskRepo { pool }
    }
    pub async fn list_tasks(&self) -> anyhow::Result<Vec<Task>> {
        Ok(sqlx::query_as::<_, TaskDAO>("SELECT * FROM `task`").fetch_all(&self.pool).await?.into_iter().map(|dao|dao.into()).collect::<Vec<Task>>())
    }
    pub async fn create_task(&self) -> anyhow::Result<()> {
        // sqlx::query!("INSERT INTO `task` () VALUE ()").execute(&self.pool).await?
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn repo_test() -> anyhow::Result<()> {
        let database_url = "mysql://root:000000@localhost:3306/noticer?parseTime=True";
        let pool = MySqlPool::connect(&database_url).await?;
        let repo = TaskRepo::new(pool);
        let tasks = repo.list_tasks().await?;
        assert_eq!(tasks.len(), 0);
        Ok(())
    }
}