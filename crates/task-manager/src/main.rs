use std::sync::Arc;

use sqlx::MySqlPool;
use task_manager::{Scheduler, TaskRepo};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = "mysql://root:000000@localhost:3306/noticer?parseTime=True";
    let pool = MySqlPool::connect(&database_url).await?;
    let repo = Arc::new(TaskRepo::new(pool));

    let scheduler = Scheduler::new(repo.clone());
    let tx = scheduler.run()?;
    loop {
        let task_id = tx.recv().unwrap();
        let mut task = repo.find_task_by_id(task_id).await.unwrap().unwrap();
        task.execute();
        repo.update_task(&task).await.unwrap();
        println!("task id: {}", task_id);
    }
    // Ok(())
}
