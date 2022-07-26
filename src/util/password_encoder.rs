pub struct PasswordEncoder {}

impl PasswordEncoder {
    /// 加密明文密码
    pub fn encode(raw_password: &str) -> String {
        let encode_str = String::from("hello");
        encode_str
    }

    pub fn verify() -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use crate::util::password_encoder::PasswordEncoder;

    #[test]
    fn test_encode() {
        let s = PasswordEncoder::encode("123456");
        println!("{}", s);
    }
}
