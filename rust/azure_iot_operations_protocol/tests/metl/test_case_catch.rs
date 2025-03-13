// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::collections::HashMap;

use azure_iot_operations_protocol::common::aio_protocol_error::AIOProtocolErrorKind;
use azure_iot_operations_protocol::telemetry::{TelemetryError, TelemetryErrorKind, Value};
use serde::Deserialize;

pub const HEADER_NAME_KEY: &str = "header-name";
pub const HEADER_VALUE_KEY: &str = "header-value";
pub const TIMEOUT_NAME_KEY: &str = "timeout-name";
pub const TIMEOUT_VALUE_KEY: &str = "timeout-value";
pub const PROPERTY_NAME_KEY: &str = "property-name";
pub const PROPERTY_VALUE_KEY: &str = "property-value";
pub const COMMAND_NAME_KEY: &str = "command-name";

#[derive(Clone, Deserialize, Debug)]
#[allow(dead_code)]
pub struct TestCaseCatch {
    #[serde(rename = "error-kind")]
    pub error_kind: String,

    #[serde(rename = "is-shallow")]
    pub is_shallow: Option<bool>,

    #[serde(rename = "is-remote")]
    pub is_remote: Option<bool>,

    #[serde(rename = "message")]
    pub message: Option<String>,

    #[serde(rename = "supplemental")]
    #[serde(default)]
    pub supplemental: HashMap<String, Option<String>>,
}

impl TestCaseCatch {
    // TODO: remove after RPCError implementation
    pub fn get_error_kind(&self) -> AIOProtocolErrorKind {
        match self.error_kind.as_str() {
            "missing header" => AIOProtocolErrorKind::HeaderMissing,
            "invalid header" => AIOProtocolErrorKind::HeaderInvalid,
            "invalid payload" => AIOProtocolErrorKind::PayloadInvalid,
            "timeout" => AIOProtocolErrorKind::Timeout,
            "cancellation" => AIOProtocolErrorKind::Cancellation,
            "invalid configuration" => AIOProtocolErrorKind::ConfigurationInvalid,
            "invalid state" => AIOProtocolErrorKind::StateInvalid,
            "internal logic error" => AIOProtocolErrorKind::InternalLogicError,
            "unknown error" => AIOProtocolErrorKind::UnknownError,
            "execution error" => AIOProtocolErrorKind::ExecutionException,
            "mqtt error" => AIOProtocolErrorKind::ClientError,
            "unsupported version" => AIOProtocolErrorKind::UnsupportedVersion,
            _ => panic!("Unrecognized error kind"),
        }
    }

    pub fn check_telemetry_error(&self, error: &TelemetryError) {
        // Kind + Kind Values
        match error.kind() {
            TelemetryErrorKind::PayloadInvalid => {
                assert_eq!(self.error_kind.as_str(), "invalid payload");
            }
            TelemetryErrorKind::ConfigurationInvalid {
                property_name,
                property_value,
            } => {
                assert_eq!(self.error_kind.as_str(), "invalid configuration");

                // TODO: investigate optionality, and if it can be reduced
                if let Some(Some(expected_property_name)) = self.supplemental.get(PROPERTY_NAME_KEY)
                {
                    check_telemetry_property_name(expected_property_name, property_name);
                }

                if let Some(Some(expected_property_value)) =
                    self.supplemental.get(PROPERTY_VALUE_KEY)
                {
                    check_telemetry_property_value(expected_property_value, property_value);
                }
            }
            TelemetryErrorKind::ArgumentInvalid {
                property_name,
                property_value,
            } => {
                assert_eq!(self.error_kind.as_str(), "invalid argument");
                check_telemetry_property_name(
                    self.supplemental
                        .get(PROPERTY_NAME_KEY)
                        .unwrap()
                        .as_ref()
                        .unwrap(),
                    property_name,
                );
                check_telemetry_property_value(
                    self.supplemental
                        .get(PROPERTY_VALUE_KEY)
                        .unwrap()
                        .as_ref()
                        .unwrap(),
                    property_value,
                );
            }
            TelemetryErrorKind::StateInvalid {
                property_name,
                property_value,
            } => {
                assert_eq!(self.error_kind.as_str(), "invalid state");
                check_telemetry_property_name(
                    self.supplemental
                        .get(PROPERTY_NAME_KEY)
                        .unwrap()
                        .as_ref()
                        .unwrap(),
                    property_name,
                );
                if let Some(property_value) = property_value {
                    check_telemetry_property_value(
                        self.supplemental
                            .get(PROPERTY_VALUE_KEY)
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        property_value,
                    );
                }
            }
            TelemetryErrorKind::InternalLogicError {
                property_name,
                property_value,
            } => {
                assert_eq!(self.error_kind.as_str(), "internal logic error");
                check_telemetry_property_name(
                    self.supplemental
                        .get(PROPERTY_NAME_KEY)
                        .unwrap()
                        .as_ref()
                        .unwrap(),
                    property_name,
                );
                if let Some(property_value) = property_value {
                    check_telemetry_property_value(
                        self.supplemental
                            .get(PROPERTY_VALUE_KEY)
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        property_value,
                    );
                }
            }
            TelemetryErrorKind::UnknownError => {
                assert_eq!(self.error_kind.as_str(), "unknown error");
            }
            TelemetryErrorKind::MqttError => {
                assert_eq!(self.error_kind.as_str(), "mqtt error");
            }
            _ => panic!("Unrecognized error kind"),
        }

        // Is Shallow
        if error.is_shallow() {
            assert_eq!(self.is_shallow, Some(true));
        } else {
            assert_eq!(self.is_shallow, Some(false));
        }

        // Is Remote (always false for Telemetry)
        assert_eq!(self.is_remote, Some(false));
    }
}

fn format_property_name(property_name: &str) -> String {
    property_name
        .chars()
        .skip(if let Some(ix) = property_name.rfind('.') {
            ix + 1
        } else {
            0
        })
        .collect::<String>()
        .replace("__", ".")
        .replace('_', "")
        .replace('.', "__")
        .to_lowercase()
}

fn check_telemetry_property_name(expected_name_string: &String, actual_name: &str) {
    assert_eq!(expected_name_string, &format_property_name(actual_name));
}

fn check_telemetry_property_value(expected_value_string: &String, actual_value: &Value) {
    match actual_value {
        Value::Integer(int_value) => {
            assert_eq!(expected_value_string, &int_value.to_string());
        }
        Value::Float(float_value) => {
            assert_eq!(expected_value_string, &float_value.to_string());
        }
        Value::String(string_value) => assert_eq!(expected_value_string, string_value),
        Value::Boolean(bool_value) => {
            assert_eq!(expected_value_string, &bool_value.to_string());
        }
    };
}
