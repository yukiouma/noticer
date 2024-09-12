use std::{collections::HashMap, sync::Arc};

use crate::sender::dingtalk::DingTalkSender;

use super::{waterbot::WaterBot, Executor};

pub static WATERBOT_ID: usize = 0;
static WATERBOT_RESET_HOUR: u32 = 18;

pub struct ExecutorManager {
    executors: HashMap<usize, Box<dyn Executor>>,
}

impl ExecutorManager {
    fn new() -> ExecutorManager {
        ExecutorManager {
            executors: HashMap::new(),
        }
    }

    fn set_executor<T: Executor + 'static>(&mut self, id: usize, executor: T) {
        self.executors.insert(id, Box::new(executor));
    }

    pub fn execute(&self, id: usize) -> anyhow::Result<()> {
        if let Some(executor) = self.executors.get(&id) {
            executor.execute()?;
        }
        Ok(())
    }
}

pub fn new_executor_manager(dingtalk_url: &str) -> ExecutorManager {
    let sender = Arc::new(DingTalkSender::new(dingtalk_url));
    let waterbot = WaterBot::new(WATERBOT_ID, sender, WATERBOT_RESET_HOUR);
    let mut manager = ExecutorManager::new();
    manager.set_executor(waterbot.id(), waterbot);
    manager
}
