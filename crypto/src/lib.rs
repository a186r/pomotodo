use ethkey::prelude::*;

pub fn test_key() {
    let key = EthAccount::load_or_generate("./res/keystore", "passwd")
        .expect("should load or generate new eth key");
    println!("{:?}", key.address());

    let message = [
        9_u8, 1_u8, 2_u8, 'e' as u8, 'c' as u8, 8_u8, 0_u8, 3_u8, 'b' as u8, 2_u8, 'c' as u8,
        'e' as u8, 4_u8, 9_u8, 'e' as u8, 4_u8, 'a' as u8, 5_u8, 4_u8, 1_u8, 0_u8, 6_u8, 8_u8,
        'd' as u8, 4_u8, 9_u8, 5_u8, 'a' as u8, 'b' as u8, 5_u8, 7_u8, 0_u8,
    ];

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

#[test]
fn test() {
    test_key();
}
