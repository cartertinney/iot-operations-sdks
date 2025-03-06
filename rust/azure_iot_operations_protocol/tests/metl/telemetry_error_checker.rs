// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use azure_iot_operations_protocol::telemetry::{TelemetryError, TelemetryErrorKind}; //Value};
use iso8601_duration::Duration;

use crate::metl::test_case_catch;

pub fn check_error(
    test_case_catch: &test_case_catch::TestCaseCatch,
    telemetry_error: &TelemetryError,
) {

}