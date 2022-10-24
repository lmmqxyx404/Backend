use crate::config::config::ApplicationConfig;

/// service 层级
//pub mod sys_config_service;

/// 系统用户层
pub mod sys_user_service;

/// 校验层
pub mod sys_auth_service;

pub use sys_auth_service::*;
pub use sys_user_service::*;

// service context 必须为 pub,否则 无法给上下文使用
pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub sys_auth_service: SysAuthService,
    pub sys_user_service: SysUserService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        ServiceContext {
            config: config,
            sys_auth_service: SysAuthService {},
            sys_user_service: SysUserService {},
        }
    }
}

/// 提供一个上下文引用，给其余service 使用
/// 1. login
/// 2. cpatcha
lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}
