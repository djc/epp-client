//! Types for EPP RGP restore request

use epp_client_macros::*;

use crate::{
    common::{ElementName, NoExtension, Extension},
    domain::{info::DomainInfo, update::DomainUpdate},
    request::Transaction,
};

use serde::{Deserialize, Serialize};

use super::XMLNS;

impl Transaction<RgpRestoreRequest> for DomainUpdate {
    type ExtensionWrapper = Extension<RgpRestoreRequest>;
    type Response = ();
    type ExtensionResponse = RgpRequestResponse;
}

impl Transaction<RgpRestoreRequest> for DomainInfo {
    type ExtensionWrapper = Extension<RgpRestoreRequest>;
    type Response = <DomainInfo as Transaction<NoExtension>>::Response;
    type ExtensionResponse = RgpRequestResponse;
}

impl RgpRestoreRequest {
    /// Creates a new instance of EppDomainRgpRestoreRequest
    pub fn new() -> Self {
        Self {
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

#[cfg(test)]
mod tests {
    use super::RgpRestoreRequest;
    use crate::domain::info::DomainInfo;
    use crate::domain::update::{DomainChangeInfo, DomainUpdate};
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn request_command() {
        let xml = get_xml("request/extensions/rgp_restore_request.xml").unwrap();

        let domain_restore_request = RgpRestoreRequest::new();

        let mut object = DomainUpdate::new("eppdev.com");

        let change_info = DomainChangeInfo {
            registrant: None,
            auth_info: None,
        };

        object.info(change_info);

        let serialized = <DomainUpdate as Transaction<RgpRestoreRequest>>::serialize_request(
            object,
            Some(domain_restore_request),
            CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn request_response() {
        let xml = get_xml("response/extensions/rgp_restore.xml").unwrap();
        let object =
            <DomainUpdate as Transaction<RgpRestoreRequest>>::deserialize_response(xml.as_str())
                .unwrap();

        let ext = object.extension.unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(ext.data.rgp_status[0].status, "pendingRestore".to_string());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn domain_info_request_response() {
        let xml = get_xml("response/extensions/domain_info_rgp.xml").unwrap();
        let object =
            <DomainInfo as Transaction<RgpRestoreRequest>>::deserialize_response(xml.as_str())
                .unwrap();

        let ext = object.extension.unwrap();

        assert_eq!(ext.data.rgp_status[0].status, "addPeriod");
        assert_eq!(ext.data.rgp_status[1].status, "renewPeriod");
    }
}
