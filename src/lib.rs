use crate::server::Server;
use crate::service_event_handler::ServiceEventHandler;

mod server;
mod service_event_handler;

pub fn run(service_name: &str) -> Result<(), anyhow::Error> {
    let sys = actix_web::rt::System::new(service_name);

    const PORT: u16 = 6916;
    Server::new(PORT).run()?;

    ServiceEventHandler::new(service_name).run()?;

    sys.run()?;

    Ok(())
}
