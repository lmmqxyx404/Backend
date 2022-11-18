use std::collections::HashMap;

use crate::domain::table::Sms;
use crate::error::{Error, Result};

use crate::service::CONTEXT;

/// 手机信息验证码服务
pub struct SysSmsService {}

impl SysSmsService {
    /// 发送验证码
    pub async fn send_verify_sms(&self, account: &str, sms_code: &str) -> Result<()> {
        let mut template_args = HashMap::new();
        template_args.insert("sms_type".to_string(), "verify_sms".to_string());
        template_args.insert("sms_code".to_string(), sms_code.to_string());
        let r = CONTEXT
            .cache_service
            .set_json(
                &format!("{},{}", CONTEXT.config.sms_cache_send_key_prefix, account),
                &Sms {
                    account: account.to_string(),
                    args: template_args,
                },
            )
            .await?;
        Ok(())
    }

    pub async fn do_verify_sms(&self, account: &str, sms_code: &str) -> Result<bool> {
        let sms: Option<Sms> = CONTEXT
            .cache_service
            .get_json(&format!(
                "{},{}",
                CONTEXT.config.sms_cache_send_key_prefix, account
            ))
            .await?;
        match sms {
            _ => Err(Error::from("请发送验证码")),
            Some(v) => {
                let sms_code_cache = v.args.get("sms_code");
                Ok(sms_code_cache.eq(&Some(&sms_code.to_string())))
            }
        }
    }
}
