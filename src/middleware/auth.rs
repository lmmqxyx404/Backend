use crate::{domain::vo::jwt::JWT_Token, service::CONTEXT};

///校验token是否有效，未过期
pub fn checked_token(token: &str, path: &str) -> Result<JWT_Token, crate::error::Error> {
    let token = JWT_Token::verify(&CONTEXT.config.jwt_secret, token);
    match token {
        Ok(token) => {
            return Ok(token);
        }
        Err(e) => {
            return Err(crate::error::Error::from(e.to_string()));
        }
    }
}

///是否处在白名单接口中
pub fn is_white_list_api(path: &str) -> bool {
    if path.eq("/") {
        return true;
    }
    for x in &CONTEXT.config.white_list_api {
        if x.contains(path) {
            return true;
        }
    }
    return false;
}

///权限校验
pub async fn check_auth(token: &JWT_Token, path: &str) -> Result<(), crate::error::Error> {
    let sys_res = CONTEXT.sys_res_service.finds_all().await?;

    for token_permission in &token.permissions {
        for x in &sys_res {
            match &x.permission {
                Some(permission) => match &x.path {
                    None => {}
                    Some(x_path) => {
                        if permission.eq(token_permission) && path.contains(x_path) {
                            return Ok(());
                        }
                    }
                },
                _ => {}
            }
        }
    }
    return Err(crate::error::Error::from("无权限访问"));
}
