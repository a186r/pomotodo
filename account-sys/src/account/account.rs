use std::collections::HashMap;

// 声明一个账户结构体, 用于登陆
#[derive(PartialEq, Eq, Hash)]
pub struct Account {
    pub(crate) username: Option<String>,
    pub(crate) email: Option<String>,
    pub(crate) password: Option<String>,
}

// 声明一个账户信息结构体，用于保存用户信息
pub struct AccountInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub eth_address: String,
    pub btc_address: String,
    pub xmr_address: String,
}

// 账户与账户信息的对应关系
pub type Accounts = HashMap<Account, AccountInfo>;

impl Account {
    /// 新建账户
    pub fn new(username: &str, email: &str, password: &str) -> Account {
        let account = Account {
            username: Some(String::from(username)),
            email: Some(String::from(email)),
            password: Some(String::from(password)),
        };
        // 校验账户字段
        check_account(&account);
        return account;
    }

    /// 修改密码
    pub fn change_password(&mut self, password: &str) -> bool {
        self.password = Some(String::from(password));
        return true;
    }
}

/// 检查账户信息是否完备
/// 将account存储到数据库里
fn check_account(account: &Account) -> bool {
    if let Account {
        username: Some(username),
        email: Some(email),
        password: Some(password),
    } = account
    {
        println!(
            "username: {:?}, email: {:?}, password: {:?}",
            username, email, password
        );
        return true;
    }
    false
}

/// 邮件验证
/// 调用这个接口，检查账户是否创建，如果已经创建，则发送验证邮件
/// 验证邮件中包含一个验证码
/// `email` - 用户邮件信息
#[allow(dead_code)]
fn check_email(_email: Option<String>) -> bool {
    return true;
}
