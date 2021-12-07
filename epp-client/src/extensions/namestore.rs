//! Types for EPP namestore request and responses

use epp_client_macros::ElementName;
use serde::{Deserialize, Serialize};

use crate::{
    common::{ElementName, StringValue},
    request::EppExtension,
};

pub const XMLNS: &str = "http://www.verisign-grs.com/epp/namestoreExt-1.1";

impl NameStore {
    /// Create a new RGP restore report request
    pub fn new(subproduct: &str) -> NameStore {
        NameStore {
            xmlns: XMLNS.to_string(),
            subproduct: subproduct.into(),
        }
    }
}

impl EppExtension for NameStore {
    type Response = NameStore;
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
