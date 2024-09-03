use std::{thread, time::Duration};

use crate::TaskRepo;

pub struct Scheduler<'a> {
    repo: &'a TaskRepo,
}

impl<'a> Scheduler<'a> {
    pub fn new(repo: &'a TaskRepo) -> Scheduler {
        Scheduler { repo }
    }
    async fn list_ready_task_id(&self) -> anyhow::Result<Vec<i32>> {
        let tasks = self.repo.list_tasks().await?;

        let ready_task_id = tasks
            .into_iter()
            .filter(|task| task.ready_to_execute())
            .map(|task| task.id())
            .collect::<Vec<_>>();
        Ok(ready_task_id)
    }
    pub fn run(&self) -> anyhow::Result<()> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        loop {
            let ready_task_id = rt.block_on(self.list_ready_task_id())?;
            // TODO: send task id to message queue
            println!("Sending task id: {:?} to message queue", ready_task_id);
            thread::sleep(Duration::from_secs(10));
        }
    }
}
