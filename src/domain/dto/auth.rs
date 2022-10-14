/// 授权校验
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct SysAuthDTO {
    pub acces_token: String,
    pub path: String,
}
