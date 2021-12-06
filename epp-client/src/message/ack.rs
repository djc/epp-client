//! Types for EPP message ack request

use epp_client_macros::*;

use crate::common::{ElementName, NoExtension};
use crate::request::Transaction;
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for MessageAck {
    type Response = String;
    type ExtensionResponse = NoExtension;
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "poll")]
/// Type for EPP XML &lt;poll&gt; command for message ack
pub struct MessageAck {
    /// The type of operation to perform
    /// The value is "ack" for message acknowledgement
    op: String,
    /// The ID of the message to be acknowledged
    #[serde(rename = "msgID")]
    message_id: String,
}

impl MessageAck {
    pub fn new(message_id: u32) -> Self {
        Self {
            op: "ack".to_string(),
            message_id: message_id.to_string(),
        }
    }
}
