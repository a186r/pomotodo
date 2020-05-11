use ethkey::prelude::*;
use ethsign_crypto::Keccak256;

pub fn test_key() {
    let key = EthAccount::load_or_generate("./res/keystore", "passwd")
        .expect("should load or generate new eth key");
    println!("{:?}", key.address());

    let str = String::from("test string");
    
    let message = Keccak256::keccak256(str.as_bytes());

    let signature = key.sign(&message).unwrap();

    let result = key.verify(&signature, &message).unwrap();
    println!(
        "{}",
        if result {
            "verification ok"
        } else {
            "wrong signature"
        }
    );
}

// 单元测试在测试的每个文件中
#[test]
fn test() {
    test_key();
}
