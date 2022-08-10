use crate::domain::domain::SysUser;
use crate::domain::dto::sign_in::SignDTO;
use crate::domain::vo::sign_in::SignVO;

/// use error module Result.
use crate::error::Result;
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

    pub async fn get_user_info(&self, user: &SysUser) -> Result<SignVO> {
        let interimVO = SignVO {
            user: Some(SignDTO {}),
        };
        Ok(interimVO)
    }
}
