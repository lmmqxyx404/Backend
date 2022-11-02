use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::table::enums::LoginCheck;

use crate::domain::table::tables::SysUserRole;
/// UserAddDTO
/// 用户添加DTO,侧重于用户
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserAddDTO {
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub role_id: Option<String>,
    pub login_check: Option<LoginCheck>,
}

/// 用户角色添加，侧重于角色
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserRoleAddDTO {
    pub id: Option<String>,
    pub role_id: Option<String>,
    pub user_id: Option<String>,
}

/// 实现不同类型的转化
impl From<UserRoleAddDTO> for SysUserRole {
    fn from(arg: UserRoleAddDTO) -> Self {
        SysUserRole {
            id: arg.id.clone(),
            user_id: arg.user_id.clone(),
            role_id: arg.role_id.clone(),
            create_date: FastDateTime::now().set_micro(0).into(),
        }
    }
}
