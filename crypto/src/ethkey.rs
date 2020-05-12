use ethkey::prelude::*;
use ethsign_crypto::Keccak256;
use ethkey::Error;

pub struct EtherAccount {
    account: Box<EthAccount>,
}

impl EtherAccount {
    pub fn generate_eth_account(file_path: &str, password: &str) -> EtherAccount {
        let eth_key =
            EthAccount::load_or_generate(file_path, password).expect("加载或者生成一个新的eth key");
        EtherAccount { account: eth_key }
    }

    pub fn sign_for_string(&self, msg: &str) -> Signature {
        self.account.sign(&Keccak256::keccak256(&msg)).unwrap()
    }

    pub fn verify(&self, sign: &Signature, msg: &str) -> bool {
        self.account
            .verify(sign, &Keccak256::keccak256(&msg))
            .unwrap()
    }

    pub fn change_password(&self, new_password: &str) -> Result<(), Error>{
        self.account.change_password(new_password)
    }
}

#[test]
fn ethkey_test() {
    // 初始化eth_account
    let ether_account = EtherAccount::generate_eth_account("./res/keystore2", "123");
    // 签名
    let signature = ether_account.sign_for_string("Hello World");
    // 验证签名
    let result = ether_account.verify(&signature, "Hello World");
    assert_eq!(result, true);

    // 修改密码
    // ether_account.change_password("123");
}
