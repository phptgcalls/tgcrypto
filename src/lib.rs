#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use ext_php_rs::binary::Binary;
use std::collections::HashMap;

#[php_const]
pub const TGCRYPTO_VERSION: &str = "0.0.3";

#[php_function]
pub fn tg_factorize(pq: u64) -> HashMap<&'static str, u64> {
    let (p,q) = grammers_crypto::factorize::factorize(pq);
    let mut result = HashMap::new();
    result.insert("p",p);
    result.insert("q",q);
    result
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

    let cipher = grammers_crypto::encrypt_ige(plain_bytes, &key_array, &iv_array);

    // Ok(grammers_crypto::hex::to_hex(&cipher))
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

    let plain = grammers_crypto::decrypt_ige(cipher_bytes, &key_array, &iv_array);

    // Ok(grammers_crypto::hex::to_hex(&plain))
    Ok(Binary::from(plain))
}

/*
#[php_class(name = "ObfuscatedCipher")]
pub struct ObfuscatedCipher {
    rx: ctr::Ctr128BE<aes::Aes256>,
    tx: ctr::Ctr128BE<aes::Aes256>,
}

#[php_impl]
impl ObfuscatedCipher {
    #[php_constructor]
    pub fn new(init: Vec<u8>) -> PhpResult<Self> {
        let mut buf = [0u8; 64];
        buf.copy_from_slice(&init);
        let buf_rev = buf.iter().copied().rev().collect::<Vec<_>>();
        Ok(Self {
            rx: ctr::Ctr128BE::<aes::Aes256>::new(
                GenericArray::from_slice(&buf_rev[8..40]),
                GenericArray::from_slice(&buf_rev[40..56]),
            ),
            tx: ctr::Ctr128BE::<aes::Aes256>::new(
                GenericArray::from_slice(&buf[8..40]),
                GenericArray::from_slice(&buf[40..56]),
            ),
        })
    }
    #[php_method]
    pub fn encrypt(&mut self, mut data: Vec<u8>) -> Vec<u8> {
        self.tx.apply_keystream(&mut data);
        data
    }
    #[php_method]
    pub fn decrypt(&mut self, mut data: Vec<u8>) -> Vec<u8> {
        self.rx.apply_keystream(&mut data);
        data
    }
}
*/
/*
#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
*/

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .constant(wrap_constant!(TGCRYPTO_VERSION))
        .function(wrap_function!(tg_factorize))
        .function(wrap_function!(tg_encrypt_ige))
        .function(wrap_function!(tg_decrypt_ige))
}



