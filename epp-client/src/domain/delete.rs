//! Types for EPP domain delete request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, NoExtension, StringValue};
use crate::request::Transaction;
use crate::response::ResponseStatus;
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for DomainDelete {
    type Response = ResponseStatus;
    type ExtensionResponse = NoExtension;
}

impl DomainDelete {
    pub fn new(name: &str) -> Self {
        Self {
            domain: DomainDeleteRequestData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
            },
        }
    }
}

/// Type for &lt;name&gt; element under the domain &lt;delete&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainDeleteRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    xmlns: String,
    /// The domain to be deleted
    #[serde(rename = "domain:name", alias = "name")]
    name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
/// Type for EPP XML &lt;delete&gt; command for domains
pub struct DomainDelete {
    /// The data under the &lt;delete&gt; tag for domain deletion
    #[serde(rename = "domain:delete", alias = "delete")]
    domain: DomainDeleteRequestData,
}

#[cfg(test)]
mod tests {
    use super::DomainDelete;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/domain/delete.xml").unwrap();

        let object = DomainDelete::new("eppdev.com");

        let serialized =
            <DomainDelete as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/domain/delete.xml").unwrap();
        let object =
            <DomainDelete as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
