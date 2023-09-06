// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Clone, Deserialize, Serialize)]
pub enum SignablePayloadStatus {
    #[serde(rename = "CREATED")]
    Created,

    #[serde(rename = "SIGNED")]
    Signed,
}

impl From<SignablePayloadStatus> for Value {
    fn from(val: SignablePayloadStatus) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for SignablePayloadStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Created => write!(f, "CREATED"),
            Self::Signed => write!(f, "SIGNED"),
        }
    }
}
