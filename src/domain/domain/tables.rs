/// 后台用户表
#[crud_table]
#[derive(Debug, Clone)]
pub struct SysUser {
    pub id: Option<String>,
    /// 邮箱登录|手机号登录
    pub account: Option<String>,
    pub password: Option<String>,
    /// 用户名登录
    pub name: Option<String>,
}
