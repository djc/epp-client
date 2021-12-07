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

#[cfg(test)]
mod tests {
    use super::MessageAck;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/message/ack.xml").unwrap();

        let object = MessageAck::new(12345);

        let serialized = object.serialize_request(None, CLTRID).unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/message/ack.xml").unwrap();
        let object = MessageAck::deserialize_response(xml.as_str()).unwrap();

        let msg = object.message_queue().unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(msg.count, 4);
        assert_eq!(msg.id, "12345".to_string());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
