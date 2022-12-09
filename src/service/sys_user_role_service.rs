use std::collections::BTreeMap;

use crate::domain::dto::user::UserRoleAddDTO;
use crate::domain::table::SysRole;
use crate::domain::vo::res::SysResVO;
use crate::domain::vo::role::SysRoleVO;
use crate::error::{Error, Result};

use crate::domain::table::tables::SysUserRole;

use crate::pool;
use crate::util::options::OptionStringRefUnwrapOrDefault;

use super::CONTEXT;
pub struct SysUserRoleService {}

impl SysUserRoleService {
    /// 添加角色
    /// 部分代码与原工程不一致
    pub async fn add(&self, arg: UserRoleAddDTO) -> Result<u64> {
        if arg.user_id.is_none() || arg.role_id.is_none() {
            return Err(Error::from("添加角色时用户和角色不能为空！"));
        }
        let user_id = arg.user_id.as_deref().unwrap().to_string();
        /// 形成角色
        let mut user_role = SysUserRole::from(arg);
        self.remove_by_user_id(user_id.as_str()).await?;
        Ok(SysUserRole::insert(pool!(), &user_role)
            .await?
            .rows_affected)
        // Err(Error::from("not completed."))
    }

    /// 删除角色（依据user_id）
    pub async fn remove_by_user_id(&self, user_id: &str) -> Result<u64> {
        Ok(
            SysUserRole::delete_by_column(pool!(), field_name!(SysUserRole.user_id), user_id)
                .await?
                .rows_affected,
        )
    }

    /// 删除角色（依据role_id）
    pub async fn remove_by_role_id(&self, role_id: &str) -> Result<u64> {
        Ok(
            SysUserRole::delete_by_column(pool!(), field_name!(SysUserRole.role_id), role_id)
                .await?
                .rows_affected,
        )
    }

    /// 查找指定的角色(依据 user_id，找出对应权限的 role )
    pub async fn find_user_role(
        &self,
        user_id: &str,
        all_res: &BTreeMap<String, SysResVO>,
    ) -> Result<Option<SysRoleVO>> {
        if user_id.is_empty() {
            return Ok(None);
        }
        let user_roles =
            SysUserRole::select_by_column(pool!(), field_name!(SysUserRole.user_id), user_id)
                .await?;
        // 下面的用法暂时还没弄清楚
        let role_ids = &rbatis::make_table_field_vec!(&user_roles, role_id);
        let roles = CONTEXT.sys_role_service.finds(role_ids).await?;
        let role_res_vec = CONTEXT
            .sys_role_service
            .find_role_res(&rbatis::make_table_field_vec!(&user_roles, role_id))
            .await?;
        let mut role_vo = vec![];
        for role in roles {
            let mut resources = vec![];
            for role_res in &role_res_vec {
                if role.id.is_some() && role.id.eq(&role_res.role_id) {
                    if let Some(res) = all_res.get(role_res.res_id.as_ref().unwrap_or_def()) {
                        resources.push(res.clone());
                    }
                }
            }
            let mut vo = SysRoleVO::from(role);
            vo.resource_ids = CONTEXT.sys_res_service.make_res_ids(&resources);
            vo.resources = resources;
            role_vo.push(vo);
        }
        if role_vo.is_empty() {
            return Ok(None);
        } else {
            return Ok(Some(role_vo[0].clone()));
        }
        // Err(Error::from("未完全实现"))
    }
}
