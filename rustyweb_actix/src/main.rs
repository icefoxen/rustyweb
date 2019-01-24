use actix_web::{App, fs, server};

fn main() {
    server::new(|| App::new()
        .handler(
            "/static",
            fs::StaticFiles::new(".")
                .unwrap()
                .show_files_listing()))
        .bind("127.0.0.1:8888")
        .expect("Can not bind to port 8888")
        .run()
}