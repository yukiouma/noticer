use sqlx::MySqlPool;
use std::{error::Error, sync::Arc};
use task::{Task, TaskUsecase};

#[tokio::test]
async fn usecase_test() -> Result<(), Box<dyn Error>> {
    let database_url = "mysql://root:000000@localhost:3306/noticer?parseTime=True";
    let pool = MySqlPool::connect(&database_url).await?;
    let uc = Arc::new(TaskUsecase::new(pool));
    let mut task = Task::new("test");
    task.set_event_id(1)
        .set_description("test")
        .set_duration((8, 0), (18, 00))
        .set_time_gap(1);
    uc.create_task(&task).await?;
    let mut task = Task::new("test2");
    task.set_event_id(2)
        .set_description("test 2")
        .set_timepoint(11, 28)
        .set_weekday(1)
        .set_weekday(2)
        .set_weekday(3)
        .set_weekday(4)
        .set_weekday(5);
    uc.create_task(&task).await?;
    Ok(())
}
