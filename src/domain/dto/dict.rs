use rbatis::sql::PageRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
}

impl From<DictPageDTO> for PageRequest {
    fn from(value: DictPageDTO) -> Self {
        PageRequest::new(value.page_no.unwrap_or(1), value.page_size.unwrap_or(10))
    }
}

impl From<&DictPageDTO> for PageRequest {
    fn from(value: &DictPageDTO) -> Self {
        PageRequest::new(value.page_no.unwrap_or(1), value.page_size.unwrap_or(10))
    }
}
