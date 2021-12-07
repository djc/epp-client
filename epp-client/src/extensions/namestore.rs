//! Types for EPP namestore request and responses

use epp_client_macros::ElementName;
use serde::{Deserialize, Serialize};

use crate::{
    common::{ElementName, NoExtension, StringValue},
    contact::{
        check::ContactCheck, create::ContactCreate, delete::ContactDelete, info::ContactInfo,
        update::ContactUpdate,
    },
    domain::{
        check::DomainCheck, create::DomainCreate, delete::DomainDelete, info::DomainInfo,
        renew::DomainRenew, transfer::DomainTransfer, update::DomainUpdate,
    },
    host::{
        check::HostCheck, create::HostCreate, delete::HostDelete, info::HostInfo,
        update::HostUpdate,
    },
    request::Transaction,
};

pub const XMLNS: &str = "http://www.verisign-grs.com/epp/namestoreExt-1.1";

// Contact

impl Transaction<NameStore> for ContactCheck {
    type Response = <ContactCheck as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for ContactCreate {
    type Response = <ContactCreate as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for ContactDelete {
    type Response = <ContactDelete as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for ContactInfo {
    type Response = <ContactInfo as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for ContactUpdate {
    type Response = <ContactUpdate as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

// Domain

impl Transaction<NameStore> for DomainCheck {
    type Response = <DomainCheck as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for DomainCreate {
    type Response = <DomainCreate as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for DomainDelete {
    type Response = <DomainDelete as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for DomainInfo {
    type Response = <DomainInfo as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for DomainRenew {
    type Response = <DomainRenew as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for DomainTransfer {
    type Response = <DomainTransfer as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for DomainUpdate {
    type Response = <DomainUpdate as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

// Host

impl Transaction<NameStore> for HostCheck {
    type Response = <HostCheck as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for HostCreate {
    type Response = <HostCreate as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for HostDelete {
    type Response = <HostDelete as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for HostInfo {
    type Response = <HostInfo as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

impl Transaction<NameStore> for HostUpdate {
    type Response = <HostUpdate as Transaction<NoExtension>>::Response;
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
    use crate::tests::{get_xml, CLTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/extensions/namestore.xml").unwrap();

        let namestore_ext = NameStore::new("com");

        let object = DomainCheck::new(vec!["example1.com", "example2.com", "example3.com"]);

        let serialized = <DomainCheck as Transaction<NameStore>>::serialize_request(
            object,
            Some(namestore_ext),
            CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/extensions/namestore.xml").unwrap();

        let object =
            <DomainCheck as Transaction<NameStore>>::deserialize_response(xml.as_str()).unwrap();

        let ext = object.extension.unwrap();

        assert_eq!(ext.data.subproduct, "com".into());
    }
}
