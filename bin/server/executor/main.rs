use std::env;

extern crate executor;

use executor::{new_executor_manager, WATERBOT_ID};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let dingtalk_url = env::var("DINGTALK_URL")?;
    println!("DingTalk Url: {}", dingtalk_url);
    let executor = new_executor_manager(&dingtalk_url);
    for _ in 0..1 {
        executor.execute(WATERBOT_ID)?;
    }
    Ok(())
}
