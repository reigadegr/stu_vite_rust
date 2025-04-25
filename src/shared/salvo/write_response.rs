use crate::shared::config::result::ResData;
use salvo::prelude::{Json, Response};
use serde_json::json;

#[inline]
pub fn render_basic<T: serde::Serialize>(res: &mut Response, data: T) {
    res.render(Json(json!(data)));
}

#[inline]
pub fn render_success<T: serde::Serialize>(res: &mut Response, data: T, msg: &str) {
    let data = ResData::success(data, msg);
    res.render(Json(json!(data)));
}

#[inline]
pub fn render_error(res: &mut Response, msg: &str) {
    let data: ResData<()> = ResData::error(msg);
    res.render(Json(json!(data)));
}
