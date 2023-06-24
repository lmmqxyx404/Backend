use crate::domain::dto::auth::SysAuthDTO;
use crate::domain::vo::jwt::JWT_Token;
use crate::error::Result;
pub struct SysAuthService {}

impl SysAuthService {
    pub async fn check_auth(&self, arg: SysAuthDTO) -> Result<JWT_Token> {
        todo!()
    }
}
