use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use sqlx::types::chrono::NaiveDateTime;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Join,
    Text,
    Close,
    IntervalBatch,
    FixedBatch
}


impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MessageType::Join => {"join"}
            MessageType::Text => {"text"}
            MessageType::Close => {"close"}
            MessageType::IntervalBatch => {"interval_batch"}
            MessageType::FixedBatch => {"fixed_batch"}
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct SocketMessage {

    pub messageType: MessageType,

    #[serde(skip_serializing)]
    pub ipaddr: String,

    #[serde(skip_serializing_if = "String::is_empty" )]
    pub name: String,

    #[serde(skip_serializing_if = "String::is_empty" )]
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime: Option<NaiveDateTime>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub timestamp: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch: Option<Vec<SocketMessage>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<(NaiveDateTime, NaiveDateTime)>,

    #[serde(skip_serializing, skip_deserializing)]
    pub return_channel: Option<oneshot::Sender<mpsc::Receiver<SocketMessage>>>
}

impl Display for SocketMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        write!(f, "\"messageType\": {}, \"ipaddr\": {}, \"name\": {}, \"text\": {}, ", self.messageType, self.ipaddr, self.name, self.text)
    }
}