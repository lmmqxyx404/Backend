use std::{
    future::{ready, Ready},
    rc::Rc,
};

use futures_util::future::LocalBoxFuture;

use crate::{domain::vo::RespVO, service::CONTEXT};

use super::auth::{check_auth, checked_token, is_white_list_api};

pub struct Auth;

