use std::collections::{BTreeMap, HashMap};

use rbatis::{
    intercept::SqlIntercept,
    sql::{Page, PageRequest},
};

use crate::{
    domain::{
        dto::{res, role::RolePageDTO},
        table::{SysRole, SysRoleRes, SysUserRole},
        vo::{res::SysResVO, role::SysRoleVO},
    },
    error::{Error, Result},
};

use crate::pool;

use super::CONTEXT;

const RES_KEY: &'static str = "sys_role:all";

pub struct SysRoleService {}

impl SysRoleService {
    pub async fn page(&self, arg: &RolePageDTO) -> Result<Page<SysRoleVO>> {
        let data = SysRole::select_page_by_name(
            pool!(),
            &PageRequest::from(arg),
            arg.name.as_deref().unwrap_or_default(),
        )
        .await?;
        let all_roles = self.finds_all_map().await?;
        let mut page = Page::<SysRoleVO>::from(data);
        /* for mut vo in &mut page.records{
            self.loop
        } */
        Ok(page)
    }

    pub async fn finds_all_map(&self) -> Result<HashMap<String, SysRole>> {
        let all = self.finds_all().await?;
        let mut result = HashMap::with_capacity(all.capacity());
        for x in all {
            result.insert(x.id.as_deref().unwrap_or_default().to_string(), x);
        }
        Ok(result)
    }

    pub async fn finds_all(&self) -> Result<Vec<SysRole>> {
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysRole>>>(RES_KEY)
            .await;
        return Ok(js?.unwrap_or_default());
    }
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
