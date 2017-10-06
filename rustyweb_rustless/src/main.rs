extern crate rustless;
extern crate iron;

use rustless::{Application, Api, Nesting};

use std::path::PathBuf;

fn main() {

    let api = Api::build(|api| {
        api.get("", |endpoint| {
            endpoint.handle(|client, _params| client.text("Hello world!".to_owned()))
        });

        api.namespace("id/:filename", |file_ns| {
            file_ns.get("", |endpoint| {
                endpoint.handle(|client, _params| client.text("Hello file!".to_owned()))
            });

            file_ns.get("", |file_endpoint| {
                file_endpoint.handle(|mut client, params| {
                    // Params is a JSON object, which is a little weird
                    // when it basically encodes the router path parameter...
                    if let Some(jsonval) = params.find("filename") {
                        // Yes we should be able to use and_then() or such
                        // to extract this but ownership makes it tricksy.
                        if let Some(s) = jsonval.as_str() {
                            println!("Returning file {:?}", params);
                            // Can't find any SAFE way of doing this so
                            // we're just gonna suck it.
                            let mut root = PathBuf::new();
                            root.push("src");
                            root.push(s);
                            client.file(&root)
                        } else {
                            client.set_status(rustless::server::status::StatusCode::BadRequest);
                            client.text("bad file type".to_owned())
                        }
                    } else {
                        client.not_found();
                        client.text("File not found".to_owned())
                    }
                })
            })
        });
    });

    let app = Application::new(api);

    iron::Iron::new(app).http("0.0.0.0:8888").unwrap();
    println!("Server started on port 8888");
}
