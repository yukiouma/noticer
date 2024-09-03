use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct DingTalkSender {
    client: Client,
}

impl DingTalkSender {
    pub fn new() -> DingTalkSender {
        let client = Client::new();
        DingTalkSender { client }
    }

    pub async fn send(&self, content: &str) -> anyhow::Result<()> {
        let reply = self
            .client
            .post("")
            .json(&DingTalkRequestBody {
                content: content.into(),
            })
            .send()
            .await?;
        match reply.json::<DingTalkReply>().await? {
            DingTalkReply::Success(_) => Ok(()),
            DingTalkReply::Failed(_) => todo!(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DingTalkRequestBody {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub enum DingTalkReply {
    Success(SuccessReply),
    Failed(FailedReply),
}

#[derive(Debug, Deserialize)]
pub struct SuccessReply {}

#[derive(Debug, Deserialize)]
pub struct FailedReply {}
