use rbatis::object_id::ObjectId;
use rbatis::rbdc::DateTime;
use rbatis::PageRequest;
use serde::{Deserialize, Serialize};

use crate::domain::table::SysPermission;

/// 资源分页
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub string: Option<String>,
}

impl From<&ResPageDTO> for PageRequest {
    fn from(arg: &ResPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResAddDTO {
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
}

impl From<&ResAddDTO> for SysPermission {
    fn from(arg: &ResAddDTO) -> Self {
        SysPermission {
            id: ObjectId::new().to_string().into(),
            parent_id: arg.parent_id.clone(),
            name: arg.name.clone(),
            permission: arg.permission.clone(),
            path: arg.path.clone(),
            // del: 0.into(),
            create_date: DateTime::now().into(),
        }
    }
}

/// 资源修改
#[derive(Debug, Clone)]

pub struct ResEditDTO {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
}

impl From<&ResEditDTO> for SysPermission {
    fn from(arg: &ResEditDTO) -> Self {
        SysPermission {
            id: arg.id.clone(),
            parent_id: arg.parent_id.clone(),
            name: arg.name.clone(),
            permission: arg.permission.clone(),
            path: arg.path.clone(),
            // del: None,
            create_date: None,
        }
    }
}
