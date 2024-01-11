use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error,
};
use futures_util::future::LocalBoxFuture;

use crate::{domain::vo::RespVO, service::CONTEXT};

use super::auth::{check_auth, checked_token, is_white_list_api};

pub struct Auth;

impl<S: 'static> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx).map_err(Into::into)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        let token = req
            .headers()
            .get("access_token")
            .map(|v| v.to_str().unwrap_or_default().to_string())
            .unwrap_or_default();

        let path = req.path().to_string();

        Box::pin(async move {
            if !CONTEXT.config.debug {
                if !is_white_list_api(&path) {
                    match checked_token(&token, &path).await {
                        Ok(data) => match check_auth(&data, &path).await {
                            Ok(_) => {}
                            Err(e) => {
                                let resp: RespVO<String> = RespVO {
                                    code: Some("-1".to_string()),
                                    msg: Some(format!("无权限访问:{}", e.to_string())),
                                    data: None,
                                };
                                return Ok(req.into_response(resp.resp_json()));
                            }
                        },
                        Err(e) => {
                            let resp: RespVO<String> = RespVO {
                                code: Some("-1".to_string()),
                                msg: Some(format!("Unauthorized for :{}", e.to_string())),
                                data: None,
                            };
                            return Err(ErrorUnauthorized(serde_json::json!((&resp).to_string())));
                        }
                    }
                }
            }

            let res = service.call(req).await?;
            Ok(res)
        })
    }
}
