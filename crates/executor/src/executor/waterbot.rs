use std::{cell::Cell, sync::Arc};

use chrono::{Local, Timelike};
use tokio::runtime::Builder;

use crate::sender::dingtalk::DingTalkSender;

use super::Executor;

pub struct WaterBot {
    id: usize,
    times: Cell<usize>,
    sender: Arc<DingTalkSender>,
    /// timepoint hour when to reset times(24 hours)
    reset: u32,
}

impl WaterBot {
    pub fn new(id: usize, sender: Arc<DingTalkSender>, reset: u32) -> Self {
        WaterBot {
            id,
            times: Cell::new(0),
            sender,
            reset,
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }
    fn build_content(&self) -> String {
        let times = self.times.get() + 1;
        format!("大家好，我是本群的【喝水提醒小助手】，这是今天的第{}轮，希望此刻看到消息的小伙伴可以和我一起喝一杯水，一小时后我会继续提醒大家喝水，和我一起成为一天喝8杯水的人！", times)
    }
}

impl Executor for WaterBot {
    fn execute(&self) -> anyhow::Result<()> {
        let runtime = Builder::new_current_thread().enable_all().build()?;
        let current_times = self.times.get();
        runtime.block_on(self.sender.send(&self.build_content()))?;
        if Local::now().hour().ge(&self.reset) {
            self.times.set(0);
        } else {
            self.times.set(current_times + 1);
        }
        Ok(())
    }
}
