// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use serde::Deserialize;

use crate::metl::default_serializer::DefaultSerializer;

#[derive(Deserialize, Debug)]
pub struct DefaultReceiver {
    #[serde(rename = "serializer")]
    pub serializer: Option<DefaultSerializer>,

    #[serde(rename = "telemetry-topic")]
    pub telemetry_topic: Option<String>,

    #[serde(rename = "topic-namespace")]
    pub topic_namespace: Option<String>,
}
