use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::process::Command;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/shutdown")]
async fn shutdown() -> impl Responder {
    Command::new("cmd")
        .args(&["/C", "shutdown -s"])
        .output()
        .expect("failed to shutdown");

    HttpResponse::Ok().body("Bye!")
}

pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }

    pub fn run(&self) -> io::Result<()> {
        let socket_addrs = vec![
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), self.port),
            SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), self.port),
        ];

        HttpServer::new(move || App::new().service(shutdown))
            .bind(&*socket_addrs)?
            .run();

        Ok(())
    }
}
