// use crate::domain::domain::SysUser;
use crate::domain::dto::sign_in::SignDTO;
use crate::domain::table::tables::SysUser;
use crate::domain::table::LoginCheck;
use crate::domain::vo::jwt::JWT_Token;
use crate::domain::vo::res::SysPermissionVO;
use crate::domain::vo::sign_in::SignVO;
use crate::domain::vo::user::SysUserVO;

use crate::domain::dto::IdDTO;
// 引入UserAddDTO
use crate::domain::dto::user::{UserAddDTO, UserEditDTO, UserPageDTO, UserRoleAddDTO};
/// use error module Result.
use crate::error::{Error, Result};

// 使用 rbatis pool macro,注意使用的路径
// 生成 pool 宏
use crate::pool;

// 引入CONTEXT
use crate::service::CONTEXT;
use crate::util::options::OptionStringRefUnwrapOrDefault;
use crate::util::password_encoder::PasswordEncoder;
use rbatis::rbdc::DateTime;
/// 引入 Page
use rbatis::{Page, PageRequest};
/// 引入BTREE
use std::collections::BTreeMap;
/// 绝大多数DTO映射成VO
pub struct SysUserService {}

impl SysUserService {
    /// 分页功能，只返回部分数据
    pub async fn page(&self, arg: &UserPageDTO) -> Result<Page<SysUserVO>> {
        let sys_user_page = SysUser::select_page(
            pool!(),
            &PageRequest::from(arg),
            arg.name.as_deref().unwrap_or_default(),
            arg.account.as_deref().unwrap_or_default(),
        )
        .await?;
        // 必须要实现相关 from trait
        let page = Page::<SysUserVO>::from(sys_user_page);
        Ok(page)
        // Err(Error::from("hello"))
    }

    /// 登录功能服务，被登录接口调用
    pub async fn sign_in(&self, arg: &SignDTO) -> Result<SignVO> {
        /// 防止爆破登录
        // println!("sign in {:?}", arg);
        let user = SysUser::select_by_column(pool!(), "account", &arg.account)
            .await?
            .into_iter()
            .next();
        // println!("{:?}", user);
        let user = user.ok_or_else(|| Error::from(format!("账号不存在: {}", arg.account)))?;
        if user.state.eq(&Some(0)) {
            return Err(Error::from("账户被封禁"));
        }
        let mut error = None;
        match user
            .login_check
            .as_ref()
            // 暂时也不知道 unwrap_or 有什么用处
            .unwrap_or(&LoginCheck::PasswordCheck)
        {
            LoginCheck::NoCheck => {}
            // 密码登录功能
            LoginCheck::PasswordCheck => {
                if !PasswordEncoder::verify(
                    user.password
                        .as_ref()
                        .ok_or_else(|| Error::from("错误的用户数据，密码为空"))?,
                    &arg.password,
                ) {
                    error = Some(Error::from("密码不正确"));
                }
            }
            LoginCheck::PasswordImgCodeCheck => {
                // 准备引入 cache_service
                let cache_code = CONTEXT
                    .cache_service
                    .get_string(&format!("cpatch:account_{}", &arg.account))
                    .await?;
                if cache_code.eq(&arg.vcode) {
                    error = Some(Error::from("验证码不正确！"))
                }
                if !PasswordEncoder::verify(
                    user.password
                        .as_ref()
                        .ok_or_else(|| Error::from("错误的用户数据，密码为空"))?,
                    &arg.password,
                ) {
                    error = Some(Error::from("密码不正确！"))
                }
            }
            LoginCheck::PhoneCodeCheck => {
                todo!()
            }
        }
        if error.is_some() {
            // todo增加重试次数
            return Err(error.unwrap());
        }
        let sign_in_vo = self.get_user_info(&user).await?;
        Ok(sign_in_vo)
        // Err(Error::from("build error"))
        /* let interimVO = SignVO {
            user: Some(SignDTO {

            }),
        };

        Ok(interimVO) */
    }

    ///  获得用户信息
    pub async fn get_user_info(&self, user: &SysUser) -> Result<SignVO> {
        /// 去除密码
        let mut user = user.clone();
        user.password = None;
        /// 拿到ID
        let user_id = user
            .id
            .clone()
            .ok_or_else(|| Error::from("用户数据错误，id为空"))?;
        // 上面是初步处理 SysUser 信息，与其余service进行隔离
        // 下面是返回 user 对应的有价值的权限信息，比如 role ,以及合成 token
        let mut sign_vo = SignVO::from(user);

        let jwt_token = JWT_Token {
            id: sign_vo.id.clone().unwrap_or_default(),
            account: sign_vo.account.clone().unwrap_or_default(),
            permissions: sign_vo.permissions.clone(),
            role_ids: vec![],
            exp: DateTime::now().unix_timestamp() as usize + CONTEXT.config.jwt_exp,
        };
        sign_vo.access_token = jwt_token.create_token(&CONTEXT.config.jwt_secret)?;
        // todo: 添加权限信息
        Ok(sign_vo)
        // let all_res = CONTEXT.sys_res_service.finds_all_map().await?;
        // interimVO.permissions = self.load_level_permission(&user_id, &all_res).await?;
    }

