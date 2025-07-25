/// 下载方式：
/// 通过schema区分
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub enum EFileSchema {
    #[default]
    S3,
    Http,
    Nexus,
    Unknown,
}

impl From<&str> for EFileSchema {
    fn from(value: &str) -> Self {
        match value {
            "http" => EFileSchema::Http,
            "s3" => EFileSchema::S3,
            "nexus" => EFileSchema::Nexus,
            _ => EFileSchema::Unknown,
        }
    }
}

impl EFileSchema {
    pub fn is_no_progress(schema: EFileSchema) -> bool {
        schema == EFileSchema::S3
    }
}

pub mod http;
pub mod nexus;
pub mod s3;
