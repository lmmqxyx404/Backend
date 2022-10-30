use serde::{Deserialize, Serialize};

use crate::domain::table::enums::LoginCheck;

/// UserAddDTO
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserAddDTO {
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub role_id: Option<String>,
    pub login_check: Option<LoginCheck>,
}
