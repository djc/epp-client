//! Types for EPP RGP restore request

use epp_client_macros::*;

use crate::common::ElementName;

use crate::request::EppExtension;

use serde::{Deserialize, Serialize};

use super::XMLNS;

impl RgpRestoreRequest {
    /// Creates a new instance of EppDomainRgpRestoreRequest
    pub fn new() -> RgpRestoreRequest {
        RgpRestoreRequest {
            xmlns: XMLNS.to_string(),
            restore: RgpRestoreRequestData {
                op: "request".to_string(),
            },
        }
    }
}

impl Default for RgpRestoreRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl EppExtension for RgpRestoreRequest {
    type Response = RgpRequestResponse;
}

// Request

/// Type corresponding to the &lt;restore&gt; tag for an rgp restore request
#[derive(Serialize, Deserialize, Debug)]
pub struct RgpRestoreRequestData {
    /// The value of the op attribute in the &lt;restore&gt; tag
    pub op: String,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "rgp:update")]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct RgpRestoreRequest {
    /// XML namespace for the RGP restore extension
    #[serde(rename = "xmlns:rgp", alias = "xmlns")]
    xmlns: String,
    /// The object holding the list of domains to be checked
    #[serde(rename = "rgp:restore", alias = "restore")]
    restore: RgpRestoreRequestData,
}

// Response

/// Type that represents the &lt;rgpStatus&gt; tag for domain rgp restore request response
#[derive(Serialize, Deserialize, Debug)]
pub struct RgpStatus {
    /// The domain RGP status
    #[serde(rename = "s")]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[serde(rename = "upData")]
#[element_name(name = "upData")]
/// Type that represents the &lt;resData&gt; tag for domain transfer response
pub struct RgpRequestResponse {
    #[serde(rename = "xmlns:rgp")]
    xmlns: String,
    /// Data under the &lt;rgpStatus&gt; tag
    #[serde(rename = "rgpStatus")]
    pub rgp_status: Vec<RgpStatus>,
}
