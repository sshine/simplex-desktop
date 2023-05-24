use std::env::args_os;
use ws::listen;
extern crate serde;
extern crate serde_json;
pub fn main() {
    let ip = args_os().nth(1).unwrap_or("127.0.0.1".into());
    let port = args_os().nth(2).unwrap_or("5100".into());
    let addr = format!("{}:{}", ip.into_string().unwrap(), port.into_string().unwrap());
    let _ws_server = listen(addr, |out| {
        move |msg| {
            println!("Got message: {}", msg);
            out.close(ws::CloseCode::Normal)
        }
    }).unwrap();
}