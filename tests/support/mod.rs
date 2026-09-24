pub mod methods;

pub fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("../../src/api-contract.json")).unwrap()
}
