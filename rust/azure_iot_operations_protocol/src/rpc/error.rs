// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#[allow(dead_code)]
#[allow(missing_docs)]
#[allow(unused_variables)]

use std::time::Duration;
use azure_iot_operations_mqtt::error::{
    CompletionError, PublishError, SubscribeError, UnsubscribeError,
};

/// An error that occurred during an RPC operation
#[derive(Debug)]
pub struct RPCError {
    /// The kind of error that occurred
    kind: RPCErrorKind,
    /// Source of the error, if any
    source: Option<Box<dyn std::error::Error>>,
    /// Indicates whether the error was detected prior to attempted network communication
    is_shallow: bool,
    /// Command name
    /// TODO: do we want this Optional?
    command_name: Option<String>,
}

impl RPCError {
    pub fn new<E>(kind: RPCErrorKind, source: Option<E>, is_shallow: bool, command_name: String) -> Self
    where
        E: Into<Box<dyn std::error::Error>>,
    {
        RPCError {
            kind,
            source: source.and_then(|e| Some(e.into())),
            is_shallow,
            command_name: Some(command_name),
        }
    }

    /// Returns the corresponding [`RPCErrorKind`] for this error
    pub fn kind(&self) -> &RPCErrorKind {
        &self.kind
    }

    /// Indicates whether the error was detected by a remote component
    pub fn is_remote(&self) -> bool {
        self.source
            .as_ref()
            .is_some_and(|e| e.downcast_ref::<RemoteError>().is_some())
    }

    /// Indicates whether the error was detected prior to attempted network communication
    pub fn is_shallow(&self) -> bool {
        self.is_shallow
    }
}

impl std::fmt::Display for RPCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{}", self.kind)
        unimplemented!()
    }
}

impl std::error::Error for RPCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

/// The kind of RPC error
#[derive(Debug)]
pub enum RPCErrorKind {
    /// A required MQTT header property is missing from a received message  //TODO: received only???
    HeaderMissing {
        /// The name of the MQTT header that is missing
        header_name: String,
    },
    /// An MQTT header property has an invalid value        // TODO: can we remove MQTT information?
    HeaderInvalid {
        /// The name of the MQTT header that has an invalid value
        header_name: String,
        /// The value of the MQTT header that is invalid
        header_value: String,
    },
    /// Payload cannot be serialized/deserialized
    PayloadInvalid,
    /// An operation was aborted due to timeout
    Timeout {
        /// The name of the timeout condition that elapsed
        timeout_name: String,
        /// The duration of the timeout condition that elapsed
        timeout_value: Duration,
    },
    /// An operation was cancelled
    Cancellation,
    /// A field, configuration file, or environment variable has an invalid value
    ConfigurationInvalid {
        /// The name of the field, configuration file, or environment variable that has an invalid value
        property_name: String,
        /// The value of the field, configuration file, or environment variable that is invalid
        property_value: Value,
    },
    /// The current program state is invalid vis!vis the function or method that was called
    StateInvalid {
        // TODO:
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
    /// The remote command executor identified an error in the request
    InvocationError {
        property_name: Option<String>,
        property_value: Option<Value>,
    },
    /// The remote command executor encountered an error while executing the command
    ExecutionError {
        property_name: Option<String>,
        property_value: Option<Value>,
    },
    /// The network communication encountered an error and failed
    MqttError,
    /// A request or response was received containing a protocol version that is not supported
    UnsupportedVersion {
        protocol_version: String,
        supported_major_versions: Vec<u16>,
    },
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

/// Represents an error reported by a remote executor
#[derive(thiserror::Error, Debug)]
#[error("Remote Error status code: {http_status_code:?}")]
pub struct RemoteError {
    /// The message received with the error
    message: Option<String>,
    

    /// Status code received from a remote service that caused the error
    http_status_code: Option<u16>,
}
