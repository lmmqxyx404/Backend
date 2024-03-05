use crate::domain::table::tables::SysUserRole;


/// 后台用户角色表 过程宏
/// 只有使用了crud！ 这个宏之后，才能生成之后的宏

crud!(SysUserRole {});

impl_select!(SysUserRole{select_list_in_user_id(user_ids:&[String])=>
"`where user_id in (`
trim ',': for _,v in user_ids:
`#{v},`
`)`"});
