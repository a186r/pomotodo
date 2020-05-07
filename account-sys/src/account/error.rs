use std::fmt;

pub type Result<T> = std::result::Result<T, AccountError>;

#[derive(Debug, PartialEq)]
pub enum AccountError {
    EmptyParam,
    ShortName,
    RepeatName,
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccountError::EmptyParam => write!(f, "username不能为空"),
            AccountError::ShortName => write!(f, "名字太短了，请重新填写username"),
            AccountError::RepeatName => write!(f, "名字重复了，请重新填写username"),
        }
    }
}
