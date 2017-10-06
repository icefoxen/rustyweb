extern crate pencil;
#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;

extern crate rustyweb;
use rustyweb::*;


use pencil::{Pencil, Request, Response, PencilResult};
use std::sync::RwLock;


lazy_static! {
    static ref SERVER_STATE: RwLock<ServerData> = {
        let mut s = ServerData::default();
        s.add_user("testuser");
        RwLock::new(s)
    };
}

fn hello(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello World!"))
}


fn get_name(request: &mut Request) -> PencilResult {
    if let Some(name) = request.view_args.get("name") {
        println!("Got get to {}", &name);
        if let Some(n) = SERVER_STATE.read().unwrap().get_name(&name) {
            pencil::jsonify(n)
        } else {
            pencil::abort(404)
        }
    } else {
        pencil::abort(404)
    }
}

fn post_name(request: &mut Request) -> PencilResult {
    if let Some(name) = request.view_args.get("name").cloned() {
        println!("Got post to {}", &name);
        if let Some(ref json) = request.get_json().clone() {
            // This is awful but I can't find a way to get the JSON from a request and
            // parse it directly into an object.  Sooooo.
            let json_str = rustc_serialize::json::encode(&json).unwrap();
            if let Ok(rename_request) = rustc_serialize::json::decode::<UpdateMessage>(&json_str) {
                match SERVER_STATE.write().unwrap().apply_update_if_valid(&name, &rename_request) {
                    Ok(_) => Ok(Response::from("ok")),
                    Err(_v) => pencil::abort(403),
                }
            } else {
                pencil::abort(400)
            }
        } else {
            pencil::abort(400)
        }
    } else {
        pencil::abort(404)
    }
}

fn main() {
    let mut app = Pencil::new("/web/hello");

    app.static_folder = env!("CARGO_MANIFEST_DIR").to_owned();
    app.static_folder += "/src/";
    app.static_url_path = "/id".to_owned();
    println!("Static folder is {}", &app.static_folder);

    app.get("/", "hello", hello);
    // You CAN'T pass a closure here, which I am somewhat annoyed about.
    // It makes it so you have to either put your state data in a global
    // or shove it into `Request.extensions_data` somehow.
    // The demo uses a `before_request()` hook to do the shoving but that
    // doesn't seem to let you actually PRESERVE state either.
    // So I'm just making it a global with `lazy_static`.
    app.get("/name/<name:String>", "get_name", get_name);
    app.post("/name/<name:String>", "post_name", post_name);
    app.enable_static_file_handling();

    app.run("127.0.0.1:8888");
}

