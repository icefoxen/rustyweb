extern crate rustless;
extern crate iron;

use rustless::{Application, Api, Nesting};

fn main() {

    let api = Api::build(|api| {
        api.get("", |endpoint| {
            endpoint.handle(|client, _params| client.text("Hello world!".to_owned()))
        })

        api.namespace("id/:foo", |file_ns| {
            file_ns.get(

            )
        }
        }
    });

    let app = Application::new(api);

    iron::Iron::new(app).http("0.0.0.0:8888").unwrap();
    println!("Server started on port 8888");
}
