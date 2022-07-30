#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)]
//允许未使用的代码
// This is a diagnostic mcro
// The following attributes are used for controlling or generating diagnostic messages during compilation.
#![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

///工具类
#[macro_use]
pub mod util;
/// 配置模块
pub mod config;

/// 接口模块
pub mod controller;
/// 服务模块
pub mod service;

/// 错误模块
/// 专门处理各种错误下信息，主动报错，或者被动报错
pub mod error;

/// 领域模型
pub mod domain;