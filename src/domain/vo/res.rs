use serde::{Deserialize, Serialize};

/// 权限资源表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysResVO {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
}
