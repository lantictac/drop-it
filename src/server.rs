use actix_web::web::Query;
use serde::Deserialize;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use system_shutdown::shutdown_with_message;

use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[derive(Deserialize)]
struct ShutdownParams {
    message: Option<String>,
    timeout_s: Option<u32>,
    force_close_apps: Option<bool>,
}

#[get("/shutdown")]
async fn shutdown(req: HttpRequest, params: Query<ShutdownParams>) -> impl Responder {
    let connection_info = req.connection_info();
    let request_addr = match connection_info.realip_remote_addr() {
        Some(addr) => addr,
        None => "<unknown>",
    };

    let user_message = match params.message.as_ref() {
        Some(message) => message,
        None => "No reason"
    };

    let message = format!("Drop-It shutdown requested by {request_addr}: {user_message}");

    match shutdown_with_message(
        &message,
        params.timeout_s.unwrap_or(10),
        params.force_close_apps.unwrap_or(false),
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
