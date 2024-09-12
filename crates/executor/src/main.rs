use std::{collections::HashMap, env, sync::Arc};

use executor::{
    executor::{waterbot::WaterBot, Executor},
    sender::dingtalk::DingTalkSender,
};

const DINGTALK_URL_ENV: &str = "DINGTALK_URL";
const WATERBOT_ID: usize = 0;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = env::var(DINGTALK_URL_ENV)?;
    let sender = Arc::new(DingTalkSender::new(&url));
    let mut executor_map: HashMap<usize, Box<dyn Executor>> = HashMap::new();
    let waterbot_executor = WaterBot::new(WATERBOT_ID, sender.clone(), 18);
    executor_map.insert(waterbot_executor.id(), Box::new(waterbot_executor));
    Ok(())
}
