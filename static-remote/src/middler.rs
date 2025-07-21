use crate::{error::ApiReqError, RespVO};

/// 判断是否返回data数据
pub fn map_respvo_data<T>(vo: RespVO<T>) -> Result<T, ApiReqError> {
    if vo.code != 0 {
        return Err(ApiReqError::RespError(vo.msg, vo.code));
    }
    if vo.data.is_none() {
        return Err(ApiReqError::RespError("data is none".to_string(), 999));
    }
    Ok(vo.data.expect(""))
}

/// 判断是否返回code错误
pub fn map_respvo_iserror<T>(vo: RespVO<T>) -> bool {
    vo.code != 0
}

/// 判断是否返回code错误，并返回resp本身
pub fn map_respvo_iserror_vo<T>(vo: RespVO<T>) -> Result<(bool, RespVO<T>), ApiReqError> {
    Ok((vo.code == 0, vo))
}
