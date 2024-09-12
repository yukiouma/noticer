mod waterbot;
mod manager;

pub use manager::{new_executor_manager, WATERBOT_ID};

trait Executor {
    fn execute(&self) -> anyhow::Result<()>;
}
