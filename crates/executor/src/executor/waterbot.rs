use std::{cell::Cell, sync::Arc};

use chrono::Local;

use crate::sender::dingtalk::DingTalkSender;

use super::Executor;

pub struct WaterBot {
    times: Cell<usize>,
    sender: Arc<DingTalkSender>
}

impl WaterBot {
    pub fn new(sender: Arc<DingTalkSender>) -> Self {
        WaterBot { times: Cell::new(0), sender }
    }
    fn build_content(&self) -> String {
        let now = Local::now();
        let times = self.times.get() + 1;
        format!("大家好，我是本群的【喝水提醒小助手】，这是今天的第{}轮，希望此刻看到消息的小伙伴可以和我一起喝一杯水，一小时后我会继续提醒大家喝水，和我一起成为一天喝8杯水的人！", times)
    }
}

impl Executor for WaterBot {
    fn execute(&self) -> anyhow::Result<()> {
        // self.sender.send(&self.build_content()).await?;
        Ok(())
    }
}
