use aes_gcm_siv::aead::AeadMut;
use rsa::Pkcs1v15Encrypt;

use crate::utility::secure::utils::{AesGcmSivCrypto, RsaCrypto};

type Result<T> = std::io::Result<T>;

pub trait Crypto {
    fn setup(&mut self) -> Result<()>;
    fn encrypt(&mut self ,plaintext: Vec<u8>) -> Vec<u8>;
    fn decrypt(&mut self, ciphertext: Vec<u8>) -> Vec<u8>;
}

impl Crypto for RsaCrypto {
    fn setup(&mut self) -> Result<()> {
        self.set_keys(2048)?;
        Ok(())
    }

    fn encrypt(&mut self, plaintext: Vec<u8>) -> Vec<u8> {
        self.take_public_key()
            .encrypt(
                &mut self.get_rng(),
                Pkcs1v15Encrypt,
                plaintext.as_slice(),
            ).expect(
            "self.take_public_key()\
                    .encrypt(\
                    &mut self.get_rng(), \
                    Pkcs1v15Encrypt, \
                    self.take_plaintext()\
                    )"
        )
    }

    fn decrypt(&mut self, ciphertext: Vec<u8>) -> Vec<u8> {
        self.take_private_key()
            .decrypt(
                Pkcs1v15Encrypt,
                ciphertext.as_slice(),
            ).expect("
                self.take_private_key()
                .decrypt(
                    Pkcs1v15Encrypt,
                    self.take_ciphertext()
                )"
        )
    }
}

impl Crypto for AesGcmSivCrypto {
    fn setup(&mut self) -> Result<()> {
        self.set_key();
        self.set_nonce()?;
        self.set_cipher();
        Ok(())
    }

    fn encrypt(&mut self, plaintext: Vec<u8>) -> Vec<u8> {
            self.get_cipher()
                .encrypt(
                    self.take_nonce(),
                    plaintext.as_slice(),
                ).expect(
                "self.take_cipher()
                        .encrypt(
                           self.take_nonce(),
                           self.take_plaintext().as_ref()
                        )"
            )
    }
    fn decrypt(&mut self, ciphertext: Vec<u8>) -> Vec<u8> {
            self.get_cipher()
                .decrypt(
                    self.take_nonce(),
                    ciphertext.as_slice()
                ).expect(
                "self.take_cipher()
                        .decrypt(
                           self.take_nonce(),
                           self.take_ciphertext().as_ref()
                        )"
            )
    }
}