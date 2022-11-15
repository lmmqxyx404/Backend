use std::collections::BTreeMap;

use crate::{
    domain::vo::res::SysResVO,
    error::{Error, Result},
};

pub struct SysRoleService {}

impl SysRoleService {
    pub async fn find_user_permission(
        &self,
        user_id: &str,
        all_res: &BTreeMap<String, SysResVO>,
    ) -> Result<Vec<String>> {
        Err(Error::from("hello"))
    }
}
