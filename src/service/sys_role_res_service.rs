use std::collections::{BTreeMap, HashMap, HashSet};

use rbatis::sql::Page;

use crate::{
    domain::{
        dto::role::{RolePageDTO, SysRoleResPageDTO},
        table::SysRoleRes,
        vo::{res::SysResVO, role::SysRoleVO},
    },
    error::{Error, Result},
    util::options::OptionStringRefUnwrapOrDefault,
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
        let role_res_map = self.find_role_res_map(&role_page.records).await?;
        // role_page.records=self.l
        Err(Error::from("zan wei wancheng"))
    }

    /// 添加角色资源
    //pub async fn add(&self,arg: &)

    /// 添加资源
    fn loop_set_res_vec(
        &self,
        arg: Vec<SysRoleVO>,
        role_res_map: &HashMap<String, HashSet<SysRoleRes>>,
        all: &BTreeMap<String, SysResVO>,
    ) -> Result<Vec<SysRoleVO>> {
        let mut data = vec![];
        for mut role in arg {
            let res_ids = role_res_map.get(role.id.as_ref().unwrap_or_def());
            let mut res_vos = vec![];
            if let Some(res_ids) = res_ids {
                for x in res_ids {
                    match all.get(x.res_id.as_ref().unwrap_or_def()) {
                        Some(res) => {
                            res_vos.push(res.clone());
                        }
                        _ => {}
                    }
                }
            }

            role.resources = res_vos;
            if role.childs.is_some() {
                role.childs = Some(self.loop_set_res_vec(
                    role.childs.unwrap_or(vec![]),
                    role_res_map,
                    all,
                )?);
            }
            role.resource_ids = rbatis::make_table_field_vec!(&role.resources, id);
            data.push(role);
        }
        Ok(data)
    }

    async fn find_role_res_map(
        &self,
        arg: &Vec<SysRoleVO>,
    ) -> Result<HashMap<String, HashSet<SysRoleRes>>> {
        Err(Error::from("zan wei wancheng"))
    }
}
