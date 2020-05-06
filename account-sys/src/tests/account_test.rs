#[allow(unused_imports)]
use crate::account::account::Account;

#[test]
fn test_account_new() {
    let username= "bob";
    let _account = Account::new(username);
}
