/* Code generated by Azure.Iot.Operations.ProtocolCompiler v0.6.0.0; DO NOT EDIT. */
#![allow(non_camel_case_types)]

use azure_iot_operations_protocol::common::payload_serialize::{FormatIndicator, PayloadSerialize};
use serde_json;

use super::get_request_payload::GetRequestPayload;

impl PayloadSerialize for GetRequestPayload {
    type Error = serde_json::Error;

    fn content_type() -> &'static str {
        "application/json"
    }

    fn format_indicator() -> FormatIndicator {
        FormatIndicator::Utf8EncodedCharacterData
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(self)
    }

    fn deserialize(payload: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(payload)
    }
}
