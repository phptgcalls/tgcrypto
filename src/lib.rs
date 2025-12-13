#![cfg_attr(windows, feature(abi_vectorcall))]

mod factorizator;
mod aesige;

use ext_php_rs::prelude::*;
use ext_php_rs::binary::Binary;
use ext_php_rs::types::{ZendStr, Zval};

#[php_const]
pub const TGCRYPTO_VERSION: &str = "0.0.1";

#[php_function]
pub fn tg_factorize(pq: u64) -> Vec<u64> {
	factorizator::factor(pq)
}

pub fn zval_from_zend_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Zval, ext_php_rs::error::Error> {
	let zstr = ZendStr::new(bytes.as_ref(), false);
	let mut zv = Zval::new();
	zv.set_zend_string(zstr);
	Ok(zv)
}

#[php_function]
pub fn tg_encrypt_ige(plain: Binary<u8>, key: Binary<u8>, iv: Binary<u8>) -> Result<Zval, String> {
	let plain_bytes = plain.as_ref();

	let key_bytes = key.as_ref();
	let iv_bytes = iv.as_ref();

	let mut key_array = [0u8; 32];
	key_array.copy_from_slice(key_bytes);

	let mut iv_array = [0u8; 32];
	iv_array.copy_from_slice(iv_bytes);

	let cipher: Vec<u8> = aesige::aes_ige_encrypt(plain_bytes, &key_array, &iv_array);

	zval_from_zend_bytes(&cipher).map_err(|e| format!("zend allocation error : {}", e))
}

#[php_function]
pub fn tg_decrypt_ige(cipher: Binary<u8>, key: Binary<u8>, iv: Binary<u8>) -> Result<Zval, String> {
	let cipher_bytes = cipher.as_ref();

	let key_bytes = key.as_ref();
	let iv_bytes = iv.as_ref();

	let mut key_array = [0u8; 32];
	key_array.copy_from_slice(key_bytes);
	
	let mut iv_array = [0u8; 32];
	iv_array.copy_from_slice(iv_bytes);

	let plain: Vec<u8> = aesige::aes_ige_decrypt(cipher_bytes, &key_array, &iv_array);

	zval_from_zend_bytes(&plain).map_err(|e| format!("zend allocation error : {}", e))
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
	module
		.constant(wrap_constant!(TGCRYPTO_VERSION))
		.function(wrap_function!(tg_factorize))
		.function(wrap_function!(tg_encrypt_ige))
		.function(wrap_function!(tg_decrypt_ige))
}