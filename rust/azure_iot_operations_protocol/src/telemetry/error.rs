// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::time::Duration;
use std::fmt;

use azure_iot_operations_mqtt::error::{PublishError, SubscribeError, UnsubscribeError, CompletionError};
use crate::common::hybrid_logical_clock::{HLCError, HLCErrorKind};

/// An error that occurred during a Telemetry operation
#[derive(Debug)]
pub struct TelemetryError {
    /// The kind of error that occurred
    kind: TelemetryErrorKind,
    /// Source of the error
    source: Box<dyn std::error::Error>,
    /// Indicates whether the error was detected prior to attempted network communication
    is_shallow: bool,
}

// TODO: should these impl a Protocol Error trait so Telemetry and Command have same interface?
impl TelemetryError {
    pub fn new<E>(kind: TelemetryErrorKind, source: E, is_shallow: bool) -> Self 
    where 
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        TelemetryError {
            kind,
            source: source.into(),
            is_shallow,
        }
    }

    /// Returns the corresponding [`TelemetryErrorKind`] for this error
    pub fn kind(&self) -> &TelemetryErrorKind {
        &self.kind
    }

    /// Indicates whether the error was detected prior to attempted network communication
    pub fn is_shallow(&self) -> bool {
        self.is_shallow
    }
}

impl std::fmt::Display for TelemetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for TelemetryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}

impl From<PublishError> for TelemetryError {
    // TODO: Make this return a ClientError kind once spec supports it
    fn from(error: PublishError) -> Self {
        //TelemetryError::new(TelemetryErrorKind::ClientError, error, true)
        TelemetryError::new(TelemetryErrorKind::UnknownError, error, true)
    }
}

impl From<SubscribeError> for TelemetryError {
    // TODO: Make this return a ClientError kind once spec supports it
    fn from(error: SubscribeError) -> Self {
        //TelemetryError::new(TelemetryErrorKind::ClientError, error, true)
        TelemetryError::new(TelemetryErrorKind::UnknownError, error, true)
    }
}

impl From<UnsubscribeError> for TelemetryError {
    // TODO: Make this return a ClientError kind once spec supports it
    fn from(error: UnsubscribeError) -> Self {
        //TelemetryError::new(TelemetryErrorKind::ClientError, error, true)
        TelemetryError::new(TelemetryErrorKind::UnknownError, error, true)

    }
}

impl From<CompletionError> for TelemetryError {
    fn from(error: CompletionError) -> Self {
        TelemetryError::new(TelemetryErrorKind::MqttError, error, false)
    }
}

impl From<HLCError> for TelemetryError {
    fn from(error: HLCError) -> Self {
        match error.kind() {
            HLCErrorKind::ClockDrift => TelemetryError::new(TelemetryErrorKind::StateInvalid { property_name: "MaxClockDrift".into(), property_value: None }, error, true),
            HLCErrorKind::OverflowWarning => TelemetryError::new(TelemetryErrorKind::InternalLogicError { property_name: "Counter".into(), property_value: None }, error, true),
        }
    }
}

/// The kind of Telemetry error
#[derive(Debug)]
#[non_exhaustive]
pub enum TelemetryErrorKind {
    /// A required MQTT header property is missing from a received message  //TODO: received only???
    HeaderMissing {
        /// The name of the MQTT header that is missing
        header_name: String
    },
    /// An MQTT header property has an invalid value        // TODO: can we remove MQTT information?
    HeaderInvalid {
        header_name: String,
        header_value: String,
    },
    /// Payload cannot be serialized/deserialized
    PayloadInvalid,
    /// An operation was aborted due to timeout
    Timeout {
        timeout_name: String,
        timeout_value: Duration,
    },
    /// An operation was cancelled
    Cancellation,
    /// A field, configuration file, or environment variable has an invalid value
    ConfigurationInvalid {
        property_name: String,
        property_value: Value,
    },
    /// An invalid argument was provided to a function or method
    ArgumentInvalid {
        property_name: String,
        property_value: Value,
    },
    /// The current program state is invalid vis-a-vis the function or method that was called
    StateInvalid {
        property_name: String,
        property_value: Option<Value>,
    },
    /// The client or service observed a condition that was thought to be impossible
    InternalLogicError {
        property_name: String,
        property_value: Option<Value>,
    },
    /// The client or service received an unexpected error from a dependent component
    UnknownError,
    /// The network communication protocol reported an error
    MqttError,
}

impl fmt::Display for TelemetryErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TelemetryErrorKind::HeaderMissing { header_name } => write!(f, "Header '{}' is missing", header_name),
            TelemetryErrorKind::HeaderInvalid { header_name, header_value } => write!(f, "Header '{}' has invalid value '{}'", header_name, header_value),
            TelemetryErrorKind::PayloadInvalid => write!(f, "Payload is invalid"),
            TelemetryErrorKind::Timeout { timeout_name, timeout_value } => write!(f, "Operation timed out after {} {}", timeout_value.as_secs(), timeout_name),
            TelemetryErrorKind::Cancellation => write!(f, "Operation was cancelled"),
            TelemetryErrorKind::ConfigurationInvalid { property_name, property_value } => write!(f, "Configuration property '{}' has invalid value '{:?}'", property_name, property_value),
            TelemetryErrorKind::ArgumentInvalid { property_name, property_value } => write!(f, "Argument '{}' has invalid value '{:?}'", property_name, property_value),
            TelemetryErrorKind::StateInvalid { property_name, property_value } => write!(f, "State property '{}' has invalid value '{:?}'", property_name, property_value),
            TelemetryErrorKind::InternalLogicError { property_name, property_value } => write!(f, "Internal logic error: property '{}' has invalid value '{:?}'", property_name, property_value),
            TelemetryErrorKind::UnknownError => write!(f, "Unknown error"),
            TelemetryErrorKind::MqttError => write!(f, "MQTT error"),
        }
    }
}

/// Represents the possible types of the value of a property
#[derive(Debug, PartialEq)]
pub enum Value {
    /// A 32-bit integer value
    Integer(i32),
    /// A 64-bit floating point value
    Float(f64),
    /// A String value
    String(String),
    /// A bool value
    Boolean(bool),
}
