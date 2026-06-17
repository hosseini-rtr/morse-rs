use crate::utils::alphabet::ENCODE_MAP;

pub fn encode(value: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for v in value.to_uppercase().chars() {
        if v == ' ' {
            result.push("/".to_string());
        } else if let Some(code) = ENCODE_MAP.get(&v) {
            result.push(code.to_string());
        }
    }

    result
}