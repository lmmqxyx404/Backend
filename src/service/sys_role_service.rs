use std::collections::{BTreeMap, HashMap};

use rbatis::{Page, PageRequest};

use crate::{
    domain::{
        dto::{
            permission,
            role::{RoleAddDTO, RoleEditDTO, RolePageDTO},
        },
        table::{SysRole, SysRolePermission, SysUserRole},
        vo::{res::SysPermissionVO, role::SysRoleVO},
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
        for mut vo in &mut page.records {
            self.loop_find_childs(&mut vo, &all_roles);
        }
        Ok(page)
    }

    /// sys_role_controller 会调用
    pub async fn finds_layer(&self) -> Result<Vec<SysRoleVO>> {
        let all = self.finds_all_map().await?;
        let mut data = vec![];
        for (k, v) in &all {
            if v.parent_id.is_none() {
                let mut top = SysRoleVO::from(v.clone());
                self.loop_find_childs(&mut top, &all);
                data.push(top);
            }
        }
        Ok(data)
        // Err(Error::from("hello"))
    }

    /// 查找 role 数组
    pub async fn finds_all(&self) -> Result<Vec<SysRole>> {
        // 查找的数据缓存于 Redis, 同时 remove, edit方法调用时刷新redis缓存
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysRole>>>(RES_KEY)
            .await;
        return Ok(js?.unwrap_or_default());
    }

    /// 所有用户Id-用户Map数据
    pub async fn finds_all_map(&self) -> Result<HashMap<String, SysRole>> {
        let all = self.finds_all().await?;
        let mut result = HashMap::with_capacity(all.capacity());
        for x in all {
            result.insert(x.id.as_deref().unwrap_or_default().to_string(), x);
        }
        Ok(result)
    }

    /// 更新所有
    pub async fn update_cache(&self) -> Result<Vec<SysRole>> {
        let all = SysRole::select_all(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_KEY, &all).await?;
        Ok(all)
    }

    pub async fn find_role_res(&self, role_ids: &Vec<String>) -> Result<Vec<SysRolePermission>> {
        if role_ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(SysRolePermission::select_by_column(pool!(), "role_id", role_ids).await?)
        // Err(Error::from("hello"))
    }

    pub async fn find_user_permission(
        &self,
        user_id: &str,
        all_res: &BTreeMap<String, SysPermissionVO>,
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

    pub async fn loop_find_childs(&self, arg: &mut SysRoleVO, all: &HashMap<String, SysRole>) {
        let mut children = vec![];
        for (k, v) in all {
            if v.parent_id.is_some() && v.parent_id.eq(&arg.id) {
                let mut item = SysRoleVO::from(v.clone());
                self.loop_find_childs(&mut item, all);
                children.push(item)
            }
        }
        if !children.is_empty() {
            arg.childs = Some(children);
        }
        // Err(Error::from("hello"))
    }

    /// 角色添加
    pub async fn add_role(&self, arg: RoleAddDTO) -> Result<(u64, String)> {
        let mut role = SysRole::from(arg);
        let result = (
            SysRole::insert(pool!(), &role).await?.rows_affected,
            role.id.clone().unwrap(),
        );

        self.update_cache().await?;
        Ok(result)
    }

    /// 角色删除
    pub async fn remove(&self, id: &str) -> Result<(u64)> {
        // trash service 暂时不需要
        let result = SysRole::delete_by_column(pool!(), field_name!(SysRole.id), id).await?;
        self.update_cache().await?;
        Ok(result.rows_affected)
    }

    /// 角色修改
    pub async fn edit(&self, arg: RoleEditDTO) -> Result<u64> {
        let role = SysRole::from(arg);
        let result = SysRole::update_by_column(pool!(), &role, field_name!(SysRole.id)).await;
        self.update_cache().await?;
        Ok(result?.rows_affected)
    }
}
