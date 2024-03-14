use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::Deserialize;
use near_sdk::{env, serde_json};

#[derive(Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct Input {
    keys: Vec<Base64VecU8>,
}

#[no_mangle]
pub fn clean() {
    let input = serde_json::from_slice::<Input>(&env::input().unwrap()).unwrap();
    input.keys.iter().for_each(|key| {
        env::storage_remove(&key.0);
    })
}
