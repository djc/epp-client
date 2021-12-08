//! Types for EPP requests

use serde::{de::DeserializeOwned, ser::SerializeStruct, ser::Serializer, Deserialize, Serialize};
use std::fmt::Debug;

use crate::{
    common::{ElementName, EppObject, StringValue},
    response::{Response, ResponseStatus},
    xml::EppXml,
};
use epp_client_macros::ElementName;

pub const EPP_VERSION: &str = "1.0";
pub const EPP_LANG: &str = "en";

/// Trait to set correct value for xml tags when tags are being generated from generic types
pub trait Transaction<Ext: ElementName + DeserializeOwned + Serialize + Sized + Debug>:
    ElementName + DeserializeOwned + Serialize + Sized + Debug
{
    type ExtensionWrapper: From<Ext> + Serialize + DeserializeOwned + Debug;
    type Response: DeserializeOwned + Serialize + Debug;
    type ExtensionResponse: ElementName + DeserializeOwned + Serialize + Debug;

    fn serialize_request(
        self,
        extension: Option<Ext>,
        client_tr_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        EppXml::serialize(&EppObject::build(Command {
            command: self,
            extension: extension.map(<Self::ExtensionWrapper as From<Ext>>::from),
            client_tr_id: client_tr_id.into(),
        }))
    }

    fn deserialize_response(
        epp_xml: &str,
    ) -> Result<Response<Self::Response, Self::ExtensionResponse>, crate::error::Error> {
        let rsp =
            <EppObject<Response<Self::Response, Self::ExtensionResponse>> as EppXml>::deserialize(
                epp_xml,
            )?;
        match rsp.data.result.code {
            0..=2000 => Ok(rsp.data),
            _ => Err(crate::error::Error::EppCommandError(ResponseStatus {
                result: rsp.data.result,
                tr_ids: rsp.data.tr_ids,
            })),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "command")]
/// Type corresponding to the &lt;command&gt; tag in an EPP XML request
/// with an &lt;extension&gt; tag
pub struct Command<C, W> {
    /// The instance that will be used to populate the &lt;command&gt; tag
    pub command: C,
    /// The client TRID
    pub extension: Option<W>,
    #[serde(rename = "clTRID")]
    pub client_tr_id: StringValue,
}

impl<C: ElementName + Serialize, W: Serialize> Serialize for Command<C, W> {
    /// Serializes the generic type T to the proper XML tag (set by the `#[element_name(name = <tagname>)]` attribute) for the request
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("command", 3)?;
        state.serialize_field(C::ELEMENT, &self.command)?;
        state.serialize_field("extension", &self.extension)?;
        state.serialize_field("clTRID", &self.client_tr_id)?;
        state.end()
    }
}
