// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::fmt;
use std::time::Duration;

use crate::common::hybrid_logical_clock::{HLCError, HLCErrorKind};
use azure_iot_operations_mqtt::error::{
    CompletionError, PublishError, SubscribeError, UnsubscribeError,
};

/// An error that occurred during a Telemetry operation
#[derive(Debug)]
pub struct TelemetryError {
    /// The kind of error that occurred
    kind: TelemetryErrorKind,
    /// Source of the error
    source: Box<dyn std::error::Error + Send + Sync>,
    /// Indicates whether the error was detected prior to attempted network communication
    is_shallow: bool,
}

impl TelemetryError {
    /// Creates a new [`TelemetryError`]
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
    #[must_use]
    pub fn kind(&self) -> &TelemetryErrorKind {
        &self.kind
    }

    /// Indicates whether the error was detected prior to attempted network communication
    #[must_use]
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
            HLCErrorKind::ClockDrift => TelemetryError::new(
                TelemetryErrorKind::StateInvalid {
                    property_name: "MaxClockDrift".into(),
                    property_value: None,
                },
                error,
                true,
            ),
            HLCErrorKind::OverflowWarning => TelemetryError::new(
                TelemetryErrorKind::InternalLogicError {
                    property_name: "Counter".into(),
                    property_value: None,
                },
                error,
                true,
            ),
        }
    }
}

/// The kind of Telemetry error
#[derive(Debug)]
#[non_exhaustive]
pub enum TelemetryErrorKind {
    /// Payload cannot be serialized/deserialized
    PayloadInvalid,
    /// A field, configuration file, or environment variable has an invalid value
    ConfigurationInvalid {
        /// Description of which field, configuration file, or environment variable has an invalid value
        property_name: String,
        /// The value of the field, configuration file, or environment variable that is invalid
        property_value: Value,
    },
    /// An invalid argument was provided to a function or method
    ArgumentInvalid {
        /// The name of the argument that has an invalid value
        property_name: String,
        /// The value of the argument that is invalid
        property_value: Value,
    },
    /// The current program state is invalid vis-a-vis the function or method that was called
    StateInvalid {
        /// Description of which area of the program state became invalid
        property_name: String,
        /// Associated value (if any) with the invalid state
        property_value: Option<Value>,
    },
    /// The client or service observed a condition that was thought to be impossible
    InternalLogicError {
        /// Description of which area of the program encountered an internal logic error
        property_name: String,
        /// Associated value (if any) with the internal logic error
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
            TelemetryErrorKind::PayloadInvalid => write!(f, "Payload is invalid"),
            TelemetryErrorKind::ConfigurationInvalid {
                property_name,
                property_value,
            } => write!(
                f,
                "Configuration property '{property_name}' has invalid value '{property_value:?}'",
            ),
            TelemetryErrorKind::ArgumentInvalid {
                property_name,
                property_value,
            } => write!(
                f,
                "Argument '{property_name}' has invalid value '{property_value:?}'",
            ),
            TelemetryErrorKind::StateInvalid {
                property_name,
                property_value,
            } => write!(
                f,
                "State property '{property_name}' has invalid value '{property_value:?}'",
            ),
            TelemetryErrorKind::InternalLogicError {
                property_name,
                property_value,
            } => write!(
                f,
                "Internal logic error: property '{property_name}' has invalid value '{property_value:?}'",
            ),
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
