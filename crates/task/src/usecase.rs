use crate::{repo::TaskRepo, task::Task};
use sqlx::{MySql, Pool};

pub struct TaskUsecase {
    repo: TaskRepo,
}

impl TaskUsecase {
    pub fn new(pool: Pool<MySql>) -> Self {
        TaskUsecase {
            repo: TaskRepo::new(pool),
        }
    }

    // pub async fn find_task_by_id(&self, id: i32) -> anyhow::Result<Option<Task>> {
    //     let tasks = self.repo.find_task_by_id(id).await?;
    //     Ok(tasks)
    // }

    pub async fn list_tasks(&self) -> anyhow::Result<Vec<Task>> {
        let tasks = self.repo.list_tasks().await?;
        Ok(tasks)
    }

    pub async fn create_task(&self, task: &Task) -> anyhow::Result<()> {
        self.repo.create_task(task).await
    }

    pub async fn update_task(&self, task: &Task) -> anyhow::Result<()> {
        self.repo.update_task(task).await
    }
}