    /// 通过token登录
    pub async fn get_user_info_by_token(&self, token: &JWT_Token) -> Result<SignVO> {
        let user = SysUser::select_by_column(pool!(), field_name!(SysUser.id), &token.id)
            .await?
            .into_iter()
            .next();
        let user =
            user.ok_or_else(|| Error::from(format!("account {} is not exist", token.account)))?;
        return self.get_user_info(&user).await;
    }

    /// 是否需要等待，防止爆破。
    pub async fn need_wait_login(&self) -> Result<()> {
        Ok(())
    }

    /// 用户详情
    pub async fn detail(&self, arg: &IdDTO) -> Result<SysUserVO> {
        let user_id = arg.id.as_deref().unwrap_or_default();
        let user = self
            .find(&user_id)
            .await?
            .ok_or_else(|| Error::from(format!("用户{:?} 不存在", user_id)))?;
        let mut user_vo = SysUserVO::from(user);
        // 下面的finds_all_map
        let all_res = CONTEXT.sys_res_service.finds_all_map().await?;
        let role = CONTEXT
            .sys_user_role_service
            .find_user_role(&user_id, &all_res)
            .await?;
        user_vo.role = role;
        Ok(user_vo)
        // Err(Error::E("接口暂时没有实现".to_string()))
    }

    /// 根据用户id查找user
    pub async fn find(&self, user_id: &str) -> Result<Option<SysUser>> {
        Ok(
            SysUser::select_by_column(pool!(), field_name!(SysUser.id), user_id)
                .await?
                .into_iter()
                .next(),
        )
        // Ok(SysUser::selec)
        // Err(Error::E("接口暂时没有实现".to_string()))
    }

    /// 根据用户 account 查找 user
    pub async fn find_by_account(&self, account: &str) -> Result<Option<SysUser>> {
        Ok(
            SysUser::select_by_column(pool!(), field_name!(SysUser.account), account)
                .await?
                .into_iter()
                .next(),
        )
        // Ok(SysUser::selec)
        // Err(Error::E("接口暂时没有实现".to_string()))
    }

    /// 添加后台账号
    pub async fn add(&self, mut arg: UserAddDTO) -> Result<u64> {
        if arg.account.is_none()
            || arg.account.as_ref().is_none()
            || arg.name.is_none()
            || arg.name.as_ref().is_none()
        {
            return Err(Error::E("用户名和姓名不能为空".to_string()));
        }
        // unwrap_or_def 这个方法需要进一步了解
        let old_user = self
            .find_by_account(arg.account.as_deref().unwrap_or_default())
            .await?;
        if old_user.is_some() {
            return Err(Error::from(format!(
                "用户账户: {}已存在",
                arg.account.as_deref().unwrap_or_default()
            )));
        }
        let mut password = arg.password.as_deref().unwrap_or_default().to_string();
        if password.is_empty() {
            // 设置默认密码
            // password = "123456".to_string()
            return Err(Error::from(format!("账户密码不能为空")));
        }

        arg.password = Some(password);
        let role_id = arg.role_id.clone();
        let user = SysUser::from(arg);
        // 默认注册的是普通角色
        // 之后考虑手动操作数据库 改变 role_id 或者调用接口批量添加管理员等其他角色
        if role_id.is_some() {
            CONTEXT
                .sys_user_role_service
                .add(UserRoleAddDTO {
                    id: None,
                    user_id: user.id.clone(),
                    role_id: role_id,
                })
                .await?;
        }
        // 注意看 rows_affected 这个函数,弄清楚返回一个u64的用意
        Ok(SysUser::insert(pool!(), &user).await?.rows_affected)
    }

    /// 移除
    pub async fn remove(&self, id: &str) -> Result<u64> {
        if id.is_empty() {
            return Err(Error::E("id不能为空".to_string()));
        }
        let trash = SysUser::select_by_column(pool!(), field_name!(SysUser.id), id).await?;
        let r = SysUser::delete_by_column(pool!(), field_name!(SysUser.id), id).await?;
        /// 记录删除日志
        /// 删除对应用户的角色
        CONTEXT.sys_user_role_service.remove_by_user_id(id).await?;
        Ok(r.rows_affected)
    }

    /// 修改用户信息
    pub async fn edit_user_info(&self, arg: UserEditDTO) -> Result<u64> {
        let role_id = arg.role_id.clone();
        let mut user = SysUser::from(arg);
        user.account = None;
        let mut password = None;

        if user.password.is_some() {
            password = Some(PasswordEncoder::encode(user.password.as_ref().unwrap()));
        }
        user.password = password;
        if role_id.is_some() {
            CONTEXT
                .sys_user_role_service
                .add(UserRoleAddDTO {
                    id: None,
                    user_id: user.id.clone(),
                    role_id: role_id,
                })
                .await?;
        }
        Ok(
            SysUser::update_by_column(pool!(), &user, field_name!(SysUser.id))
                .await?
                .rows_affected,
        )
    }

    /// 删除用户信息
    pub async fn remove_user_info(&self) -> Result<()> {
        Ok(())
    }

    /// 查找用户-权限
    pub async fn load_level_permission(
        &self,
        user_id: &str,
        all_res: &BTreeMap<String, SysPermissionVO>,
    ) -> Result<Vec<String>> {
        CONTEXT
            .sys_role_service
            .find_user_permission(user_id, all_res)
            .await
    }
}
