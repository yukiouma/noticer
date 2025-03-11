use super::Executor;
use crate::sender::dingtalk::DingTalkSender;
use async_trait::async_trait;
use chrono::{Datelike, Local};
use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};
use tracing::error;

pub struct WaterBot {
    id: usize,
    times: AtomicUsize,
    sender: Arc<DingTalkSender>,
    day: Mutex<u32>,
}

impl WaterBot {
    pub fn new(id: usize, sender: Arc<DingTalkSender>) -> Self {
        let day = Mutex::new(Local::now().day());
        WaterBot {
            id,
            times: AtomicUsize::new(0),
            sender,
            day,
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }
    fn build_content(&self) -> String {
        format!("大家好，我是本群的【喝水提醒小助手】，这是今天的第{}轮，希望此刻看到消息的小伙伴可以和我一起喝一杯水，一小时后我会继续提醒大家喝水，和我一起成为一天喝8杯水的人～", self.times.load(Ordering::Relaxed) + 1)
    }
}

#[async_trait]
impl Executor for WaterBot {
    async fn execute(&self) -> anyhow::Result<()> {
        let content = self.build_content();
        let sender = Arc::clone(&self.sender);
        if let Err(err) = sender.send(&content).await {
            error!("send message failed, because: {}", err);
        };
        let mut day = self.day.lock().expect("Failed to get the lock of day");
        if Local::now().day().ne(day.deref()) {
            self.times.store(1, Ordering::Relaxed);
            *day = Local::now().day();
        } else {
            self.times.fetch_add(1, Ordering::Relaxed);
        }
        Ok(())
    }
}
