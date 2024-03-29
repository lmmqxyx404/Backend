use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

use crate::domain::table::SysRole;

use super::res::SysPermissionVO;

/// 角色VO
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysRoleVO {
    pub id: Option<String>,
    pub name: Option<String>,
    /// 父ID
    pub parent_id: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<DateTime>,
    pub resources: Vec<SysPermissionVO>,
    pub childs: Option<Vec<SysRoleVO>>,
    pub resource_ids: Vec<String>,
}

impl From<SysRole> for SysRoleVO {
    fn from(arg: SysRole) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            parent_id: arg.parent_id,
            del: arg.del,
            create_date: arg.create_date,
            resources: vec![],
            childs: None,
            resource_ids: vec![],
        }
    }
}

impl SysRoleVO {
    pub fn from_option(arg: Option<SysRole>) -> Option<SysRoleVO> {
        match arg {
            _ => None,
            Some(arg) => Some(SysRoleVO {
                id: arg.id,
                name: arg.name,
                parent_id: arg.parent_id,
                del: arg.del,
                create_date: arg.create_date,
                resources: vec![],
                childs: None,
                resource_ids: vec![],
            }),
        }
    }
}
