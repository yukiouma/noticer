use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc,
    },
    thread,
    time::Duration,
};

use crate::TaskRepo;

pub struct Scheduler {
    repo: Arc<TaskRepo>,
}

impl Scheduler {
    pub fn new(repo: Arc<TaskRepo>) -> Scheduler {
        Scheduler { repo }
    }

    pub fn run(&self) -> anyhow::Result<Receiver<i32>> {
        let (tx, rx) = mpsc::channel::<i32>();
        let repo = self.repo.clone();
        tokio::spawn(async move {
            loop {
                if let Ok(tasks) = repo.list_tasks().await {
                    let ready_task_id = tasks
                        .into_iter()
                        .filter(|task| task.ready_to_execute())
                        .map(|task| task.id())
                        .collect::<Vec<_>>();
                    println!("Ready task id: {:?}", ready_task_id);
                    for id in ready_task_id {
                        tx.send(id).unwrap();
                    }
                }
                thread::sleep(Duration::from_secs(10));
            }
        });
        Ok(rx)
    }
}
