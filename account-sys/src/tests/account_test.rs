#[allow(unused_imports)]
use crate::account::account::Account;

#[test]
fn test_account_new() {
    let account1 = Account::new("a186r", "password");
    println!("{:?}", account1);
}
