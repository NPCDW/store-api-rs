use actix_web::{HttpResponse};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
pub struct ResponseResult<T> {
    pub success: bool,
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ResponseResult<T> {
    #[allow(dead_code)]
    pub fn ok_data(data: T) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        Ok(HttpResponse::Ok().json(ResponseResult::<T> {
            success: true,
            code: 20000,
            message: "success".to_string(),
            data: Some(data),
        }))
    }
}

impl ResponseResult<usize> {
    #[allow(dead_code)]
    pub fn ok() -> Result<HttpResponse, Box<dyn std::error::Error>> {
        Ok(HttpResponse::Ok().json(ResponseResult::<usize> {
            success: true,
            code: 20000,
            message: "success".to_string(),
            data: None,
        }))
    }
    
    #[allow(dead_code)]
    pub fn error_msg(message: String) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        Ok(HttpResponse::Ok().json(ResponseResult::<usize> {
            success: true,
            code: 50000,
            message,
            data: None,
        }))
    }
}