use executor::{DingTalkSender, ExecutorManager, LunchBot, OaBot, WaterBot};
use std::{env, sync::Arc};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let port = env::var("EXECUTOR_PORT")?.parse::<u16>()?;
    let dingtalk_url = env::var("DINGTALK_URL")?;
    let waterbot_id = env::var("WATERBOT_ID")?.parse::<usize>()?;
    let lunchbot_id = env::var("LUNCHBOT_ID")?.parse::<usize>()?;
    let oabot_id = env::var("OABOT_ID")?.parse::<usize>()?;
    let mut manager = ExecutorManager::new();
    let dingtalk_sender = Arc::new(DingTalkSender::new(&dingtalk_url));
    let water_bot = WaterBot::new(waterbot_id, Arc::clone(&dingtalk_sender));
    let lunch_bot = LunchBot::new(lunchbot_id, Arc::clone(&dingtalk_sender));
    let oa_bot = OaBot::new(oabot_id, Arc::clone(&dingtalk_sender));
    manager.set_executor(water_bot.id(), water_bot);
    manager.set_executor(lunch_bot.id(), lunch_bot);
    manager.set_executor(oa_bot.id(), oa_bot);
    executor::serve(port, Arc::new(manager)).await?;
    Ok(())
}
