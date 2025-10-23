use serde::{self, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArtifactoryCellInfo {
    #[serde(skip_deserializing)]
    pub pid: i32,
    pub name: String,
    pub ver: String,
    pub md5: String,
    pub descript: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<serde_json::Value>,
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
    pub ci_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_raw: Option<bool>,
}

impl TryFrom<serde_json::Value> for ArtifactoryCellInfo {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}

impl TryFrom<serde_yaml::Value> for ArtifactoryCellInfo {
    type Error = serde_yaml::Error;

    fn try_from(value: serde_yaml::Value) -> Result<Self, Self::Error> {
        serde_yaml::from_value(value)
    }
}

pub struct AddRuntime<'r> {
    pub runtime_name: &'r str,
    pub runtime_ver: &'r str,
}

#[derive(Serialize)]
pub struct Runtime<'r> {
    pub runtime: &'r str,
    pub ver_str: &'r str,
}
