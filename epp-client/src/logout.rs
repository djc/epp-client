use std::fmt::Debug;

use epp_client_macros::ElementName;
use serde::{Deserialize, Serialize};

use crate::{
    common::{ElementName, NoExtension},
    request::Transaction,
};

impl Transaction<NoExtension> for Logout {
    type Response = ();
    type ExtensionResponse = NoExtension;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "logout")]
/// Type corresponding to the &lt;logout&gt; tag in an EPP XML logout request
pub struct Logout;
