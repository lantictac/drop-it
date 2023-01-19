use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use system_shutdown::shutdown_with_message;

use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/shutdown")]
async fn shutdown(req: HttpRequest) -> impl Responder {
    let connection_info = req.connection_info();
    let request_addr = match connection_info.realip_remote_addr() {
        Some(addr) => addr,
        None => "<unknown>",
    };

    match shutdown_with_message(
        format!("Drop-It shutdown invoked remotely by {request_addr}").as_str(),
        10,
        false
    ) {
        Ok(_) => HttpResponse::Ok().body("Bye!"),
        Err(_error) => HttpResponse::InternalServerError().into(),
    }
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
