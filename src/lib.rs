use crate::server::Server;
use crate::service_event_handler::ServiceEventHandler;

mod server;
mod service_event_handler;

pub fn run(service_name: &str, port: u16) -> Result<(), anyhow::Error> {
    let sys = actix_web::rt::System::new(service_name);

    Server::new(port).run()?;
    ServiceEventHandler::new(service_name).run()?;

    sys.run()?;

    Ok(())
}
