use aes_gcm_siv::aead::AeadMut;
use rsa::Pkcs1v15Encrypt;

use crate::utility::secure::utils::{AesGcmSivCrypto, RsaCrypto};

pub trait Crypto {
    fn setup(&mut self) -> std::io::Result<()>;
    fn encrypt(&mut self) -> std::io::Result<()>;
    fn decrypt(&mut self) -> std::io::Result<()>;
}

impl Crypto for RsaCrypto {
    fn setup(&mut self) -> std::io::Result<()> {
        self.set_keys(2048)?;
        Ok(())
    }

    fn encrypt(&mut self) -> std::io::Result<()> {
        self.set_ciphertext(
            self.take_public_key()
                .encrypt(
                    &mut self.get_rng(),
                    Pkcs1v15Encrypt,
                    self.take_plaintext(),
                ).expect(
                "self.take_public_key()\
                        .encrypt(\
                        &mut self.get_rng(), \
                        Pkcs1v15Encrypt, \
                        self.take_plaintext()\
                        )"
            )
        );
        Ok(())
    }

    fn decrypt(&mut self) -> std::io::Result<()> {
        self.set_plaintext(
            self.take_private_key()
                .decrypt(
                    Pkcs1v15Encrypt,
                    self.take_ciphertext(),
                ).expect("
                    self.take_private_key()
                    .decrypt(
                        Pkcs1v15Encrypt,
                        self.take_ciphertext()
                    )"
            )
        );
        Ok(())
    }
}

impl Crypto for AesGcmSivCrypto {
    fn setup(&mut self) -> std::io::Result<()> {
        self.set_key();
        self.set_nonce()?;
        self.set_cipher();
        Ok(())
    }

    fn encrypt(&mut self) -> std::io::Result<()> {
        self.set_ciphertext(
            self.get_cipher()
                .encrypt(
                    self.take_nonce(),
                    self.take_plaintext().as_ref(),
                ).expect(
                "self.take_cipher()
                        .encrypt(
                           self.take_nonce(),
                           self.take_plaintext().as_ref()
                        )"
            )
        );
        Ok(())
    }
    fn decrypt(&mut self) -> std::io::Result<()> {
        self.set_plaintext(
            self.get_cipher()
                .decrypt(
                    self.take_nonce(),
                    self.take_ciphertext().as_ref(),
                ).expect(
                "self.take_cipher()
                        .decrypt(
                           self.take_nonce(),
                           self.take_ciphertext().as_ref()
                        )"
            )
        );
        Ok(())
    }
}