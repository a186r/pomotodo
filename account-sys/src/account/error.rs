use std::fmt;

pub type Result<T> = std::result::Result<T, AccountError>;

#[derive(Debug)]
pub enum AccountError {
    ShortName,
    EmptyParam,
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccountError::ShortName => write!(f, "名字太短了兄弟"),
            AccountError::EmptyParam => write!(f, "不能为空"),
        }
    }
}
