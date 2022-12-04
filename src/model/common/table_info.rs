#[allow(dead_code)]
pub struct TableInfo<T> {
    pub count: u32,
    pub list: Vec<T>,
}

impl<T> TableInfo<T> {
    #[allow(dead_code)]
    pub fn new(count: u32, list: Vec<T>) -> TableInfo<T> {
        TableInfo {
            count,
            list,
        }
    }
}