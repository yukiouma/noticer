use dotenv::dotenv;
use sqlx::MySqlPool;
use std::{env, error::Error, sync::Arc};
use task::{EventLooper, TaskUsecase};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let gap = env::var("GAP")?.parse::<u64>()?;
    let database_url = env::var("DATABASE_URL")?;
    let pool = MySqlPool::connect(&database_url).await?;
    let uc = Arc::new(TaskUsecase::new(pool));
    let looper = EventLooper::new(Arc::clone(&uc));
    looper.run(gap).await?;
    Ok(())
}
