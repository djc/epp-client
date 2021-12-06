//! Types for EPP domain check request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, NoExtension, StringValue};
use crate::request::Transaction;
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for DomainCheck {
    type Response = DomainCheckResponse;
    type ExtensionResponse = NoExtension;
}

impl DomainCheck {
    pub fn new(domains: Vec<&str>) -> Self {
        Self {
            list: DomainList {
                xmlns: XMLNS.to_string(),
                domains: domains
                    .into_iter()
                    .map(|d| d.into())
                    .collect::<Vec<StringValue>>(),
            },
        }
    }
}

// Request

/// Type for &lt;name&gt; elements under the domain &lt;check&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainList {
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    /// XML namespace for domain commands
    pub xmlns: String,
    #[serde(rename = "domain:name", alias = "name")]
    /// List of domains to be checked for availability
    pub domains: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct DomainCheck {
    /// The object holding the list of domains to be checked
    #[serde(rename = "domain:check", alias = "check")]
    list: DomainList,
}

// Response

/// Type that represents the &lt;name&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainAvailable {
    /// The domain name
    #[serde(rename = "$value")]
    pub name: StringValue,
    /// The domain (un)availability
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the &lt;cd&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckResponseDataItem {
    /// Data under the &lt;name&gt; tag
    #[serde(rename = "name")]
    pub domain: DomainAvailable,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the &lt;chkData&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckResponseData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// Data under the &lt;cd&gt; tag
    #[serde(rename = "cd")]
    pub domain_list: Vec<DomainCheckResponseDataItem>,
}

/// Type that represents the &lt;resData&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckResponse {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "chkData")]
    pub check_data: DomainCheckResponseData,
}
