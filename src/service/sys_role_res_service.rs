use std::collections::{BTreeMap, HashMap, HashSet};

use rbatis::sql::Page;

use crate::{
    domain::{
        dto::role::{
            RoleAddDTO, RoleEditDTO, RolePageDTO, SysRoleResAddDTO, SysRoleResPageDTO,
            SysRoleResUpdateDTO,
        },
        table::SysRoleRes,
        vo::{res::SysResVO, role::SysRoleVO},
    },
    error::{Error, Result},
    util::options::OptionStringRefUnwrapOrDefault,
};

use super::CONTEXT;

pub struct SysRoleResService {}

impl SysRoleResService {
    /// 角色-资源 分页
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
        role_page.records = self.loop_set_res_vec(role_page.records, &role_res_map, &all)?;
        Ok(role_page)
        // Err(Error::from("zan wei wancheng"))
    }

    /// 添加角色资源
    pub async fn add(&self, arg: &SysRoleResAddDTO) -> Result<u64> {
        let (_, role_id) = CONTEXT
            .sys_role_service
            .add_role(RoleAddDTO::from(arg.clone()))
            .await?;
        Err(Error::from("zan wei wancheng"))
    }

    /// 保存所有资源
    async fn save_role_res(&self, role_id: &str, resource_ids: Vec<String>) -> Result<u64> {
        Err(Error::from("zan wei wancheng"))
    }

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
        // todo:
        let role_ids=self.loop_find_role_ids(arg);
        
        todo!()
        // Err(Error::from("zan wei wancheng"))
    }

    /// 删除角色资源（依据role_id）
    pub async fn remove_by_role_id(&self, role_id: &str) -> Result<u64> {
        Err(Error::from("zan wei wancheng"))
        // Ok(SysRoleRes)
    }

    /// 角色删除，同时删除用户关系，权限关系
    pub async fn remove_role(&self, role_id: &str) -> Result<u64> {
        let remove_roles = CONTEXT.sys_role_service.remove(role_id).await?;
        let remove_user_roles = CONTEXT
            .sys_user_role_service
            .remove_by_role_id(role_id)
            .await?;
        // todo:
        Err(Error::from("zan wei wancheng"))
    }

    pub async fn edit(&self, arg: &SysRoleResUpdateDTO) -> Result<u64> {
        let role_id = arg
            .id
            .as_ref()
            .ok_or_else(|| Error::from("role id cn not be empty"))?;
        CONTEXT
            .sys_role_service
            .edit(RoleEditDTO::from(arg.clone()))
            .await?;
        todo!()
    }

    fn loop_find_role_ids(&self, arg: &Vec<SysRoleVO>) -> Vec<String> {
        // todo
        todo!()
    }
}
