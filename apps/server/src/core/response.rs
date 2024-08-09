use actix_web::{http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};

use super::constants::MESSAGE_SUCCESS;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    status: String,
    message: String,
}

impl Diagnostic {
    pub fn new(status: &str, message: &str) -> Diagnostic {
        Diagnostic {
            status: status.to_string(),
            message: message.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody<T> {
    pub diagnostic: Diagnostic,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> From<ResponseBody<T>> for HttpResponse
where
    T: Serialize,
{
    fn from(val: ResponseBody<T>) -> Self {
        HttpResponse::Ok().json(val)
    }
}

impl<T> ResponseBody<T> {
    pub fn new(diagnostic: Diagnostic, data: Option<T>) -> ResponseBody<T> {
        let data = data;

        ResponseBody { diagnostic, data }
    }

    pub fn success(data: Option<T>) -> ResponseBody<T> {
        let data = data;

        ResponseBody {
            diagnostic: Diagnostic::new(&StatusCode::OK.as_u16().to_string(), MESSAGE_SUCCESS),
            data,
        }
    }
}
