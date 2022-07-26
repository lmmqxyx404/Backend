#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
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

/// 服务模块
pub mod service;