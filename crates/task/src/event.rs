use crate::{task::Task, usecase::TaskUsecase};
use lazy_static::lazy_static;
use std::{env, sync::Arc};
use tokio;
use tracing::{error, info, trace, Level};

lazy_static! {
    static ref EXECUTOR_HOST: String = env::var("EXECUTOR_HOST").unwrap();
    static ref EXECUTOR_PORT: String = env::var("EXECUTOR_PORT").unwrap();
}

pub struct EventLooper {
    usecase: Arc<TaskUsecase>,
}

impl EventLooper {
    pub fn new(usecase: Arc<TaskUsecase>) -> Self {
        EventLooper { usecase }
    }

    pub async fn run(&self, gap_seconds: u64) -> anyhow::Result<()> {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
        // create_tasks(Arc::clone(&self.usecase)).await?;
        info!("Event looper start with gap: {} seconds", gap_seconds);
        loop {
            trace!("EventLooper awake");
            let tasks = self.usecase.list_tasks().await?;
            let mut no_task = true;
            for mut task in tasks {
                if task.ready_to_execute() {
                    no_task = false;
                    let name = task.name().to_owned();
                    info!("Start to execute task: {}", name);
                    match self.execute(&mut task).await {
                        Ok(_) => info!("Task {} complete", name),
                        Err(err) => error!("Task {} failed because: {}", name, err),
                    }
                }
            }
            if no_task {
                info!("There is no task to be executed");
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(gap_seconds)).await;
        }
    }

    pub async fn execute(&self, task: &mut Task) -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let execute_url = format!(
            "{}:{}/api/execute?id={}",
            *EXECUTOR_HOST,
            *EXECUTOR_PORT,
            task.event_id.unwrap()
        );
        client.get(&execute_url).send().await?;
        task.execute();
        self.usecase.update_task(&task).await?;
        Ok(())
    }
}

#[allow(unused)]
async fn create_tasks(uc: Arc<TaskUsecase>) -> anyhow::Result<()> {
    create_water_task(Arc::clone(&uc)).await?;
    create_lunch_task(Arc::clone(&uc)).await?;
    create_oa_task(Arc::clone(&uc)).await?;
    Ok(())
}

async fn create_water_task(uc: Arc<TaskUsecase>) -> anyhow::Result<()> {
    let mut task = Task::new("water");
    task.set_event_id(1)
        .set_description("drink water notice")
        .set_duration((8, 0), (20, 00))
        .set_time_gap(60);
    uc.create_task(&task).await?;
    Ok(())
}

async fn create_lunch_task(uc: Arc<TaskUsecase>) -> anyhow::Result<()> {
    let mut task = Task::new("lunch");
    task.set_event_id(2)
        .set_description("lunch time notice")
        .set_timepoint(11, 30)
        .set_weekday(1)
        .set_weekday(2)
        .set_weekday(3)
        .set_weekday(4)
        .set_weekday(5);
    uc.create_task(&task).await?;
    Ok(())
}

async fn create_oa_task(uc: Arc<TaskUsecase>) -> anyhow::Result<()> {
    let mut task = Task::new("oa");
    task.set_event_id(3)
        .set_description("oa notice")
        .set_timepoint(9, 30)
        .set_weekday(1);
    uc.create_task(&task).await?;
    Ok(())
}
