#[allow(unused_imports)]
use crate::account::account::Account;

#[test]
fn test_account_new() {
    let (username, email, password) = ("bob", "hi.bob@gmail.com", "bob#gmail");
    let mut account = Account::new(username, email, password);
}

#[test]
fn test_change_password() {
    let mut account = Account::new("alina", "alina@gmail.com", "alina#password");
    let result = Account::change_password(&mut account, "alina#pass");
    assert_eq!(true, result);
}
