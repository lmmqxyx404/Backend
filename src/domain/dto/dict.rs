use rbatis::{rbdc::datetime::FastDateTime, sql::PageRequest};
use serde::{Deserialize, Serialize};

use crate::domain::table::SysDict;

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

/// 字典修改 （date: 2023年3月23日00:35）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictEditDTO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
}

impl From<&DictEditDTO> for SysDict {
    fn from(value: &DictEditDTO) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            code: value.code.clone(),
            state: value.state.clone(),
            create_date: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictAddDTO {
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
}

impl From<DictAddDTO> for SysDict {
    fn from(value: DictAddDTO) -> Self {
        Self {
            id: value.name.clone(),
            name: value.name.clone(),
            code: value.code.clone(),
            state: value.state.clone(),
            create_date: FastDateTime::now().set_micro(0).into(),
        }
    }
}
