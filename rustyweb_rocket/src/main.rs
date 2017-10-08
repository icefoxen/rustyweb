#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("hello world")
}

fn main() {
    let config = rocket::config::Config::build(rocket::config::Environment::Staging)
        .port(8888)
        .finalize().expect("Could not create config");

    rocket::custom(config, false)
        .mount("/", routes![hello])
        .launch();
}



#[cfg(test)]
mod tests {
    extern crate reqwest;
    use std::thread;
    use std::io::Read;
    use super::main;

    #[test]
    fn test_basic() {
        let _t = thread::spawn(main);
        let mut resp = reqwest::get("http://localhost:8888").unwrap();
        assert!(resp.status().is_success());
        let mut content = String::new();
        resp.read_to_string(&mut content).unwrap();
        assert_eq!(content, "hello world");
    }
}