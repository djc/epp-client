//! Types for EPP contact delete request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, Extension, NoExtension, StringValue};
use crate::request::Transaction;
use crate::response::ResponseStatus;
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for ContactDelete {
    type ExtensionWrapper = Extension<NoExtension>;
    type Response = ResponseStatus;
    type ExtensionResponse = NoExtension;
}

/// Type containing the data for the &lt;delete&gt; tag for contacts
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactDeleteRequestData {
    /// XML namespace for the &lt;delete&gt; command for contacts
    #[serde(rename = "xmlns:contact", alias = "xmlns")]
    xmlns: String,
    /// The id of the contact to be deleted
    #[serde(rename = "contact:id", alias = "id")]
    id: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
/// The &lt;delete&gt; type for the contact delete EPP command
pub struct ContactDelete {
    #[serde(rename = "contact:delete", alias = "delete")]
    /// The data for the &lt;delete&gt; tag for a contact delete command
    contact: ContactDeleteRequestData,
}

impl ContactDelete {
    pub fn new(id: &str) -> ContactDelete {
        Self {
            contact: ContactDeleteRequestData {
                xmlns: XMLNS.to_string(),
                id: id.into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContactDelete;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/contact/delete.xml").unwrap();

        let object = ContactDelete::new("eppdev-contact-3");

        let serialized =
            <ContactDelete as Transaction<NoExtension>>::serialize_request(object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/contact/delete.xml").unwrap();
        let object =
            <ContactDelete as Transaction<NoExtension>>::deserialize_response(xml.as_str())
                .unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
