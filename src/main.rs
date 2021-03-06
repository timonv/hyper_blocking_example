#![feature(old_io)]

extern crate hyper;

use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::old_io::net::ip::Ipv4Addr;
use hyper::server::{Server, Request, Response};

fn main() {
    let message = get_message();
    println!("Never called: {}", message)
}

fn get_message() -> String {
    let server = Server::http(Ipv4Addr(127, 0, 0, 1), 9999);

    let (tx, rx) = channel();
    let mtx = Mutex::new(tx);

    let mut guard = server.listen_threads(move |_: Request, res: Response| {
        println!("Request received!");
        mtx.lock().unwrap().send("Boosh!").unwrap();
        println!("Request ended!");

        let mut res = res.start().unwrap();
        res.end().unwrap();
    }, 1).unwrap();

    let message = rx.recv().unwrap();
    guard.close().unwrap();
    println!("{}", message);
    message.to_string()
}
