#![cfg_attr(windows, feature(abi_vectorcall))]

mod factorizator;
mod aesige;

use ext_php_rs::prelude::*;
use ext_php_rs::binary::Binary;

#[php_const]
pub const TGCRYPTO_VERSION: &str = "0.0.1";

#[php_function]
pub fn tg_factorize(pq: u64) -> Vec<u64> {
    factorizator::factor(pq)
}

#[php_function]
pub fn tg_encrypt_ige(plain: Binary<u8>, key: Binary<u8>, iv: Binary<u8>) -> Result<Binary<u8>, String> {
    let plain_bytes = plain.as_ref();

    let key_bytes = key.as_ref();
    let iv_bytes = iv.as_ref();

    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(key_bytes);
    
    let mut iv_array = [0u8; 32];
    iv_array.copy_from_slice(iv_bytes);

    let cipher = aesige::aes_ige_encrypt(plain_bytes, &key_array, &iv_array);

    Ok(Binary::from(cipher))
}

#[php_function]
pub fn tg_decrypt_ige(cipher: Binary<u8>, key: Binary<u8>, iv: Binary<u8>) -> Result<Binary<u8>, String> {
    let cipher_bytes = cipher.as_ref();

    let key_bytes = key.as_ref();
    let iv_bytes = iv.as_ref();

    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(key_bytes);
    
    let mut iv_array = [0u8; 32];
    iv_array.copy_from_slice(iv_bytes);

    let plain = aesige::aes_ige_decrypt(cipher_bytes, &key_array, &iv_array);

    Ok(Binary::from(plain))
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .constant(wrap_constant!(TGCRYPTO_VERSION))
        .function(wrap_function!(tg_factorize))
        .function(wrap_function!(tg_encrypt_ige))
        .function(wrap_function!(tg_decrypt_ige))
}
