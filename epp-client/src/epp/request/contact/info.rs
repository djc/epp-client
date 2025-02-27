//! Types for EPP contact info request

use epp_client_macros::*;

use crate::epp::object::data::AuthInfo;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_CONTACT_XMLNS;
use serde::{Deserialize, Serialize};

/// Type for the &lt;epp&gt; request for contact &lt;info&gt; command
///
/// ## Usage
///
/// ```ignore
/// use epp_client::EppClient;
/// use epp_client::epp::{EppContactInfo, EppContactInfoResponse};
/// use epp_client::epp::generate_client_tr_id;
///
/// #[tokio::main]
/// async fn main() {
///     // Create an instance of EppClient, specifying the name of the registry as in
///     // the config file
///     let mut client = match EppClient::new("verisign").await {
///         Ok(client) => client,
///         Err(e) => panic!("Failed to create EppClient: {}",  e)
///     };
///
///     // Create an EppContactInfo instance
///     let contact_info = EppContactInfo::new(
///         "eppdev-contact-100",
///         "epP4uthd#v",
///         generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppContactInfoResponse
///     let response = client.transact::<_, EppContactInfoResponse>(&contact_info).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppContactInfo = EppObject<Command<ContactInfo>>;

/// Type for elements under the contact &lt;info&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactInfoData {
    /// XML namespace for contact commands
    xmlns: String,
    /// The contact id for the info command
    id: StringValue,
    /// The &lt;authInfo&gt; data
    #[serde(rename = "authInfo")]
    auth_info: AuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
/// Type for EPP XML &lt;info&gt; command for contacts
pub struct ContactInfo {
    /// Data for &lt;info&gt; command for contact
    #[serde(rename = "info")]
    info: ContactInfoData,
}

impl EppContactInfo {
    /// Creates a new EppObject for contact info corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(id: &str, auth_password: &str, client_tr_id: &str) -> EppContactInfo {
        let contact_info = ContactInfo {
            info: ContactInfoData {
                xmlns: EPP_CONTACT_XMLNS.to_string(),
                id: id.to_string_value(),
                auth_info: AuthInfo::new(auth_password),
            },
        };

        EppObject::build(Command::<ContactInfo>::new(contact_info, client_tr_id))
    }
}
