pub trait isEmptyString {
    fn is_empty(&self) -> bool;
}

impl isEmptyString for Option<String> {
    fn is_empty(&self) -> bool {
        match self {
            Some(s) => s.is_empty(),
            _ => true,
        }
    }
}
