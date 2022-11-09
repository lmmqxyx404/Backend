use crate::domain::dto::user::UserRoleAddDTO;
use crate::error::{Error, Result};

use crate::domain::table::tables::SysUserRole;

use crate::pool;
pub struct SysUserRoleService {}

impl SysUserRoleService {
    /// 添加角色
    /// 部分代码与原工程不一致
    pub async fn add(&self, arg: UserRoleAddDTO) -> Result<u64> {
        if arg.user_id.is_none() || arg.role_id.is_none() {
            return Err(Error::from("添加角色时用户和角色不能为空！"));
        }
        let user_id = arg.user_id.as_deref().unwrap().to_string();
        /// 形成角色
        let mut user_role = SysUserRole::from(arg);
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
    pub async fn find_user_role(&self) -> Result<u64> {
        Err(Error::from("未完全实现"))
    }
}
