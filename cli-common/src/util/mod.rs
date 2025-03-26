use chrono::{DateTime, Utc};

pub mod filesys;
pub mod shcmd;

///获取格式化时间戳
pub fn get_strfmt_timestr<'a>(fmtstr: &str) -> String {
    let now: DateTime<Utc> = Utc::now();
    return now.format(fmtstr).to_string();
}