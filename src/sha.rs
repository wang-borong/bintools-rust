use sha2::{Sha256, Digest};

fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    strs.join("")
}

pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);

    to_hex_string(hasher.finalize().to_vec())
}
