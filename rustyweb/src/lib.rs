extern crate ring;
extern crate untrusted;
extern crate chrono;
extern crate base64;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use ring::{signature, rand};
use chrono::prelude::*;

use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ValidationError {
    UnknownUser(String),
    MalformedSignature,
    InvalidSignature,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateMessage {
    pub user: String,
    pub utc: DateTime<Utc>,
    pub signature: String,
    pub new_contents: String,
}
impl UpdateMessage {
    pub fn signed_message(keypair: &signature::Ed25519KeyPair, user: &str, msg: &str) -> UpdateMessage {
        let aggregated_message = String::from(user) + " " + msg;
        let message_bytes = aggregated_message.as_bytes();
        let sig = keypair.sign(message_bytes);
        let base64_sig = base64::encode(sig.as_ref());
        UpdateMessage {
            user: user.to_string(),
            utc: Utc::now(),
            signature: base64_sig,
            new_contents: msg.to_string(),
        }
    }

    pub fn verify_signature(&self, pubkey_bytes: &[u8]) -> Result<(), ValidationError> {
        let aggregated_message = String::from(self.user.as_str()) + " " + self.new_contents.as_ref();
        let message_bytes = aggregated_message.as_bytes();
        let sig_bytes = base64::decode(&self.signature)
            .map_err(|_decode_error| ValidationError::MalformedSignature)?;
        let pubkey = untrusted::Input::from(pubkey_bytes);
        let msg = untrusted::Input::from(message_bytes);
        let sig = untrusted::Input::from(&sig_bytes);
        signature::verify(&signature::ED25519, pubkey, msg, sig)
            .map_err(|_err| ValidationError::InvalidSignature)
    }
}


#[derive(Debug, Default, Clone)]
pub struct ServerData {
    names: HashMap<String, UpdateMessage>,
    keys: HashMap<String, Vec<u8>>,
}

impl ServerData {
    pub fn get_name(&self, name: &str) -> Option<&UpdateMessage> {
        self.names.get(name)
    }

    pub fn get_id_key(&self, id: &str) -> Option<&[u8]> {
        self.keys.get(id).map(|x| x.as_ref())
    }

    pub fn add_id(&mut self, id: &str, key: &[u8]) {
        self.keys.insert(id.into(), key.into());
    }

    pub fn validate_update(&self, msg: &UpdateMessage) -> Result<(), ValidationError> {
        match self.keys.get(&msg.user) {
            Some(key) => msg.verify_signature(key),
            None => Err(ValidationError::UnknownUser(msg.user.clone()))
        }
    }

    pub fn update_name(&mut self, name: &str, contents: &UpdateMessage) {
        self.names.insert(name.to_string(), contents.clone());
    }

    pub fn apply_update_if_valid(&mut self, dest: &str, msg: &UpdateMessage) -> Result<(), ValidationError> {
        let _ = self.validate_update(msg)?;
        self.update_name(dest, &msg);
        Ok(())
    }

    pub fn add_user(&mut self, username: &str) {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let keypair = signature::Ed25519KeyPair::from_pkcs8(
            untrusted::Input::from(pkcs8_bytes.as_ref())
        ).unwrap();

        let encoded_privkey = base64::encode(pkcs8_bytes.as_ref());
        println!("Private key for {} is: {}", username, encoded_privkey);

        use ring::signature::KeyPair;
        let pubkey_bytes = keypair.public_key().as_ref();
        self.add_id(username, pubkey_bytes);

    }
}