use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::Rng;
use std::fs;
use std::io::{Read, Write};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: ./encryptor <encrypt|decrypt> <file> <keyfile>");
        std::process::exit(1);
    }
    let action = &args[1];
    let input_file = &args[2];
    let key_file = &args[3];

    let key_data = fs::read(key_file)?;
    let key = Key::from_slice(&key_data);
    let cipher = Aes256Gcm::new(key);

    match action.as_str() {
        "encrypt" => {
            let plaintext = fs::read(input_file)?;
            let mut rng = rand::thread_rng();
            let mut nonce_bytes = [0u8; 12];
            rng.fill(&mut nonce_bytes);
            let nonce = Nonce::from_slice(&nonce_bytes);
            let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).unwrap();
            let mut output = Vec::new();
            output.extend_from_slice(&nonce_bytes);
            output.extend_from_slice(&ciphertext);
            fs::write(format!("{}.enc", input_file), output)?;
            println!("Encrypted: {}.enc", input_file);
        }
        "decrypt" => {
            let data = fs::read(input_file)?;
            let nonce = Nonce::from_slice(&data[0..12]);
            let ciphertext = &data[12..];
            let plaintext = cipher.decrypt(nonce, ciphertext).unwrap();
            let out_path = input_file.replace(".enc", "");
            fs::write(&out_path, plaintext)?;
            println!("Decrypted: {}", out_path);
        }
        _ => eprintln!("Unknown action"),
    }
    Ok(())
}
