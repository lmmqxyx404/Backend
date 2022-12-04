use std::collections::{BTreeMap, HashMap};

use rbatis::sql::{Page, PageRequest};

use crate::{
    domain::{
        dto::res::{ResEditDTO, ResPageDTO},
        table::SysRes,
        vo::res::SysResVO,
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
    pub async fn page(&self, arg: &ResPageDTO) -> Result<Page<SysResVO>> {
        let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = SysRes::select_page(pool!(), &PageRequest::from(arg), &arg).await?;
        let all_res = self.finds_all_map().await?;
        let mut all_res_vo = HashMap::new();
        for (k, v) in all_res {
            all_res_vo.insert(k, v);
        }
        let mut page = Page::<SysResVO>::from(data);
        for vo in &mut page.records {
            vo.set_childs_recursive(&all_res_vo);
        }
        // for vo in &mut page.re
        Ok(page)
    }

    /// 添加资源
    pub async fn add(&self, arg: &SysRes) -> Result<u64> {
        let old = SysRes::select_by_permission_or_name(
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
        let res = Ok(SysRes::insert(pool!(), &arg).await?.rows_affected);
        self.update_cache().await?;
        res
    }

    /// 修改资源
    pub async fn edit(&self, arg: &ResEditDTO) -> Result<u64> {
        let data = SysRes::from(arg);
        let res = SysRes::update_by_column(pool!(), &data, "id").await?;
        self.update_cache().await?;
        Ok(res.rows_affected)
    }

    /// 删除资源
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let trash = SysRes::select_by_column(pool!(), "id", id).await?;
        let num = SysRes::delete_by_column(pool!(), "id", id)
            .await?
            .rows_affected;

        Err(Error::from("_"))
    }

    /// 更新缓存
    pub async fn update_cache(&self) -> Result<Vec<SysResVO>> {
        let all = SysRes::select_all(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_KEY, &all).await?;
        let mut v = vec![];
        for x in all {
            v.push(x.into());
        }
        Ok(v)
        // Err(Error::from("temporary"))
    }

    /// 查找res数组
    pub async fn finds_all(&self) -> Result<Vec<SysResVO>> {
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysRes>>>(RES_KEY)
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

    pub fn make_res_ids(&self, args: &Vec<SysResVO>) -> Vec<String> {
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

    pub async fn finds_all_map(&self) -> Result<BTreeMap<String, SysResVO>> {
        Err(Error::from("temporary"))
    }
}
