use std::collections::BTreeMap;

use rbatis::object_id::ObjectId;
use rbatis::Page;

use crate::domain::dto::user::{UserPageDTO, UserRoleAddDTO, UserRolePageDTO};
use crate::domain::table::SysRole;
use crate::domain::vo::res::SysPermissionVO;
use crate::domain::vo::role::SysRoleVO;
use crate::domain::vo::user::SysUserVO;
use crate::error::{Error, Result};

use crate::domain::table::tables::SysUserRole;

use crate::pool;
use crate::util::options::OptionStringRefUnwrapOrDefault;

use super::CONTEXT;
pub struct SysUserRoleService {}

impl SysUserRoleService {
    /// 角色分页
    pub async fn page(&self, arg: &UserRolePageDTO) -> Result<Page<SysUserVO>> {
        let mut vo = CONTEXT
            .sys_user_service
            .page(&UserPageDTO::from(arg))
            .await?;
        // 暂时还不知道下面这一部分代码的含义
        /* if arg.resp_set_role.unwrap_or(true) {
            let all_role = CONTEXT.sys_role_service.finds_all_map().await?;
            // let user_ids
        } */
        Ok(vo)
        // Err(Error::from("未完全实现"))
    }

    /// 添加角色
    /// 部分 变量名 与原工程不一致
    pub async fn add(&self, arg: UserRoleAddDTO) -> Result<u64> {
        if arg.user_id.is_none() || arg.role_id.is_none() {
            return Err(Error::from("添加角色时用户和角色不能为空！"));
        }
        let user_id = arg.user_id.as_deref().unwrap().to_string();
        /// 形成角色
        let mut user_role = SysUserRole::from(arg);
        if user_role.id.is_none() {
            user_role.id = Some(ObjectId::new().to_string());
        }
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
        all_res: &BTreeMap<String, SysPermissionVO>,
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
                    if let Some(res) = all_res.get(role_res.permission_id.as_ref().unwrap_or_def())
                    {
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
