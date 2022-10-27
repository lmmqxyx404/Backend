<!--
 * @Author: Lmmqxyx
 * @Date: 2022-03-07 17:48:39
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-03-07 17:55:36
 * @FilePath: \backend\README_ZH.md
 * @Description: 
-->
# 项目简介
这是一个传统的restful风格后端工程项目，
我的目标是让这个项目能够对有志于使用rust开发web后端的新手提供有效的指引。
当然也可以理解为我个人的开发笔记

如果只是想在目前基础上快速开发，建议看一下项目功能历史以及项目依赖即可。

## 指导开发思想
ddd (领域驱动设计)

# 存在问题（大概是BUG）
## 数据库连接时，暂时无法处理连接失败的情况
已经解决，```init()```方法，不会建立真实连接，只是读取配置（先检测连接配置是否正常）。
可以使用```rb.try_acquire().await?;```去检测连接的是否成功。

# 项目功能历史
## 0.1 验证码接口功能
完整实现了图形验证码接口功能

## 0.2 登录接口
### 登录方式暂时只考虑用户名密码登录
1. 密码后端拿到的是明文
2. 存储在数据库中的是密文
3. 采用对称加密
4. 后期会改进安全性
 
### 后期考虑添加支持手机验证码登录

## 0.3 退出登录接口


# 项目依赖
## 1. tokio
提供异步运行时

## 2. rbatis
提供 orm

## 3. jsonwebtoken
引入 json_web_token 校验

## 4. md5
用来加密密码（之后会考虑更严谨的加密）


# rust 代码开发技巧
## 使用空的结构体实现一系列方法集合
```
struct service {};
impl service {
    fn new(){};
    fn default(){};
    fn detail(){};
    ...
}
```
进而在一系列 services 之间共享。
