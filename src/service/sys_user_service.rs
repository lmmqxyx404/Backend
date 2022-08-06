use crate::domain::dto::sign_in::SignDTO;
use crate::domain::vo::sign_in::SignVO;

pub struct SysUserService {}

impl SysUserService {
    pub async fn sign_in(&self, arg: &SignDTO) -> Result<(), E> {
        /// 防止爆破登录
        let mut error = None;
        if error.is_some() {
            /// 增加重试次数
            return Err(error.unwrap());
        }
        return Ok(());
    }
}
