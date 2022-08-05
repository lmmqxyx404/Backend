use crate::config::config::ApplicationConfig;

/// service 层级
pub mod sys_config_service;

// service context 必须为 pub,否则 无法给上下文使用
pub struct ServiceContext {
    pub config: ApplicationConfig,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        ServiceContext { config }
    }
}

/// 提供一个上下文引用，给其余service 使用
/// 1. login
/// 2. cpatcha
lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}
