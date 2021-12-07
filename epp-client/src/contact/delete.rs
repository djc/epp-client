//! Types for EPP contact delete request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, NoExtension, StringValue};
use crate::request::Transaction;
use crate::response::ResponseStatus;
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for ContactDelete {
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
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn contact_delete() {
        let xml = get_xml("response/contact/delete.xml").unwrap();
        let object = ContactDelete::deserialize_response(xml.as_str()).unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
