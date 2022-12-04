#[allow(dead_code)]
pub struct ResponseResult<T> {
    pub success: bool,
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ResponseResult<T> {
    #[allow(dead_code)]
    pub fn ok() -> ResponseResult<T> {
        ResponseResult {
            success: true,
            code: 20000,
            message: "success".to_string(),
            data: None,
        }
    }
    
    #[allow(dead_code)]
    pub fn ok_data(t: T) -> ResponseResult<T> {
        ResponseResult {
            success: true,
            code: 20000,
            message: "success".to_string(),
            data: Some(t),
        }
    }
    
    #[allow(dead_code)]
    pub fn error_msg(message: String) -> ResponseResult<T> {
        ResponseResult {
            success: true,
            code: 50000,
            message,
            data: None,
        }
    }
}