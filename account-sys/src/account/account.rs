use super::error::{AccountError, Result};
use std::collections::HashMap;

// 声明一个账户结构体, 用于登陆
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Account {
    pub(crate) username: Option<String>,
}

// 声明一个账户信息结构体，用于保存用户信息
pub struct AccountInfo {
    pub eth_address: String,
    pub btc_address: String,
    pub xmr_address: String,
}

// 账户与账户信息的对应关系
pub type Accounts = HashMap<Account, AccountInfo>;

impl Account {
    /// 新建账户
    pub fn new(username: &str) -> Result<Account> {
        let account = Account {
            username: Some(String::from(username)),
        };

        // 检查账户信息
        match check_account(&account) {
            Ok(_) => Ok(account),
            Err(e) => Err(e),
        }

        // 为账户生成keystore
        
    }
}

/// 检查账户信息是否完备
fn check_account(account: &Account) -> Result<bool> {
    let mut result: Result<bool> = Err(AccountError::ShortName);

    if let Account {
        username: Some(username),
    } = account
    {
        result = match check_username(username) {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        };
    };

    return result;
}

/// 检查username是否重复
fn check_username(username: &String) -> Result<bool> {
    match username.as_str().len() {
        0 => Err(AccountError::EmptyParam),
        1 | 2 => Err(AccountError::ShortName),
        _ => Ok(true),
    }
}