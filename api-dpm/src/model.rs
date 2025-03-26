use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ModelDpm {
    pub name: String,
    pub ver: String,
    pub md5: String,
    pub descript: String,
    pub license: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub cont_size: i64,
    pub runtime: String, 
    pub scripts: Option<HashMap<String, String>>
}

impl ModelDpm {
    ///设置包的md5
    pub fn set_md5(&mut self, md5: String) {
        self.md5 = md5;
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
