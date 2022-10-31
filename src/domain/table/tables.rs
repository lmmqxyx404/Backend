use rbatis::object_id::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{domain::dto::user::UserAddDTO, util::password_encoder::PasswordEncoder};

/// 后台用户表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUser {
    pub id: Option<String>,
    /// 邮箱登录|手机号登录
    pub account: Option<String>,
    pub password: Option<String>,
    /// 用户名登录
    pub name: Option<String>,
}

/// 转化
impl From<UserAddDTO> for SysUser {
    fn from(arg: UserAddDTO) -> Self {
        SysUser {
            id: ObjectId::new().to_string().into(),
            account: arg.account.clone(),
            password: PasswordEncoder::encode(&arg.password.unwrap_or_default()).into(),
            name: arg.name.clone(),
        }
    }
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
