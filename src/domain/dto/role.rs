use rbatis::{object_id::ObjectId, rbdc::datetime::FastDateTime, sql::PageRequest};
use serde::{Deserialize, Serialize};

use crate::domain::table::SysRole;

/// 角色分页
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolePageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}

impl From<&RolePageDTO> for PageRequest {
    fn from(arg: &RolePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}
/// 角色资源分页
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysRoleResPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}

impl From<&SysRoleResPageDTO> for PageRequest {
    fn from(arg: &SysRoleResPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleAddDTO {
    pub name: Option<String>,
    pub parent_id: Option<String>,
}

impl From<RoleAddDTO> for SysRole {
    fn from(value: RoleAddDTO) -> Self {
        SysRole {
            id: ObjectId::new().to_string().into(),
            name: value.name,
            parent_id: value.parent_id,
            del: 0.into(),
            create_date: FastDateTime::now().set_micro(0).into(),
        }
    }
}

/// 角色资源添加
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysRoleResAddDTO {
    pub name: Option<String>,
    pub parent_id: Option<String>,
    // 资源 ID 集合
    pub resource_ids: Vec<String>,
}

impl From<SysRoleResAddDTO> for RoleAddDTO {
    fn from(value: SysRoleResAddDTO) -> Self {
        Self {
            name: value.name,
            parent_id: value.parent_id,
        }
    }
}
