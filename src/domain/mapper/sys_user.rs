use crate::domain::table::tables::SysUser;

/// 后台用户表 过程宏
/// 只有使用了crud！ 这个宏之后，才能生成之后的宏
crud!(SysUser {});

impl_select_page!(SysUser{select_page(name:&str,account:&str)=>
  "`where 0 = 0`
  if name != '':
    ` and name like #{'%'+name+'%'}`
  if account != '':
    ` and account like #{'%'+account+'%'}`
  if !sql.contains('count'):
   ` order by create_date desc`"});

