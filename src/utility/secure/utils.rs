use aes_gcm_siv::{aead::{KeyInit, OsRng}, Aes256GcmSiv, Key, Nonce};
use rand::rngs::ThreadRng;
use rsa::{RsaPrivateKey, RsaPublicKey};

pub struct RsaCrypto {
    public_key: RsaPublicKey,
    private_key: RsaPrivateKey,
    plaintext: Vec<u8>,
    ciphertext: Vec<u8>,
    rng: ThreadRng,
}

pub struct AesGcmSivCrypto {
    plaintext: Vec<u8>,
    ciphertext: Vec<u8>,
    key: Key<Aes256GcmSiv>,
    cipher: Aes256GcmSiv,
    nonce: Nonce,
}

pub trait AesCrypto {
    fn set_plaintext(&mut self, source: Vec<u8>);
    fn set_ciphertext(&mut self, source: Vec<u8>);
    fn set_key(&mut self);
    fn set_cipher(&mut self);
    fn set_nonce(&mut self) -> Result<(), getrandom::Error>;
    fn take_key(&self) -> &Key<Aes256GcmSiv>;
    fn take_cipher(&self) -> &Aes256GcmSiv;
    fn take_nonce(&self) -> &Nonce;
    fn take_plaintext(&self) -> &Vec<u8>;
    fn take_ciphertext(&self) -> &Vec<u8>;
    fn get_cipher(&self) -> Aes256GcmSiv;
}

impl AesGcmSivCrypto {
    pub fn new() -> AesGcmSivCrypto {
        AesGcmSivCrypto {
            plaintext: vec![],
            ciphertext: vec![],
            key: Default::default(),
            cipher: Default::default(),
            nonce: Default::default(),
        }
    }
    pub fn default() -> AesGcmSivCrypto {
        let mut crypto = AesGcmSivCrypto {
            plaintext: vec![],
            ciphertext: vec![],
            key: Default::default(),
            cipher: Default::default(),
            nonce: Default::default(),
        };
        crypto.set_key();
        crypto.set_cipher();
        crypto.set_nonce().expect("secure.set_nonce()");
        crypto
    }
    pub fn set_plaintext(&mut self, source: Vec<u8>) {
        self.plaintext = source
    }
    pub fn set_ciphertext(&mut self, source: Vec<u8>) {
        self.ciphertext = source
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
    pub fn take_plaintext(&self) -> &Vec<u8> {
        &self.plaintext
    }
    pub fn take_ciphertext(&self) -> &Vec<u8> {
        &self.ciphertext
    }
    pub fn get_cipher(&self) -> Aes256GcmSiv {
        self.cipher.clone()
    }
}

impl RsaCrypto {
    pub fn new() -> RsaCrypto {
        RsaCrypto {
            public_key: Default::default(),
            private_key: Default::default(),
            plaintext: vec![],
            ciphertext: vec![],
            rng: Default::default(),
        }
    }
    pub fn default() -> AesGcmSivCrypto {
        let mut crypto = RsaCrypto {
            public_key: Default::default(),
            private_key: Default::default(),
            plaintext: vec![],
            ciphertext: vec![],
            rng: Default::default(),
        };
        crypto.set_keys(2048).expect("secure.set_keys(2048)");
        crypto.set_cipher();
        crypto
    }
    pub fn set_plaintext(&mut self, source: Vec<u8>) {
        self.plaintext = source
    }
    pub fn set_ciphertext(&mut self, source: Vec<u8>) {
        self.ciphertext = source
    }
    pub fn set_keys(&mut self, bit: usize) -> std::io::Result<()> {
        self.rng = rand::thread_rng();
        self.private_key = RsaPrivateKey::new(&mut self.rng, bit).
            expect("RsaPrivateKey::new(&mut rng, bit)");
        self.set_public_key(RsaPublicKey::from(&self.private_key));
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
    pub fn take_plaintext(&self) -> &Vec<u8> {
        &self.plaintext
    }
    pub fn take_ciphertext(&self) -> &Vec<u8> {
        &self.ciphertext
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