use std::io;
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

const SOCKET_ADDRESS: &str = "0.0.0.0:6916";

pub fn run() -> io::Result<()> {
    HttpServer::new(move || App::new().service(shutdown))
        .bind(SOCKET_ADDRESS)?
        .run();

    Ok(())
}
