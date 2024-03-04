use std::collections::HashMap;

use crate::domain::table::SysPermission;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

// use crate::domain::table::SysRes;

/// 权限资源表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysPermissionVO {
    pub id: Option<String>,
    // father id(can empty)
    pub parent_id: Option<String>,
    pub name: Option<String>,
    // permission
    pub permission: Option<String>,
    // menu path
    pub path: Option<String>,
    // pub del: Option<i32>,
    pub create_date: Option<DateTime>,
    pub childs: Option<Vec<SysPermissionVO>>,
}

impl From<SysPermission> for SysPermissionVO {
    fn from(arg: SysPermission) -> Self {
        Self {
            id: arg.id,
            parent_id: arg.parent_id,
            name: arg.name,
            permission: arg.permission,
            path: arg.path,
            // del: arg.del,
            create_date: arg.create_date,
            childs: None,
        }
    }
}

impl SysPermissionVO {
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
