use serde::{self, Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NexusBase {
    #[serde(skip_deserializing)]
    pub pid: i32,
    pub name: String,
    pub ver: String,
    pub md5: String,
    pub descript: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Value>,
    pub cont_size: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_runtime_ver: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_runtime_ver: Option<i32>,
    pub runtime: String,
    pub s3_key: String,
    pub s3_inc_id: i32,
    pub is_artifactory_ready: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ci_info: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Raw {
    core: NexusBase,
}
