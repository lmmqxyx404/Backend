use serde::{Deserializer, Serializer};

#[derive(Clone)]
pub enum LoginCheck {
    NoCheck,
    PasswordCheck,
    PasswordImgCodeCheck,
    PhoneCodeCheck,
}
