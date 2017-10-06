#[macro_use]
extern crate rustful;

use rustful::{Server, Context, Response, TreeRouter};

fn say_hello(context: Context, response: Response) {
    response.send(format!("Hello, world!"));
}


fn main() {
    let router = insert_routes! {
        TreeRouter::new() => {
            Get: say_hello,
        }
    };

    //Build and run the server.
    let server = Server {
        handlers: router,

        //Turn a port number into an IPV4 host address (0.0.0.0:8080 in this case).
        host: 8888.into(),

        //Use default values for everything else.
        ..Server::default()
    };
    
    server.run().expect("Could not run server?");
}
/*
#[cfg(test)]
mod tests {
    extern crate reqwest;
    use lazy_static;
    use std::thread;
    use std::io::Read;
    use serde::Serialize;
    use ring::{rand, signature};
    use untrusted;
    use base64;
    use chrono::prelude::*;
    use super::UpdateMessage;

    const UNITTEST_USER: &str = "unittest_user";
    const UNITTEST_NAME: &str = "unittest_name";

    fn start_test_server() {
        use super::ServerData;
        let mut s = ServerData::default();
        let pubkey_bytes = KEYPAIR.public_key_bytes();
        s.add_id(UNITTEST_USER, pubkey_bytes);
        s.update_name(UNITTEST_NAME, &UNITTEST_NAME_VALUE);
        super::run(s, "127.0.0.1:8888");

    }

    fn generate_keypair() -> signature::Ed25519KeyPair {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let keypair = signature::Ed25519KeyPair::from_pkcs8(
            untrusted::Input::from(&pkcs8_bytes)
        ).unwrap();
        keypair
    }

    lazy_static! {
        static ref SERVER_THREAD: thread::JoinHandle<()> = thread::spawn(start_test_server);
        static ref KEYPAIR: signature::Ed25519KeyPair = generate_keypair();
        static ref UNITTEST_NAME_VALUE: UpdateMessage = UpdateMessage::signed_message(&KEYPAIR, UNITTEST_USER, "unittest_value");
    }


    fn spawn_server_and_get(path: &str) -> reqwest::Response {
        lazy_static::initialize(&SERVER_THREAD);
        let new_path = String::from("http://localhost:8888") + path;
        reqwest::get(&new_path).unwrap()
    }

    fn spawn_server_and_post<T: Serialize>(path: &str, json: &T) -> reqwest::Response {
        lazy_static::initialize(&SERVER_THREAD);
        let client = reqwest::Client::new().unwrap();
        let new_path = String::from("http://localhost:8888") + path;
        client.post(&new_path).unwrap()
            .json(json).unwrap()
            .send().unwrap()
    }

    #[test]
    fn test_basic() {
        let mut resp = spawn_server_and_get("/");
        assert!(resp.status().is_success());
        let mut content = String::new();
        resp.read_to_string(&mut content).unwrap();
        assert_eq!(content, "hello world");
    }

    #[test]
    fn test_id() {
        let mut resp = spawn_server_and_get((String::from("/id/") + UNITTEST_USER).as_str());
        assert!(resp.status().is_success());
        let mut content = String::new();
        resp.read_to_string(&mut content).unwrap();
        let pubkey_bytes = KEYPAIR.public_key_bytes();
        let pubkey_string = base64::encode(pubkey_bytes);
        assert_eq!(content, pubkey_string);
    }

    #[test]
    fn test_get_name() {
        // Test unset name default
        let resp = spawn_server_and_get("/name/test_no_name");
        assert_eq!(resp.status(), reqwest::StatusCode::NotFound);

        // Test set name
        let mut resp = spawn_server_and_get((String::from("/name/") + UNITTEST_NAME).as_str());
        assert!(resp.status().is_success());
        let resp_msg: UpdateMessage = resp.json().unwrap();
        assert_eq!(resp_msg, *UNITTEST_NAME_VALUE);
        // let mut content = String::new();
        // resp.read_to_string(&mut content).unwrap();
        // assert_eq!(content, UNITTEST_NAME_VALUE.new_contents);
    }

    #[test]
    fn test_post_name() {
        const NEWNAME: &str = "/name/test_post_name";
        // See that name DNE
        let resp = spawn_server_and_get(NEWNAME);
        assert!(!resp.status().is_success());

        let changed_name = "foo!";
        let data = super::UpdateMessage::signed_message(&KEYPAIR, UNITTEST_USER, changed_name);

        // Change name
        let mut resp = spawn_server_and_post(NEWNAME, &data);
        assert!(resp.status().is_success());
        let mut content = String::new();
        resp.read_to_string(&mut content).unwrap();
        assert_eq!(content, "ok");

        // Test name now that it's been changed
        let mut resp = spawn_server_and_get(NEWNAME);
        assert!(resp.status().is_success());
        let resp_msg: UpdateMessage = resp.json().unwrap();
        assert_eq!(resp_msg, data);
        // let mut content = String::new();
        // resp.read_to_string(&mut content).unwrap();
        // assert_eq!(content, changed_name);

        // Try changing it again with unsigned request
        let baddata = super::UpdateMessage {
            user: UNITTEST_USER.into(),
            utc: Utc::now(),
            signature: "".into(),
            new_contents: "aieeee!".into(),
        };
        let resp = spawn_server_and_post(NEWNAME, &baddata);
        assert!(!resp.status().is_success());

        // Ensure it hasn't changed.
        let mut resp = spawn_server_and_get(NEWNAME);
        assert!(resp.status().is_success());
        let resp_msg: UpdateMessage = resp.json().unwrap();
        assert_eq!(resp_msg, data);

        // let mut content = String::new();
        // resp.read_to_string(&mut content).unwrap();
        // assert_eq!(content, changed_name);

    }
}
*/