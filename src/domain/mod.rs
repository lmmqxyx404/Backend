/// * Data Transfer Object dto 层级，存放接口传输的结构体，一般是 http request 参数
pub mod dto;

/// * View Object 展示层，存放展示的结构体。也就是http response
pub mod vo;

/// * Domain 领域层，存放数据库结构体模型
/// 2022年10月25日22点51分，domain 已经被拆分为 table 和 mapper 两部分
// pub mod domain;


/// * 实现具体的数据库查询功能
pub mod mapper;

/// * DDD 分层架构，数据库结构，存放数据库结构体模型
pub mod table;

use crate::rbatis::Rbatis;
// 注意一下是否有区别（22点38分2022年10月25日）
use crate::service::ApplicationConfig;
// use crate::config::config::ApplicationConfig;

/// 实例化 rbatis orm 连接池，主要检测运行模式与配置选项是否匹配
pub fn init_rbatis(config: &ApplicationConfig) -> Rbatis {
    let rbatis_instance = Rbatis::new();
    if rbatis_instance.is_debug_mode() == false && config.debug.eq(&true) {
        panic!(
            r#" 已经使用了 release 模式运行，但是配置选项是config ，请去修改 application.yml 中debug配置项"#
        );
    }
    rbatis_instance
}
