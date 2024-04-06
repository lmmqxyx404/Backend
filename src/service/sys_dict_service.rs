use rbatis::{Page, PageRequest};

use crate::{
    domain::{
        dto::dict::{DictEditDTO, DictPageDTO},
        table::SysDict,
        vo::dict::SysDictVO,
    },
    error::{Error, Result},
    pool,
};

use super::CONTEXT;

const DICT_KEY: &'static str = "sys_dict:all";
pub struct SysDictService {}

impl SysDictService {
    /// 字典分页
    pub async fn page(&self, arg: &DictPageDTO) -> Result<Page<SysDictVO>> {
        let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = SysDict::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        let page = Page::<SysDictVO>::from(data);
        Ok(page)
    }

    /// 添加字典
    pub async fn add(&self, arg: &SysDict) -> Result<u64> {
        let old = SysDict::select_by_column(
            pool!(),
            rbatis::field_name!(SysDict.id),
            arg.id.as_deref().unwrap_or_default(),
        )
        .await?;

        if old.len() > 0 {
            return Err(Error::from("zan wei wancheng"));
        }
        let result = Ok(SysDict::insert(pool!(), &arg).await?.rows_affected);
        self.update_cache().await?;
        result
    }

    /// 修改字典
    pub async fn edit(&self, arg: &DictEditDTO) -> Result<u64> {
        let data = SysDict::from(arg);
        // TODO: 以后逻辑可能会改
        let res = SysDict::update_by_column(pool!(), &data, "id").await?;
        self.update_cache().await?;
        Ok(res.rows_affected)
    }

    /// 删除字典
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let targets = SysDict::select_by_column(pool!(), "id", id).await?;
        let r = SysDict::delete_by_column(pool!(), "id", id).await?;
        if r.rows_affected > 0 {
            self.update_cache().await?;
            CONTEXT.sys_trash_service.add("sys_dict", &targets).await?;
        }
        Ok(r.rows_affected)
        // Err(Error::from("zan wei wancheng"))
    }

    /// 更新字典
    pub async fn update_cache(&self) -> Result<()> {
        let all = SysDict::select_all(pool!()).await?;
        CONTEXT.cache_service.set_json(DICT_KEY, &all).await?;
        Ok(())
    }
}
