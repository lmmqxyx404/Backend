use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::PageRequest;
use serde::{Deserialize, Serialize};

use crate::domain::table::enums::LoginCheck;

use crate::domain::table::{SysUser, SysUserRole};
use crate::util::password_encoder::PasswordEncoder;
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

pub struct UserPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub account: Option<String>,
    pub name: Option<String>,
}

impl From<&UserPageDTO> for PageRequest {
    fn from(arg: &UserPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

/// 编辑用户信息
pub struct UserEditDTO {
    pub id: Option<String>,
    /// 邮箱登录|手机号登录
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub state: Option<i32>,
    pub login_check: Option<LoginCheck>,
    pub role_id: Option<String>,
}

impl From<UserEditDTO> for SysUser {
    fn from(arg: UserEditDTO) -> Self {
        SysUser {
            id: arg.id,
            account: arg.account,
            password: arg.password,
            login_check: arg.login_check,
            name: arg.name,
            state: arg.state,
            del: None,
            create_date: None,
        }
    }
}
