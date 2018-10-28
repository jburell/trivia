use actix_web::{HttpResponse, Query, Result};
use std::collections::HashMap;

pub fn get_from_ws(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    debug!("=========={:?}=========", query);
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("{[[:ev-id, 2]]}"))
        //.body("{[\":ev-id\", [1, 2, 3]]}"))
}

pub fn options(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    debug!("=========={:?}=========", query);
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("(msg \"hello\")."))
}
