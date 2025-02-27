//! Types for EPP message poll response

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; tag for the EPP XML message poll response
pub type EppMessagePollResponse = EppObject<CommandResponse<MessagePollResult>>;

/// Type that represents the &lt;trnData&gt; tag for message poll response
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDomainTransferData {
    /// XML namespace for message response data
    #[serde(rename = "xmlns:obj")]
    xmlns: String,
    /// The name of the domain under transfer
    pub name: StringValue,
    /// The domain transfer status
    #[serde(rename = "trStatus")]
    pub transfer_status: StringValue,
    /// The epp user who requested the transfer
    #[serde(rename = "reID")]
    pub requester_id: StringValue,
    /// The date of the transfer request
    #[serde(rename = "reDate")]
    pub requested_at: StringValue,
    /// The epp user who should acknowledge the transfer request
    #[serde(rename = "acID")]
    pub ack_id: StringValue,
    /// The date by which the transfer request should be acknowledged
    #[serde(rename = "acDate")]
    pub ack_by: StringValue,
    /// The domain expiry date
    #[serde(rename = "exDate")]
    pub expiring_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for message poll response
#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePollResult {
    /// Data under the &lt;trnData&gt; tag
    #[serde(rename = "trnData")]
    pub message_data: MessageDomainTransferData,
}
