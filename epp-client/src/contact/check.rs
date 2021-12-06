use std::fmt::Debug;

/// Types for EPP contact check request
use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, NoExtension, StringValue};
use crate::request::Transaction;
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for ContactCheck {
    type Response = ContactCheckResponse;
    type ExtensionResponse = NoExtension;
}

// Request

/// Type that represents the &lt;check&gt; command for contact transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactList {
    /// The XML namespace for the contact &lt;check&gt;
    #[serde(rename = "xmlns:contact", alias = "xmlns")]
    xmlns: String,
    /// The list of contact ids to check for availability
    #[serde(rename = "contact:id", alias = "id")]
    pub contact_ids: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
/// The &lt;command&gt; type for contact check command
pub struct ContactCheck {
    /// The &lt;check&gt; tag for the contact check command
    #[serde(rename = "contact:check", alias = "check")]
    list: ContactList,
}

impl ContactCheck {
    pub fn new(contact_ids: &[&str]) -> Self {
        let contact_ids = contact_ids
            .iter()
            .map(|&d| d.into())
            .collect::<Vec<StringValue>>();

        Self {
            list: ContactList {
                xmlns: XMLNS.to_string(),
                contact_ids,
            },
        }
    }
}

// Response

/// Type that represents the &lt;id&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactAvailable {
    /// The text of the &lt;id&gt; tag
    #[serde(rename = "$value")]
    pub id: StringValue,
    /// The avail attr on the &lt;id&gt; tag
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the &lt;cd&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckResponseDataItem {
    /// Data under the &lt;id&gt; tag
    #[serde(rename = "id")]
    pub contact: ContactAvailable,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the &lt;chkData&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckResponseData {
    /// XML namespace for contact response data
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    /// Data under the &lt;cd&gt; tag
    #[serde(rename = "cd")]
    pub contact_list: Vec<ContactCheckResponseDataItem>,
}

/// Type that represents the &lt;resData&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckResponse {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "chkData")]
    pub check_data: ContactCheckResponseData,
}
