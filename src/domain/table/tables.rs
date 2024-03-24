use log::LevelFilter;
use rbatis::{
    dark_std::defer,
    intercept_log::LogInterceptor,
    object_id::ObjectId,
    table_sync::{
        ColumMapper, MssqlTableMapper, MysqlTableMapper, PGTableMapper, SqliteTableMapper,
    },
    RBatis,
};
use serde::{Deserialize, Serialize};

use crate::{domain::dto::user::UserAddDTO, util::password_encoder::PasswordEncoder};

/// 创建角色时用的上
use rbatis::rbdc::datetime::DateTime;

use crate::domain::table::LoginCheck;

/// 后台用户表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUser {
    pub id: Option<String>,
    /// 邮箱登录|手机号登录
    pub account: Option<String>,
    pub password: Option<String>,
    pub login_check: Option<LoginCheck>,
    /// 用户名登录
    pub name: Option<String>,
    pub state: Option<i32>,
    // pub del: Option<i32>,
    pub create_date: Option<DateTime>,
}

/// 转化
impl From<UserAddDTO> for SysUser {
    fn from(arg: UserAddDTO) -> Self {
        SysUser {
            id: ObjectId::new().to_string().into(),
            account: arg.account.clone(),
            password: PasswordEncoder::encode(&arg.password.unwrap_or_default()).into(),
            name: arg.name.clone(),
            login_check: arg.login_check.clone(),
            state: Some(arg.state.unwrap_or(1)),
            create_date: DateTime::now().into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUserRole {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
    pub create_date: Option<DateTime>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SysRoleRes {
    pub id: Option<String>,
    /// 角色ID
    pub role_id: Option<String>,
    /// 资源ID
    pub res_id: Option<String>,
    pub create_date: Option<DateTime>,
}

/// 权限资源表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysPermission {
    pub id: Option<String>,
    //father id(can empty)
    pub parent_id: Option<String>,
    pub name: Option<String>,
    //permission
    pub permission: Option<String>,
    //menu path
    pub path: Option<String>,
    pub create_date: Option<DateTime>,
}

/// 角色表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysRole {
    pub id: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysTrash {
    pub id: Option<String>,
    pub table_name: Option<String>,
    pub date: Option<String>,
    pub create_date: Option<DateTime>,
}

/// 字典表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysDict {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<DateTime>,
}

///Role Permission relational tables (relational tables do not use logical deletion)
#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SysRolePermission {
    pub id: Option<String>,
    pub role_id: Option<String>,
    pub permission_id: Option<String>,
    pub create_date: Option<DateTime>,
}

pub async fn sync_tables(rb: &RBatis) {
    //disable log
    let log_intercept = rb.get_intercept::<LogInterceptor>().unwrap();
    let level = log_intercept.get_level_filter().clone();
    log_intercept.set_level_filter(LevelFilter::Off);
    defer!(|| {
        log_intercept.set_level_filter(level);
    });
    let mapper = {
        match rb.driver_type().unwrap() {
            "sqlite" => &SqliteTableMapper {} as &dyn ColumMapper,
            "mssql" => &MssqlTableMapper {} as &dyn ColumMapper,
            "mysql" => &MysqlTableMapper {} as &dyn ColumMapper,
            "postgres" => &PGTableMapper {} as &dyn ColumMapper,
            _ => {
                panic!("not find driver mapper")
            }
        }
    };
    let conn = rb.acquire().await.expect("connection database fail");
    let table = SysPermission {
        id: Some("".to_string()),
        parent_id: Some("".to_string()),
        name: Some("".to_string()),
        permission: Some("".to_string()),
        path: Some("".to_string()),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_permission").await;
    let table = SysRole {
        id: Some("".to_string()),
        parent_id: Some("".to_string()),
        name: Some("".to_string()),
        create_date: Some(DateTime::now()),
        del: None,
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_role").await;
    let table = SysRolePermission {
        id: Some("".to_string()),
        role_id: Some("".to_string()),
        permission_id: Some("".to_string()),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_role_permission").await;
    let table = SysUser {
        id: Some("".to_string()),
        account: Some("".to_string()),
        password: Some("".to_string()),
        name: Some("".to_string()),
        login_check: Some(LoginCheck::NoCheck),
        state: Some(0),
        create_date: Some(DateTime::now()),
        // del: None,
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_user").await;
    let table = SysUserRole {
        id: Some("".to_string()),
        user_id: Some("".to_string()),
        role_id: Some("".to_string()),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_user_role").await;
    let table = SysDict {
        id: Some("".to_string()),
        name: Some("".to_string()),
        code: Some("".to_string()),
        state: Some(0),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_dict").await;
    let table = SysTrash {
        id: Some("".to_string()),
        table_name: Some("".to_string()),
        date: Some("".to_string()),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_trash").await;
}

pub async fn sync_tables_data(rb: &RBatis) {
    let conn = rb.acquire().await.expect("init data fail");
    if let Ok(v) = SysUser::select_by_column(&conn, "id", "1").await {
        if v.len() > 0 {
            //if user exists,return
            return;
        }
    };
    let _ = SysUser::insert(
        &conn,
        &SysUser {
            id: Some("1".to_string()),
            account: Some("00000000000".to_string()),
            password: Some("e10adc3949ba59abbe56e057f20f883e".to_string()),
            name: Some("admin".to_string()),
            login_check: Some(LoginCheck::PasswordCheck),
            state: Some(1),
            // del: None,
            create_date: Some(DateTime::now()),
        },
    )
    .await;

    let _ = SysRole::insert(
        &conn,
        &SysRole {
            id: Some(1.to_string()),
            name: Some("admin".to_string()),
            parent_id: None,
            create_date: Some(DateTime::now()),
            del: None,
        },
    )
    .await;

    let _ = SysUserRole::insert(
        &conn,
        &SysUserRole {
            id: Some(1.to_string()),
            user_id: Some(1.to_string()),
            role_id: Some(1.to_string()),
            create_date: Some(DateTime::now()),
        },
    )
    .await;

    let sys_permissions = vec![
        SysPermission {
            id: Some(1.to_string()),
            parent_id: None,
            name: Some("1".to_string()),
            permission: Some("/".to_string()),
            path: Some("/".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(2.to_string()),
            parent_id: None,
            name: Some("dashboard".to_string()),
            permission: Some("dashboard".to_string()),
            path: Some("dashboard".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(3.to_string()),
            parent_id: None,
            name: Some("首页".to_string()),
            permission: Some("/".to_string()),
            path: Some("".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(4.to_string()),
            parent_id: None,
            name: Some("form".to_string()),
            permission: Some("form".to_string()),
            path: Some("form".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(5.to_string()),
            parent_id: None,
            name: Some("table".to_string()),
            permission: Some("table".to_string()),
            path: Some("table".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(6.to_string()),
            parent_id: None,
            name: Some("profile".to_string()),
            permission: Some("profile".to_string()),
            path: Some("profile".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(7.to_string()),
            parent_id: None,
            name: Some("result".to_string()),
            permission: Some("result".to_string()),
            path: Some("result".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(8.to_string()),
            parent_id: None,
            name: Some("exception".to_string()),
            permission: Some("exception".to_string()),
            path: Some("exception".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(9.to_string()),
            parent_id: None,
            name: Some("user".to_string()),
            permission: Some("user".to_string()),
            path: Some("user".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(10.to_string()),
            parent_id: None,
            name: Some("setting".to_string()),
            permission: Some("setting".to_string()),
            path: Some("setting".to_string()),
            create_date: Some(DateTime::now()),
        },
    ];

    let mut index = 1;
    for permission in sys_permissions {
        let _ = SysPermission::insert(&conn, &permission).await;
        let role_permission = SysRolePermission {
            id: Some(index.to_string()),
            role_id: Some(1.to_string()),
            permission_id: permission.id.clone(),
            create_date: Some(DateTime::now()),
        };
        let _ = SysRolePermission::insert(&conn, &role_permission).await;
        index += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::domain::table::enums::LoginCheck;

    use super::*;

    #[test]
    fn convert() {
        let user = UserAddDTO {
            account: Some("hello".to_string()),
            password: Some("String".to_string()),
            name: Some("String".to_string()),
            role_id: Some("String".to_string()),
            login_check: Some(LoginCheck::NoCheck),
            state: None,
        };
        let b = SysUser::from(user);
        println!("{:?}", b);
    }
}
