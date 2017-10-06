#[macro_use] 
extern crate nickel;
#[macro_use]
extern crate lazy_static;

use nickel::{Nickel, Mountable, StaticFilesHandler, MiddlewareResult, Request, Response};



fn hello_world<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.send("Hello world")
}

fn main() {
    let mut server = Nickel::new();
    // Apparently this falls-through if we don't math the
    // first mount.
    server.mount("/id/", StaticFilesHandler::new("src/"));
    server.mount("/id/", middleware! { |req|
        let path = req.path_without_query().unwrap();
        format!("No static file with path '{}'!", path)
    });

    // The ordering is apparently important here; if I put this first then
    // it seems to match everything.
    server.mount("/",  hello_world);
    
    server.listen("127.0.0.1:8888").unwrap();
}


#[cfg(test)]             
mod tests {             
    extern crate reqwest;
    use lazy_static;                           
    use std::thread;
    use std::io::Read;                                                                
                                                      
    lazy_static! {
        static ref SERVER_THREAD: thread::JoinHandle<()> = thread::spawn(super::main);
    }                                                         
                                                
    fn spawn_server_and_get(path: &str) -> reqwest::Response {            
        lazy_static::initialize(&SERVER_THREAD);
        let new_path = String::from("http://localhost:8888") + path;                                                            
        reqwest::get(&new_path).unwrap()
    }                
                                                 
    #[test]                                 
    fn test_basic() {                   
        let mut resp = spawn_server_and_get("/");  
        assert!(resp.status().is_success());
        let mut content = String::new();
        resp.read_to_string(&mut content).unwrap();
        assert_eq!(content, "Hello world");               
    }               
                                                        
    #[test]                                 
    fn test_file() {                                                 
        let mut resp = spawn_server_and_get("/id/main.rs"); 
        assert!(resp.status().is_success());    
        let mut content = String::new();                                        
        resp.read_to_string(&mut content).unwrap();
        assert_eq!(content, include_str!("main.rs"));
    }
}