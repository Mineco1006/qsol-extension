use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce, KeyInit, aead::Aead};
use qubic_types::{QubicWallet, QubicId};
use sha3::{Sha3_512, Digest, Sha3_256};
use serde::{Serialize, Deserialize};
use wbg_rand::Rng;


pub fn hash_password(pwd: &str, salt: [u8; 4]) -> String {
    let mut pwd = pwd.as_bytes().to_vec();
    pwd.extend(salt);

    let mut hash = Sha3_512::digest(&pwd);
    for _ in 0..2048 {
        hash = Sha3_512::digest(&hash);
    }

    hex::encode(&hash.to_vec())
}

pub struct EncryptionResult {
    pub encrypted_seed: String,
    pub nonce: String
}

pub fn encrypt(seed: &str, pwd: &str) -> EncryptionResult {
    let key = Sha3_256::digest(pwd.as_bytes());
    let mut rng = wbg_rand::wasm_rng();

    let key = Key::<Aes256GcmSiv>::from_slice(&key);
    let cipher = Aes256GcmSiv::new(&key);
    let nonce: [u8; 12] = rng.gen();
    let nonce_hex = hex::encode(&nonce);
    let nonce = Nonce::from_slice(&nonce);

    let ciphertext = cipher.encrypt(nonce, seed.as_bytes()).unwrap();

    let ct_hex = hex::encode(&ciphertext);
    
    EncryptionResult { encrypted_seed: ct_hex, nonce: nonce_hex }
}

pub fn decrypt(encrypted_seed: &str, pwd: &str, nonce: &str) -> String {
    let key = Sha3_256::digest(pwd.as_bytes());
    let key = Key::<Aes256GcmSiv>::from_slice(&key);
    let cipher = Aes256GcmSiv::new(&key);
    let nonce = hex::decode(nonce).unwrap();

    let nonce = Nonce::from_slice(&nonce);

    String::from_utf8(cipher.decrypt(nonce, hex::decode(encrypted_seed).unwrap().as_slice()).unwrap()).unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeystoreFile {
    pub pwd: String,
    pub salt: [u8; 4],
    pub wallets: Vec<Keystore>
}

impl KeystoreFile {
    pub fn add_wallet(&mut self, name: String, pwd: &str, seed: &str) -> Result<(), String> {
        let rhs = hash_password(pwd, self.salt);

        if self.pwd != rhs {
            return Err("Incorrect Password!".to_string())
        }
        let wallet = match QubicWallet::from_seed(seed) {
            Ok(w) => w,
            Err(e) => {
                return Err(e.to_string())
            }
        };
        let id = wallet.public_key;

        let res = encrypt(seed, pwd);

        let ks = Keystore { name, id, seed: res.encrypted_seed, c_nonce: res.nonce };

        self.wallets.push(ks);

        Ok(())
    }

    pub fn get_wallet(&self, account_index: usize, pwd: &str) -> Result<QubicWallet, String> {
        let rhs = hash_password(pwd, self.salt);

        if self.pwd != rhs {
            return Err("Incorrect Password!".to_string())
        }

        if let Some(ks) = self.wallets.get(account_index) {
            let seed = decrypt(&ks.seed, pwd, &ks.c_nonce);
            let wallet = match QubicWallet::from_seed(&seed) {
                Ok(w) => w,
                Err(e) => {
                    return Err(e.to_string())
                }
            };
            drop(seed);

            Ok(wallet)
        } else {
            Err(format!("Account with index {account_index} does not exist!"))
        }
    }

    pub fn get_seed(&self, account_index: usize, pwd: &str) -> Result<String, String> {
        let rhs = hash_password(pwd, self.salt);

        if self.pwd != rhs {
            return Err("Incorrect Password!".to_string())
        }

        if let Some(ks) = self.wallets.get(account_index) {
            let seed = decrypt(&ks.seed, pwd, &ks.c_nonce);

            Ok(seed)
        } else {
            Err(format!("Account with index {account_index} does not exist!"))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keystore {
    pub name: String,
    pub id: QubicId,

    // aes-gcm-siv 256 encryption
    pub seed: String,
    pub c_nonce: String
}