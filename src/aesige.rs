use aes::Aes256;
use aes::cipher::{block_padding::ZeroPadding, KeyIvInit, BlockEncryptMut, BlockDecryptMut};
use ige::{Encryptor, Decryptor};

type Enc = Encryptor<Aes256>;

type Dec = Decryptor<Aes256>;

fn zero_pad(mut data: Vec<u8>) -> Vec<u8> {
	let block = 16;
	let pad = (block - (data.len() % block)) % block;

	if pad != 0 {
		data.extend(std::iter::repeat(0u8).take(pad));
	}

	data
}

pub fn aes_ige_encrypt(
	plaintext: &[u8],
	key: &[u8; 32],
	iv: &[u8; 32],
) -> Vec<u8> {
	let padded = zero_pad(plaintext.to_vec());

	let mut out = vec![0u8; padded.len()];

	Enc::new(key.into(), iv.into()).encrypt_padded_b2b_mut::<ZeroPadding>(&padded, &mut out).unwrap();

	out
}

pub fn aes_ige_decrypt(
	ciphertext: &[u8],
	key: &[u8; 32],
	iv: &[u8; 32],
) -> Vec<u8> {
	let padded = zero_pad(ciphertext.to_vec());

	let mut out = vec![0u8; padded.len()];

	Dec::new(key.into(), iv.into()).decrypt_padded_b2b_mut::<ZeroPadding>(&padded, &mut out).unwrap();

	out
}