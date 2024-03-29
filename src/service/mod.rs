/// 配置来源
pub use crate::config::config::ApplicationConfig;
/// 具体使用的依赖
/// 1. 静态化一个全局变量方便全局共享
use once_cell::sync::Lazy;
/// 2. 辅助生成 rbatis 实例,实现 orm 线程池
use rbatis::rbatis::RBatis;
/// service 层级

/// 系统用户层
mod sys_user_service;

/// 校验层
mod sys_auth_service;

/// 系统用户角色
/// 有普通用户，管理员，超级管理员，员工
mod sys_user_role_service;

/// 缓存服务
mod cache_service;
mod mem_service;
mod redis_service;

/// 角色服务
mod sys_role_service;

/// 系统角色-权限资源服务
mod sys_role_res_service;

/// 手机信息验证码服务
mod sys_sms_service;

/// 系统执行的一些资源服务
mod sys_res_service;

/// 垃圾桶服务
mod sys_trash_service;

/// 系统字典服务
mod sys_dict_service;

pub use mem_service::*;
pub use redis_service::*;
// pub mod sys_config_service;

pub use sys_auth_service::*;
pub use sys_dict_service::*;
pub use sys_user_role_service::*;
pub use sys_user_service::*;

/// 使用缓存服务
use self::{
    cache_service::CacheService, sys_res_service::SysResService,
    sys_role_res_service::SysRoleResService, sys_role_service::SysRoleService,
    sys_sms_service::SysSmsService, sys_trash_service::SysTrashService,
};
// service context 必须为 pub,否则 无法给上下文使用
pub struct ServiceContext {
    // 2022年10月26日00点15分 添加 rbatis
    pub rbatis: RBatis,
    pub config: ApplicationConfig,
    pub sys_auth_service: SysAuthService,
    pub sys_user_service: SysUserService,
    pub cache_service: CacheService,
    pub sys_role_service: SysRoleService,
    pub sys_sms_service: SysSmsService,
    pub sys_res_service: SysResService,
    pub sys_user_role_service: SysUserRoleService,
    pub sys_role_res_service: SysRoleResService,
    pub sys_dict_service: SysDictService,
    pub sys_trash_service: SysTrashService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        // 调用以下方法
        // 1. 生成配置结构体
        let config = ApplicationConfig::default();
        // 2. 生成 rbatis 实例
        // let rbatis_instanece = crate::domain::init_rbatis(&config);
        ServiceContext {
            cache_service: CacheService::new(&config).unwrap(),
            rbatis: {
                let rb = RBatis::new();
                if rb.is_debug_mode() == false && config.debug.eq(&true) {
                    panic!(r#"please edit application.json5   “debug: false” "#);
                }
                rb
            },
            sys_auth_service: SysAuthService {},
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_sms_service: SysSmsService {},
            sys_res_service: SysResService {},
            sys_user_role_service: SysUserRoleService {},
            // 2023年1月18日23点49分
            sys_role_res_service: SysRoleResService {},
            sys_dict_service: SysDictService {},
            sys_trash_service: SysTrashService {},
            // must set at the last so that can transfer the ownership correctly.
            config: config,
        }
    }
}

// use rbdc_mysql::driver::MysqlDriver;

impl ServiceContext {
    pub async fn init_pool(&self) {
        // 连接数据库
        // println!("hello world");
        // println!("[backend] rbatis pool init ({})", &self.config.database_url);
        log::info!(
            "[abs_admin] rbatis pool init ({})...",
            self.config.database_url
        );
        // 这是异步方法
        self.rbatis
            .link(
                include!("../../target/driver.rs"),
                &self.config.database_url,
            )
            .await
            .expect("[abs_admin] rbatis pool init fail!");
        // let res = self.rbatis.init(MysqlDriver {}, &self.config.database_url);
        log::info!(
            "[abs_admin] rbatis pool init success! pool state = {}",
            self.rbatis
                .get_pool()
                .expect("pool not init!")
                .state()
                .await
        );
        log::info!(
            " - Local:   http://{}",
            self.config.server_url.replace("0.0.0.0", "127.0.0.1")
        );

        //.expect("[backend] rbatis failed");
        // 输出日志
    }
}
/// 提供一个上下文引用，给其余service 使用
/// CONTEXT  is all of the service.
/// 2022年10月26日00点00分 修改了默认实现
/// 1. login
/// 2. cpatcha
/*
lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}
*/

pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

/// 生成 rbatis 连接宏
/// 在具体的 service 中使用
/// 1. sys_user_service::find
#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::service::CONTEXT.rbatis.clone()
    };
}

mod test {
    use super::*;

    #[test]
    fn test_link_database() {
        let aaa = ServiceContext::default();
        println!("prepare to init_pool");
        // 异步测试需要单独做
        // let res = aaa.rbatis.link(MysqlDriver {}, &aaa.config.database_url).await;
        println!("{}", aaa.config.database_url);
        // assert_eq!(res.is_ok(), true);
    }
}
