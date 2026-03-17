use std::fs::File;
use std::io::{Read, Write};

use orion::hazardous::{
    aead::xchacha20poly1305::{open, Nonce, SecretKey},
    mac::poly1305::POLY1305_OUTSIZE,
    stream::xchacha20::XCHACHA_NONCESIZE
};

use orion::hazardous::stream::chacha20::CHACHA_KEYSIZE;

use crate::crypto::crypto_utils::{simple_split_encrypted, create_key};

const CHUNK_SIZE: usize = 128;

fn decrypt_core(
    dist: &mut File, 
    contents: Vec<u8>,
    key: &SecretKey, 
    nonce: Nonce
) {
    let split = simple_split_encrypted(contents.as_slice());
    let mut output = vec![0u8; split.1.len() - POLY1305_OUTSIZE];

    match open(&key, &nonce, split.1.as_slice(), Some(split.0.as_slice()), &mut output) {
        Ok(t) => t,
        Err(_e) => eprintln!("Error with password!"),
    };
    dist.write(&output.as_slice()).unwrap();
}

pub fn decrypt_large_file(
    file_path: &str, 
    output_path: &str,
    password: String
) -> Result<(), orion::errors::UnknownCryptoError> {
    let mut input_file = File::open(file_path).expect("Failed to open input file");
    let mut output_file = File::create(output_path).expect("Failed to create output file");

    let mut src: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut src).expect("Failed to read input file");

    let nonce = src[..XCHACHA_NONCESIZE].to_vec();

    src = src[XCHACHA_NONCESIZE..].to_vec();

    let key = create_key(password, nonce.clone());
    let nonce = Nonce::from_slice(nonce.as_slice()).unwrap();

    for (_n_chunk, src_chunk) in src.chunks(CHUNK_SIZE + CHACHA_KEYSIZE + POLY1305_OUTSIZE).enumerate() {
        decrypt_core(&mut output_file, src_chunk.to_vec(), &key, nonce);
    }

    Ok(())
}