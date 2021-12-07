//! Types for EPP namestore request and responses

use epp_client_macros::ElementName;
use serde::{Deserialize, Serialize};

use crate::{
    common::{ElementName, NoExtension, StringValue},
    domain::check::DomainCheck,
    request::Transaction,
};

pub const XMLNS: &str = "http://www.verisign-grs.com/epp/namestoreExt-1.1";

impl Transaction<NameStore> for DomainCheck {
    type Response = <DomainCheck as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl NameStore {
    /// Create a new RGP restore report request
    pub fn new(subproduct: &str) -> NameStore {
        NameStore {
            xmlns: XMLNS.to_string(),
            subproduct: subproduct.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "namestoreExt:namestoreExt")]
/// Type for EPP XML &lt;namestoreExt&gt; extension
pub struct NameStore {
    /// XML namespace for the RGP restore extension
    #[serde(rename = "xmlns:namestoreExt", alias = "xmlns")]
    pub xmlns: String,
    /// The object holding the list of domains to be checked
    #[serde(rename = "namestoreExt:subProduct", alias = "subProduct")]
    pub subproduct: StringValue,
}

#[cfg(test)]
mod tests {
    use super::NameStore;
    use crate::domain::check::DomainCheck;
    use crate::request::Transaction;
    use crate::tests::get_xml;

    #[test]
    fn namestore() {
        let xml = get_xml("response/extensions/namestore.xml").unwrap();

        let object =
            <DomainCheck as Transaction<NameStore>>::deserialize_response(xml.as_str()).unwrap();

        let ext = object.extension.unwrap();

        assert_eq!(ext.data.subproduct, "com".into());
    }
}
