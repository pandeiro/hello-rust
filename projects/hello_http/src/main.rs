extern crate hyper;

use hyper::Server;
use hyper::server::Response;
use hyper::server::Request;

fn hello (_: Request, resp: Response) {
    resp.send(b"Hello World!").unwrap();
}

fn main() {
    Server::http("127.0.0.1:3000").unwrap().handle(hello);
}
