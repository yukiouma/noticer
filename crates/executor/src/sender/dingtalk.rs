use reqwest::{header::CONTENT_TYPE, Client};
use serde::{Deserialize, Serialize};

static MESSAGE_TYPE: &str = "text";

pub struct DingTalkSender {
    client: Client,
    url: String,
}

impl DingTalkSender {
    pub fn new(url: &str) -> DingTalkSender {
        let client = Client::new();
        DingTalkSender {
            client,
            url: url.into(),
        }
    }

    pub async fn send(&self, content: &str) -> anyhow::Result<()> {
        let reply = self
            .client
            .post(&self.url)
            .header(CONTENT_TYPE, "application/json")
            .json(&DingTalkRequestBody {
                msgtype: MESSAGE_TYPE.into(),
                text: Text {
                    content: content.into(),
                },
            })
            .send()
            .await?;
        let reply = reply.json::<DingTalkReply>().await?;
        println!("[{}]: {}", reply.errcode, reply.errmsg);
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct DingTalkRequestBody {
    pub msgtype: String,
    pub text: Text,
}

#[derive(Debug, Serialize)]
pub struct Text {
    content: String,
}

#[derive(Debug, Deserialize)]
pub struct DingTalkReply {
    pub errcode: usize,
    pub errmsg: String,
}
