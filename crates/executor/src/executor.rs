mod lunchbot;
mod manager;
mod oabot;
mod waterbot;
use async_trait::async_trait;
pub use lunchbot::LunchBot;
pub use manager::ExecutorManager;
pub use oabot::OaBot;
pub use waterbot::WaterBot;

#[async_trait]
pub trait Executor {
    async fn execute(&self) -> anyhow::Result<()>;
}
