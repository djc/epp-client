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

#[cfg(test)]
mod tests {
    use super::Logout;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SVTRID};

    #[test]
    fn logout() {
        let xml = get_xml("response/logout.xml").unwrap();
        let object = Logout::deserialize_response(xml.as_str()).unwrap();

        assert_eq!(object.result.code, 1500);
        assert_eq!(
            object.result.message,
            "Command completed successfully; ending session".into()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
