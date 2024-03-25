///  切记 任何一个暴露出去的接口都应该打印log日志

/// 图片验证码接口(get)
pub mod img_verify_controller;
/// 登录接口(post)
pub mod sys_user_controller;

/// 校验 check 接口
pub mod sys_auth_controller;

/// 角色初始化接口
pub mod sys_role_controller;

/// 系统字典接口
pub mod sys_dict_controller;

///系统权限资源接口
pub mod sys_res_controller;

/// 获取订阅地址接口
pub mod shop_proxy_controller;
