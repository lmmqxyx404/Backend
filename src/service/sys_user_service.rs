use crate::domain::domain::SysUser;
use crate::domain::dto::sign_in::SignDTO;
use crate::domain::vo::sign_in::SignVO;

/// use error module Result.
use crate::error::{Error, Result};
pub struct SysUserService {}

impl SysUserService {
    pub async fn sign_in(&self, arg: &SignDTO) -> Result<SignVO> {
        /// 防止爆破登录
        let mut error = None;
        if error.is_some() {
            /// 增加重试次数
            return Err(error.unwrap());
        }
        let interimVO = SignVO {
            user: Some(SignDTO {}),
        };
        Ok(interimVO)
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
        let interimVO = SignVO {
            user: Some(SignDTO {}),
        };
        Ok(interimVO)
    }

    /// 通过token登录
    pub async fn get_user_info_by_token(&self, user: &SysUser) -> Result<SignVO> {
        return self.get_user_info(&user).await;
    }

    /// 是否需要等待，防止爆破。
    pub async fn need_wait_login(&self) -> Result<()> {
        Ok(())
    }

    /// 修改用户信息
    pub async fn edit_user_info(&self) -> Result<()> {
        Ok(())
    }

    /// 删除用户信息
    pub async fn remove_user_info(&self) -> Result<()> {
        Ok(())
    }
}
