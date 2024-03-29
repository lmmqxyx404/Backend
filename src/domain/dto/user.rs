use rbatis::object_id::ObjectId;
use rbatis::rbdc::DateTime;
use rbatis::PageRequest;
use serde::{Deserialize, Serialize};

use crate::domain::table::enums::LoginCheck;

use crate::domain::table::{SysUser, SysUserRole};
use crate::util::password_encoder::PasswordEncoder;

/// UserAddDTO
/// 用户添加DTO,侧重于用户
/// todo: 之后要做各种长度校验
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserAddDTO {
    /// 一般认为是 email account
    /// 之后要做限制 校验之类的
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub role_id: Option<String>,
    pub state: Option<i32>,
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
            create_date: DateTime::now().into(),
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

/// 实现 UserRolePageDTO --> UserPageDTO
impl From<&UserRolePageDTO> for UserPageDTO {
    fn from(arg: &UserRolePageDTO) -> Self {
        Self {
            page_no: arg.page_no.clone(),
            page_size: arg.page_size.clone(),
            account: arg.account.clone(),
            name: arg.name.clone(),
        }
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
            // del: None,
            create_date: None,
        }
    }
}

/// 用户角色分页
pub struct UserRolePageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub account: Option<String>,
    pub name: Option<String>,

    // 默认添加 role
    pub resp_set_role: Option<bool>,
}

impl From<UserRolePageDTO> for PageRequest {
    fn from(value: UserRolePageDTO) -> Self {
        PageRequest::new(value.page_no.unwrap_or(1), value.page_size.unwrap_or(10))
    }
}
