use rbatis::sql::Page;

use crate::error::{Error, Result};

struct SysDictVO {}
pub struct SysDictService {}

impl SysDictService {
    pub async fn page(&self) -> Result<Page<SysDictVO>> {
        Err(Error::from("zan wei wancheng"))
    }
}
