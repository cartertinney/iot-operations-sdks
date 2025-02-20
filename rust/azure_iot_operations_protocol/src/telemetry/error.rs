// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::time::Duration;

/// An error that occurred during a Telemetry operation
#[derive(Debug)]
pub struct TelemetryError {
    /// The kind of error that occurred
    kind: TelemetryErrorKind,
    /// Source of the error, if any
    source: Option<Box<dyn std::error::Error>>,
    /// Indicates whether the error was detected prior to attempted network communication
    is_shallow: bool,
}

// TODO: should these impl a Protocol Error trait so Telemetry and Command have same interface?
impl TelemetryError {
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
        // TODO: complete
        write!(f, "temp")
    }
}

impl std::error::Error for TelemetryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

/// The kind of Telemetry error
#[derive(Debug)]
pub enum TelemetryErrorKind {
    /// A required MQTT header property is missing from a received message  //TODO: received only???
    HeaderMissing { header_name: String },
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
    /// The current program state is invalid vis!vis the function or method that was called
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
    /// The network communication encountered an error and failed
    ClientError,
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
