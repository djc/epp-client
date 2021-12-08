//! Types for EPP consolidate request

use std::fmt;

use chrono::FixedOffset;
use epp_client_macros::ElementName;
use serde::{Deserialize, Serialize};

use crate::{
    common::{ElementName, NoExtension, StringValue, Extension},
    domain::update::DomainUpdate,
    request::Transaction,
};

use super::namestore::NameStore;

pub const XMLNS: &str = "http://www.verisign.com/epp/sync-1.0";

impl Transaction<Sync> for DomainUpdate {
    type ExtensionWrapper = Extension<Sync>;
    type Response = <DomainUpdate as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NoExtension;
}

impl Transaction<SyncWithNameStore> for DomainUpdate {
    type ExtensionWrapper = Wrapper;
    type Response = <DomainUpdate as Transaction<NoExtension>>::Response;
    type ExtensionResponse = NameStore;
}

#[derive(PartialEq, Debug)]
pub struct GMonthDay {
    pub month: u8,
    pub day: u8,
    pub timezone: Option<FixedOffset>,
}

// Taken from https://github.com/lumeohq/xsd-parser-rs/blob/main/xsd-types/src/types/gmonthday.rs
/// Represents a gMonthDay type https://www.w3.org/TR/xmlschema-2/#gMonthDay
impl GMonthDay {
    pub fn new(month: u8, day: u8, timezone: Option<FixedOffset>) -> Result<Self, String> {
        if !(1..=12).contains(&month) {
            return Err("Month value within GMonthDay should lie between 1 and 12".to_string());
        }

        if !(1..=31).contains(&day) {
            return Err("Day value within GMonthDay should lie between 1 and 31".to_string());
        }

        const MONTH_MAX_LEN: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if day > MONTH_MAX_LEN[month as usize - 1] {
            return Err("Day value within GMonthDay is to big for specified month".to_string());
        }

        Ok(GMonthDay {
            month,
            day,
            timezone,
        })
    }
}

impl fmt::Display for GMonthDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.timezone {
            Some(tz) => write!(f, "--{:02}-{:02}{}", self.month, self.day, tz),
            None => write!(f, "--{:02}-{:02}", self.month, self.day),
        }
    }
}

impl Sync {
    /// Create a new RGP restore report request
    pub fn new(expiration: GMonthDay) -> Self {
        Self {
            xmlns: XMLNS.to_string(),
            exp: expiration.to_string().into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[serde(rename = "sync:update")]
/// Type for EPP XML &lt;consolidate&gt; extension
pub struct Sync {
    /// XML namespace for the consolidate extension
    #[serde(rename = "xmlns:sync", alias = "xmlns")]
    pub xmlns: String,
    /// The expiry date of the domain
    #[serde(rename = "sync:expMonthDay", alias = "sync")]
    pub exp: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "extension")]
pub struct SyncWithNameStore {
    #[serde(rename = "sync:update")]
    pub sync: Sync,
    #[serde(rename = "namestoreExt:namestoreExt")]
    pub namestore: NameStore,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Wrapper(SyncWithNameStore);

impl From<SyncWithNameStore> for Wrapper {
    fn from(inner: SyncWithNameStore) -> Self {
        Self(inner)
    }
}

#[cfg(test)]
mod tests {
    use super::{GMonthDay, Sync, SyncWithNameStore};
    use crate::domain::update::{DomainChangeInfo, DomainUpdate};
    use crate::extensions::namestore::NameStore;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/extensions/consolidate.xml").unwrap();

        let exp = GMonthDay::new(5, 31, None).unwrap();

        let consolidate_ext = Sync::new(exp);

        let mut object = DomainUpdate::new("eppdev.com");

        object.info(DomainChangeInfo {
            registrant: None,
            auth_info: None,
        });

        let serialized = <DomainUpdate as Transaction<Sync>>::serialize_request(
            object,
            Some(consolidate_ext),
            CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn consolidate_namestore() {
        let xml = get_xml("request/extensions/consolidate_namestore.xml").unwrap();

        let exp = GMonthDay::new(1, 15, None).unwrap();

        let consolidate_ext = Sync::new(exp);
        let namestore_ext = NameStore::new("com");

        let ext = SyncWithNameStore {
            sync: consolidate_ext,
            namestore: namestore_ext,
        };

        let mut object = DomainUpdate::new("eppdev.com");

        object.info(DomainChangeInfo {
            registrant: None,
            auth_info: None,
        });

        let serialized = <DomainUpdate as Transaction<SyncWithNameStore>>::serialize_request(
            object,
            Some(ext),
            CLTRID,
        )
        .unwrap();

        similar_asserts::assert_eq!(xml, serialized);
    }
}
