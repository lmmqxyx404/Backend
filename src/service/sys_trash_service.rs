use rbatis::{object_id::ObjectId, rbdc::datetime::FastDateTime};
use serde::Serialize;

use crate::{domain::table::SysTrash, error::Error, pool};

/// 垃圾桶服务，回收垃圾，各种删除操作，记录下来
pub struct SysTrashService {}

impl SysTrashService {
    pub async fn add<T>(&self, table_name: &str, args: &[T]) -> Result<u64, Error>
    where
        T: Serialize,
    {
        if args.is_empty() {
            return Ok(0);
        }
        let mut trashs: Vec<SysTrash> = Vec::with_capacity(args.len());

        for x in args {
            trashs.push(SysTrash {
                id: Some(ObjectId::new().to_string().into()),
                table_name: Some(table_name.to_string()),
                date: Some(serde_json::to_string(x).unwrap_or_default()),
                create_date: Some(FastDateTime::now()),
            })
        }
        Ok(SysTrash::insert_batch(pool!(), &trashs, 20)
            .await?
            .rows_affected)
    }
}
