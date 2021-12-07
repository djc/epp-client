//! Types for EPP host create request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, HostAddr, NoExtension, StringValue};
use crate::request::Transaction;
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for HostCreate {
    type Response = HostCreateResponse;
    type ExtensionResponse = NoExtension;
}

impl HostCreate {
    pub fn new(host: &str, addresses: Vec<HostAddr>) -> Self {
        Self {
            host: HostCreateRequestData {
                xmlns: XMLNS.to_string(),
                name: host.into(),
                addresses: Some(addresses),
            },
        }
    }
}

// Request

/// Type for data under the host &lt;create&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCreateRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// The name of the host to be created
    #[serde(rename = "host:name", alias = "name")]
    pub name: StringValue,
    /// The list of IP addresses for the host
    #[serde(rename = "host:addr", alias = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
/// Type for EPP XML &lt;create&gt; command for hosts
pub struct HostCreate {
    /// The instance holding the data for the host to be created
    #[serde(rename = "host:create", alias = "create")]
    host: HostCreateRequestData,
}

// Response

/// Type that represents the &lt;creData&gt; tag for host create response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCreateData {
    /// XML namespace for host response data
    #[serde(rename = "xmlns:host")]
    xmlns: String,
    /// The host name
    pub name: StringValue,
    /// The host creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCreateResponse {
    /// Data under the &lt;creData&gt; tag
    #[serde(rename = "creData")]
    pub create_data: HostCreateData,
}

#[cfg(test)]
mod tests {
    use super::HostCreate;
    use crate::common::HostAddr;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/host/create.xml").unwrap();

        let addresses = vec![
            HostAddr::new("v4", "29.245.122.14"),
            HostAddr::new("v6", "2404:6800:4001:801::200e"),
        ];

        let object = HostCreate::new("host1.eppdev-1.com", addresses);

        let serialized = object.serialize_request(None, CLTRID).unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/host/create.xml").unwrap();
        let object = HostCreate::deserialize_response(xml.as_str()).unwrap();

        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(result.create_data.name, "host2.eppdev-1.com".into());
        assert_eq!(
            result.create_data.created_at,
            "2021-07-26T05:28:55.0Z".into()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
