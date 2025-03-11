use executor::{DingTalkSender, ExecutorManager, WaterBot};
use std::{env, sync::Arc};

#[tokio::test]
async fn test_executor() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let port = env::var("EXECUTE_PORT")?.parse::<u16>()?;
    let dingtalk_url = env::var("DINGTALK_URL")?;
    let waterbot_id = env::var("WATERBOT_ID")?.parse::<usize>()?;
    let mut manager = ExecutorManager::new();
    let dingtalk_sender = Arc::new(DingTalkSender::new(&dingtalk_url));
    let water_bot = WaterBot::new(waterbot_id, Arc::clone(&dingtalk_sender));
    manager.set_executor(water_bot.id(), water_bot);
    executor::serve(port, Arc::new(manager)).await?;
    Ok(())
}
