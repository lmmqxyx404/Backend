use std::collections::{HashMap, HashSet};

use rbatis::sql::Page;

use crate::{
    domain::{
        dto::role::{RolePageDTO, SysRoleResPageDTO},
        table::SysRoleRes,
        vo::role::SysRoleVO,
    },
    error::{Error, Result},
};

use super::CONTEXT;

pub struct SysRoleResService {}

impl SysRoleResService {
    pub async fn page(&self, arg: &SysRoleResPageDTO) -> Result<Page<SysRoleVO>> {
        let mut role_page = CONTEXT
            .sys_role_service
            .page(&RolePageDTO {
                page_no: arg.page_no.clone(),
                page_size: arg.page_size.clone(),
                name: arg.name.clone(),
            })
            .await?;
        let all = CONTEXT.sys_res_service.finds_all_map().await?;
        // let role_res_map=
        Err(Error::from("zan wei wancheng"))
    }

    async fn find_role_res_map(
        &self,
        arg: &Vec<SysRoleVO>,
    ) -> Result<HashMap<String, HashSet<SysRoleRes>>> {
        
        Err(Error::from("zan wei wancheng"))
    }
}
