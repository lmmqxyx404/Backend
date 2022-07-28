pub struct PasswordEncoder {}

impl PasswordEncoder {
    /// 加密明文密码
    pub fn encode(raw_password: &str) -> String {
        let encode_str = md5::compute(raw_password);
        format!("{:x}", encode_str)
    }

    pub fn verify(password: &str, raw_password: &str) -> bool {
        if password.eq(raw_password) {
            return true;
        }
        let hashed = PasswordEncoder::encode(raw_password);
        password.eq(&hashed)
    }
}

#[cfg(test)]
mod test {
    use crate::util::password_encoder::PasswordEncoder;

    #[test]
    fn test_encode() {
        let s = PasswordEncoder::encode("123456789");
        println!("{}", s);
    }
    #[test]
    fn test_verify() {
        assert!(PasswordEncoder::verify("password", "password"));
        let encode_str = PasswordEncoder::encode("123456");
        assert!(PasswordEncoder::verify("123456", &encode_str));
    }
}
