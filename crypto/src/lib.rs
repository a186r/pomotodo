// use ethkey::prelude::*;
// use ethsign_crypto::Keccak256;

// /// 初始化或者载入一个eth key
// pub fn generate_eth_account() -> Box<EthAccount> {
//     EthAccount::load_or_generate("./res/keystore", "passwd")
//         .expect("应该加载或者生成一个新的eth key")
// }

// /// 签名字符串
// pub fn sign_for_string(message: &str) -> Signature {
//     sign_for_bytes(&Keccak256::keccak256(message.as_bytes()))
// }

// /// 签名u8类型的，长度为32的数组
// pub fn sign_for_bytes(message: &[u8; 32]) -> Signature {
//     generate_eth_account().sign(&message).unwrap()
// }

// /// 验证签名
// pub fn verify(sign: &Signature, msg: &str) -> bool {
//     generate_eth_account()
//         .verify(sign, &Keccak256::keccak256(msg.as_bytes()))
//         .unwrap()
// }

// #[test]
// fn ethkey_test() {
//     // 签名字符串
//     let signature = sign_for_string("Hello World!");
//     // 验证签名
//     let result = verify(&signature, "Hello World!");

//     assert_eq!(result, true);
// }
pub mod ethkey;