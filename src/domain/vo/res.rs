use std::collections::HashMap;

use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::table::SysRes;

/// 权限资源表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysResVO {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<FastDateTime>,
    pub childs: Option<Vec<SysResVO>>,
}

impl From<SysRes> for SysResVO {
    fn from(arg: SysRes) -> Self {
        Self {
            id: arg.id,
            parent_id: arg.parent_id,
            name: arg.name,
            permission: arg.permission,
            path: arg.path,
            del: arg.del,
            create_date: arg.create_date,
            childs: None,
        }
    }
}

impl SysResVO {
    pub fn get_father_id(&self) -> &Option<String> {
        &self.parent_id
    }

    pub fn set_childs_recursive(&mut self, all_record: &HashMap<String, Self>) {
        let mut childs: Option<Vec<Self>> = None;
        if self.id.is_some() {
            for (key, val) in all_record {
                if val.get_father_id().is_some() && self.id.eq(&val.get_father_id()) {
                    let mut item = val.clone();
                    item.set_childs_recursive(all_record);
                    match &mut childs {
                        Some(childs) => {
                            childs.push(item);
                        }
                        None => {
                            let mut vec = vec![];
                            vec.push(item);
                            childs = Some(vec);
                        }
                    }
                }
            }
        }
        if childs.is_some() {
            self.childs = Some(childs.unwrap());
        }
    }
}
