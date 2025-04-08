use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArtifactoryCellInfo {
    #[serde(skip_serializing)]
    pub pid: i32,
    pub name: String,
    pub ver: String,
    pub descript: String,
    pub md5: Option<String>,
    pub cont_size: Option<i64>,
    pub url: Option<String>,
    pub runtime: String,
    pub min_runtime_ver: Option<i32>,
    pub max_runtime_ver: Option<i32>,
    pub tag: Option<serde_json::Value>,
    pub is_private:Option<bool>
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
