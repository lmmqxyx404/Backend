use std::collections::BTreeMap;

use crate::{
    domain::{
        table::{SysRole, SysRoleRes, SysUserRole},
        vo::res::SysResVO,
    },
    error::{Error, Result},
};

use crate::pool;
pub struct SysRoleService {}

impl SysRoleService {
    pub async fn find_role_res(&self, ids: &Vec<String>) -> Result<Vec<SysRoleRes>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(SysRoleRes::select_by_role_id(pool!(), ids).await?)
        // Err(Error::from("hello"))
    }

    pub async fn find_user_permission(
        &self,
        user_id: &str,
        all_res: &BTreeMap<String, SysResVO>,
    ) -> Result<Vec<String>> {
        let user_roles =
            SysUserRole::select_by_column(pool!(), field_name!(SysUserRole.user_id), user_id)
                .await?;
        // let role_res=self.find_role_res();
        Err(Error::from("hello"))
    }

    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysRole>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(SysRole::select_list_by_ids(pool!(), ids).await?)
        // Err(Error::from("hello"))
    }
}
