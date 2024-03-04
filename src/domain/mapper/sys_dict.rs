// use crate::domain::dto::dict::DictPageDTO;
// use crate::domain::table::SysDict;
use crate::domain::table::*;

// 系统字典表 过程宏
// 只有使用了crud！ 这个宏之后，才能生成之后的宏

/* 
crud!(SysDict {});

impl_select_page!(SysDict{select_page(dto: &crate::domain::dto::dict::DictPageDTO) =>
"`where id!=''`
if dto.code != null:
    ` and code = #{dto.code}`
if dto.name != null:
    ` and name = #{dto.name}`
if !sql.contains('count'):
`order by create data`"
});
*/