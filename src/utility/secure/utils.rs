use aes_gcm_siv::{aead::{KeyInit, OsRng}, Aes256GcmSiv, Key, Nonce};
use rand::rngs::ThreadRng;
use rsa::{RsaPrivateKey, RsaPublicKey};

pub struct RsaCrypto {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
    rng: ThreadRng,
}

pub struct AesGcmSivCrypto {
    key: Key<Aes256GcmSiv>,
    cipher: Aes256GcmSiv,
    nonce: Nonce,
}

pub trait AesCrypto {
    fn set_key(&mut self);
    fn set_cipher(&mut self);
    fn set_nonce(&mut self) -> Result<(), getrandom::Error>;
    fn take_key(&self) -> &Key<Aes256GcmSiv>;
    fn take_cipher(&self) -> &Aes256GcmSiv;
    fn take_nonce(&self) -> &Nonce;
    fn get_cipher(&self) -> Aes256GcmSiv;
}

impl AesGcmSivCrypto {
    pub fn default() -> AesGcmSivCrypto {
        let key = Aes256GcmSiv::generate_key(&mut OsRng);
        let mut buf = [0u8; 12];
        getrandom::getrandom(&mut buf).expect("getrandom::getrandom(&mut buf)");
        let mut crypto = AesGcmSivCrypto {
            key,
            cipher: Aes256GcmSiv::new(&key),
            nonce: *Nonce::from_slice(&buf[..]),
        };
        crypto.set_key();
        crypto.set_cipher();
        crypto.set_nonce().expect("secure.set_nonce()");
        crypto
    }
    pub fn set_key(&mut self) {
        self.key = Aes256GcmSiv::generate_key(&mut OsRng);
    }
    pub fn set_cipher(&mut self) {
        self.cipher = Aes256GcmSiv::new(&self.key);
    }
    pub fn set_nonce(&mut self) -> Result<(), getrandom::Error> {
        let mut buf = [0u8; 12];
        getrandom::getrandom(&mut buf)?;
        self.nonce = *Nonce::from_slice(&buf[..]);
        Ok(())
    }
    pub fn take_key(&self) -> &Key<Aes256GcmSiv> {
        &self.key
    }
    pub fn take_cipher(&self) -> &Aes256GcmSiv {
        &self.cipher
    }
    pub fn take_nonce(&self) -> &Nonce {
        &self.nonce
    }
    pub fn get_cipher(&self) -> Aes256GcmSiv {
        self.cipher.clone()
    }
}

impl RsaCrypto {
    pub fn default() -> RsaCrypto {
        let mut rng = rand::thread_rng();
        let pvt_key = RsaPrivateKey::new(&mut rng, 2048).expect("RsaPrivateKey::new(&mut rng, bit)");
        RsaCrypto {
            public_key: RsaPublicKey::from(&pvt_key),
            private_key: pvt_key,
            rng,
        }
    }
    pub fn set_keys(&mut self, bit: usize) -> std::io::Result<()> {
        self.rng = rand::thread_rng();
        self.private_key = RsaPrivateKey::new(&mut self.rng, bit).expect("RsaPrivateKey::new(&mut rng, bit)");
        self.public_key = RsaPublicKey::from(&self.private_key);
        Ok(())
    }
    pub fn set_public_key(&mut self, public_key: RsaPublicKey) {
        self.public_key = public_key
    }
    pub fn take_public_key(&self) -> &RsaPublicKey {
        &self.public_key
    }
    pub fn take_private_key(&self) -> &RsaPrivateKey {
        &self.private_key
    }
    pub fn get_rng(&self) -> ThreadRng {
        self.rng.clone()
    }
    pub fn get_public_key(&self) -> RsaPublicKey {
        self.public_key.clone()
    }
    pub fn get_private_key(&self) -> RsaPrivateKey {
        self.private_key.clone()
    }
}