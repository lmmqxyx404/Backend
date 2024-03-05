use rbatis::object_id::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{domain::dto::user::UserAddDTO, util::password_encoder::PasswordEncoder};

/// 创建角色时用的上
use rbatis::rbdc::datetime::DateTime;

use crate::domain::table::LoginCheck;

/// 后台用户表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUser {
    pub id: Option<String>,
    /// 邮箱登录|手机号登录
    pub account: Option<String>,
    pub password: Option<String>,
    pub login_check: Option<LoginCheck>,
    /// 用户名登录
    pub name: Option<String>,
    pub state: Option<i32>,
    pub del: Option<i32>,
    pub create_date: Option<DateTime>,
}

/// 转化
impl From<UserAddDTO> for SysUser {
    fn from(arg: UserAddDTO) -> Self {
        SysUser {
            id: ObjectId::new().to_string().into(),
            account: arg.account.clone(),
            password: PasswordEncoder::encode(&arg.password.unwrap_or_default()).into(),
            name: arg.name.clone(),
            login_check: arg.login_check,
            state: 0.into(),
            del: 0.into(),
            create_date: DateTime::now().into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUserRole {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
    pub create_date: Option<DateTime>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SysRoleRes {
    pub id: Option<String>,
    /// 角色ID
    pub role_id: Option<String>,
    /// 资源ID
    pub res_id: Option<String>,
    pub create_date: Option<DateTime>,
}

/// 权限资源表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysPermission {
    pub id: Option<String>,
    //father id(can empty)
    pub parent_id: Option<String>,
    pub name: Option<String>,
    //permission
    pub permission: Option<String>,
    //menu path
    pub path: Option<String>,
    pub create_date: Option<DateTime>,
}

/// 角色表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysRole {
    pub id: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysTrash {
    pub id: Option<String>,
    pub table_name: Option<String>,
    pub date: Option<String>,
    pub create_date: Option<DateTime>,
}

/// 字典表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysDict {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<DateTime>,
}

///Role Permission relational tables (relational tables do not use logical deletion)
#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SysRolePermission {
    pub id: Option<String>,
    pub role_id: Option<String>,
    pub permission_id: Option<String>,
    pub create_date: Option<DateTime>,
}

mod test {
    use crate::domain::table::enums::LoginCheck;

    use super::*;

    #[test]
    fn convert() {
        let user = UserAddDTO {
            account: Some("hello".to_string()),
            password: Some("String".to_string()),
            name: Some("String".to_string()),
            role_id: Some("String".to_string()),
            login_check: Some(LoginCheck::NoCheck),
        };
        let b = SysUser::from(user);
        println!("{:?}", b);
    }
}
