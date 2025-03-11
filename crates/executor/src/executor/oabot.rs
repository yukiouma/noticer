use super::Executor;
use crate::sender::dingtalk::DingTalkSender;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::error;

pub struct OaBot {
    id: usize,
    sender: Arc<DingTalkSender>,
}

impl OaBot {
    pub fn new(id: usize, sender: Arc<DingTalkSender>) -> Self {
        OaBot { id, sender }
    }
    pub fn id(&self) -> usize {
        self.id
    }
    fn build_content(&self) -> String {
        format!("大家好，我是本群的【OA小助手】，现在该去看看上周的考勤和周汇报有没有正常提交啦～")
    }
}

#[async_trait]
impl Executor for OaBot {
    async fn execute(&self) -> anyhow::Result<()> {
        let sender = Arc::clone(&self.sender);
        let content = Arc::new(self.build_content());
        tokio::spawn(async move {
            let content = Arc::clone(&content);
            for _ in 0..5 {
                if let Err(err) = sender.send(&content).await {
                    error!("send message failed, because: {}", err);
                };
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            }
        });
        Ok(())
    }
}
