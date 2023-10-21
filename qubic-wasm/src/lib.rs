mod utils;
mod crypto;

use crypto::{hash_password, KeystoreFile, encrypt, decrypt};
use kangarootwelve::KangarooTwelve;
use qubic_rpc_types::{JsonRpcRequest, TransactionParams, JsonRpcResponse};
use qubic_tcp_types::types::{Transaction, RawTransaction};
use qubic_types::{QubicId, traits::AsByteEncoded, QubicWallet, Signature};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wbg_rand::Rng;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn check_password(keystore: JsValue, rhs: &str) -> Result<bool, String> {
    let keystore: KeystoreFile = match serde_wasm_bindgen::from_value(keystore) {
        Ok(ks) => ks,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let rhs = hash_password(rhs, keystore.salt);
    
    Ok(keystore.pwd == rhs)
}

#[wasm_bindgen]
pub fn set_password(pwd: &str) -> JsValue {
    let mut rng = wbg_rand::wasm_rng();
    let salt: [u8; 4] = rng.gen();;
    let pwd_hash = hash_password(pwd, salt);

    let keystore = KeystoreFile { pwd: pwd_hash, salt, wallets: Vec::new() };

    serde_wasm_bindgen::to_value(&keystore).unwrap()
}

#[wasm_bindgen]
pub fn add_wallet(keystore: JsValue, pwd: &str, name: &str, seed: &str) -> Result<JsValue, String> {
    let mut keystore: KeystoreFile = match serde_wasm_bindgen::from_value(keystore) {
        Ok(ks) => ks,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    keystore.add_wallet(name.to_string(), pwd, seed)?;

    Ok(serde_wasm_bindgen::to_value(&keystore).unwrap())
}

#[wasm_bindgen]
pub async fn get_entities(keystore: JsValue, jsonrpc: &str) -> Result<JsValue, String> {
    let keystore: KeystoreFile = match serde_wasm_bindgen::from_value(keystore) {
        Ok(ks) => ks,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let client = reqwest::Client::new();

    let mut entities = Vec::with_capacity(keystore.wallets.len());

    for wallet in &keystore.wallets {
        let req = JsonRpcRequest::RequestEntity { jsonrpc: "2.0".to_string(), id: 0, params: wallet.id.clone() };

        match client.post(jsonrpc).json(&req).send().await {
            Ok(r) => {
                match r.json().await {
                    Ok(r) => {
                        if let JsonRpcResponse::RequestEntity { jsonrpc: _, id: _, result, error: _ } = r {
                            if let Some(r) = result {
                                entities.push(r);
                            }
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }

    Ok(serde_wasm_bindgen::to_value(&entities).unwrap())
}

#[wasm_bindgen]
pub async fn get_current_tick(jsonrpc: &str) -> Result<u32, String> {
    let client = reqwest::Client::new();

    let req = JsonRpcRequest::RequestCurrentTickInfo { jsonrpc: "2.0".to_string(), id: 0 };

    match client.post(jsonrpc).json(&req).send().await {
        Ok(res) => {
            let res: JsonRpcResponse = res.json().await.unwrap();

            if let JsonRpcResponse::RequestCurrentTickInfo { jsonrpc: _, id: _, result, error } = res {
                if let Some(e) = error {
                    Err(e)
                } else if let Some(r) = result {
                    Ok(r.tick)
                } else {
                    Err("Invalid response!".to_string())
                }
            } else {
                Err("Invalid response".to_string())
            }
        },
        Err(e) => {
            Err(e.to_string())
        }
    }
}

#[wasm_bindgen]
pub async fn send_transaction(keystore: JsValue, pwd: &str, account_index: usize, jsonrpc: &str, to: &str, amount: u64, tick: u32, input_type: u16, input_size: u16) -> Result<(), String> {
    let keystore: KeystoreFile = match serde_wasm_bindgen::from_value(keystore) {
        Ok(ks) => ks,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let client =  reqwest::Client::new();//Client::<Tcp>::new(jsonrpc);

    let to = match QubicId::from_str(to) {
        Ok(id) => id,
        Err(e) => {
            return Err(e.to_string())
        }
    };

    let wallet = keystore.get_wallet(account_index, pwd)?;
    

    let transaction = RawTransaction {
        from: wallet.public_key,
        to,
        amount,
        tick,
        input_type,
        input_size
    };

    let signature = wallet.sign(&transaction);

    let params = TransactionParams {
        from: wallet.public_key,
        to,
        amount,
        tick,
        signature
    };

    let req = JsonRpcRequest::SendTransaction { jsonrpc: "2.0".to_string(), id: 0, params };

    match client.post(jsonrpc).json(&req).send().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

#[wasm_bindgen]
pub fn get_id(seed: &str) -> Result<String, String> {
    match QubicWallet::from_seed(seed) {
        Ok(w) => Ok(w.get_identity()),
        Err(e) => Err(e.to_string())
    }
}

#[wasm_bindgen]
pub fn get_random_seed() -> String {
    let mut seed: [u8; 55] = [0; 55];

    let mut rng = wbg_rand::wasm_rng();

    for i in 0..55 {
        seed[i] = b'a' + (rng.gen::<u8>() % 26);
    }

    String::from_utf8(seed.to_vec()).unwrap()
}

#[wasm_bindgen]
pub fn retrieve_seed(keystore: JsValue, pwd: &str, account_index: usize) -> Result<String, String> {
    let keystore: KeystoreFile = match serde_wasm_bindgen::from_value(keystore) {
        Ok(ks) => ks,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    Ok(keystore.get_seed(account_index, pwd)?)
}

#[derive(Debug, Clone, Default, Serialize)]
struct Authentication {
    pub service_name: String,
    pub timestamp: u32,
    pub id: QubicId,
    pub signature: Signature
}

#[wasm_bindgen]
pub fn sign_authentication_request(keystore: JsValue, pwd: &str, service_name: &str, account_index: usize) -> Result<JsValue, String> {
    let keystore: KeystoreFile = match serde_wasm_bindgen::from_value(keystore) {
        Ok(ks) => ks,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    let mut auth = Authentication::default();
    let timestamp = wasm_timer::SystemTime::now().duration_since(wasm_timer::SystemTime::UNIX_EPOCH).unwrap().as_secs() as u32;

    let mut auth_vec: Vec<u8> = Vec::new();
    auth_vec.extend(&timestamp.to_le_bytes());
    auth.timestamp = timestamp;
    auth_vec.extend(service_name.as_bytes());
    auth.service_name = service_name.to_string();
    let wallet = keystore.get_wallet(account_index, pwd)?;
    auth_vec.extend(&wallet.public_key.0);
    auth.id = wallet.public_key;

    let mut digest = [0; 32];
    let mut kg = KangarooTwelve::hash(&auth_vec, &[]);
    kg.squeeze(&mut digest);

    auth.signature = wallet.sign_raw(digest);

    Ok(serde_wasm_bindgen::to_value(&auth).unwrap())
}

#[wasm_bindgen]
pub struct SessionPwd {
    pwd: String,
    key: String,
    n: String
}

#[wasm_bindgen]
pub fn store_session_pwd(pwd: &str) -> SessionPwd {
    let mut key = [0; 64];
    let mut rng = wbg_rand::wasm_rng();

    for i in 0..64 {
        key[i] = rng.gen::<u8>();
    }

    let key = String::from_utf8(key.to_vec()).unwrap();
    let res = encrypt(pwd, &key);

    SessionPwd { pwd: res.encrypted_seed, key, n: res.nonce }
}

#[wasm_bindgen]
pub fn get_pwd_from_session(spwd: SessionPwd) -> String {
    let pwd = decrypt(&spwd.pwd, &spwd.key, &spwd.n);

    pwd
}