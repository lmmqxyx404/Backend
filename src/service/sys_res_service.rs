use std::collections::{BTreeMap, HashMap};

use rbatis::{Page, PageRequest};

use crate::{
    domain::{
        dto::permission::{ResEditDTO, ResPageDTO},
        table::SysPermission,
        vo::res::SysPermissionVO,
    },
    error::{Error, Result},
    pool,
};

use super::{sys_role_service::SysRoleService, CONTEXT};

const RES_KEY: &'static str = "sys_res:all";
/// 系统资源服务
pub struct SysResService {}

impl SysResService {
    /// 资源分页
    pub async fn page(&self, arg: &ResPageDTO) -> Result<Page<SysPermissionVO>> {
        let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = SysPermission::select_page(pool!(), &PageRequest::from(arg), &arg).await?;
        let all_res = self.finds_all_map().await?;
        let mut all_res_vo = HashMap::new();
        for (k, v) in all_res {
            all_res_vo.insert(k, v);
        }
        let mut page = Page::<SysPermissionVO>::from(data);
        for vo in &mut page.records {
            vo.set_childs_recursive(&all_res_vo);
        }
        // for vo in &mut page.re
        Ok(page)
    }

    /// 添加资源
    pub async fn add(&self, arg: &SysPermission) -> Result<u64> {
        let old = SysPermission::select_by_permission_or_name(
            pool!(),
            arg.permission.as_deref().unwrap_or_default(),
            arg.name.as_deref().unwrap_or_default(),
        )
        .await?;
        if old.len() > 0 {
            return Err(Error::from(format!(
                "权限已经存在了， 权限{:?}",
                rbatis::make_table_field_vec!(old, name)
            )));
        }
        let res = Ok(SysPermission::insert(pool!(), &arg).await?.rows_affected);
        self.update_cache().await?;
        res
    }

    /// 修改资源
    pub async fn edit(&self, arg: &ResEditDTO) -> Result<u64> {
        let data = SysPermission::from(arg);
        let res = SysPermission::update_by_column(pool!(), &data, "id").await?;
        self.update_cache().await?;
        Ok(res.rows_affected)
    }

    /// 删除资源
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let trash = SysPermission::select_by_column(pool!(), "id", id).await?;
        let num = SysPermission::delete_by_column(pool!(), "id", id)
            .await?
            .rows_affected;

        Err(Error::from("_"))
    }

    /// 更新缓存
    pub async fn update_cache(&self) -> Result<Vec<SysPermissionVO>> {
        let all = SysPermission::select_all(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_KEY, &all).await?;
        let mut v = vec![];
        for x in all {
            v.push(x.into());
        }
        Ok(v)
        // Err(Error::from("temporary"))
    }

    /// 查找res数组
    pub async fn finds_all(&self) -> Result<Vec<SysPermissionVO>> {
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysPermission>>>(RES_KEY)
            .await;

        if js.is_err()
            || js.as_ref().unwrap().is_none()
            || js.as_ref().unwrap().as_ref().unwrap().is_empty()
        {
            let all = self.update_cache().await?;
            return Ok(all);
        }
        /// 之后可以添加代码实现打印日志
        let mut arr = vec![];
        if let Ok(v) = js {
            for x in v.unwrap_or(vec![]) {
                arr.push(x.into());
            }
        }
        Ok(arr)
        //Err(Error::from("temporary"))
    }

    pub fn make_res_ids(&self, args: &Vec<SysPermissionVO>) -> Vec<String> {
        let mut ids = vec![];
        for x in args {
            ids.push(x.id.as_deref().unwrap_or_default().to_string());
            if let Some(childs) = &x.childs {
                let child_ids = rbatis::make_table_field_vec!(childs, id);
                for child_id in child_ids {
                    ids.push(child_id);
                }
            }
        }
        ids
    }

    /// 登陆的时候，就需要用
    pub async fn finds_all_map(&self) -> Result<BTreeMap<String, SysPermissionVO>> {
        let all = self.finds_all().await?;
        let mut result = BTreeMap::new();
        for x in all {
            result.insert(x.id.as_deref().unwrap_or_default().to_string(), x);
        }
        Ok(result)
        // Err(Error::from("temporary"))
    }
}
