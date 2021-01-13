use actix_web::{get, App, HttpServer, Result};
use std::{ffi::OsString, process::Command, time::Duration};
use windows_service::{define_windows_service, service_dispatcher};
use windows_service::{
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
};

const SERVICE_NAME: &str = "DropIt";
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;
const SOCKET_ADDRESS: &str = "0.0.0.0:6916";

#[get("/shutdown")]
async fn shutdown() -> Result<String> {
    Command::new("cmd")
        .args(&["/C", "shutdown -s"])
        .output()
        .expect("failed to shutdown");

    Ok(String::from("Bye!"))
}

define_windows_service!(ffi_service_main, my_service_main);

fn my_service_main(arguments: Vec<OsString>) {
    if let Err(_e) = run_service(arguments) {
        // Handle errors in some way.
    }
}

fn main() -> Result<(), windows_service::Error> {
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
    Ok(())
}

pub fn run_service(_arguments: Vec<OsString>) -> Result<(), windows_service::Error> {
    let sys = actix_web::rt::System::new(SERVICE_NAME);

    HttpServer::new(move || App::new().service(shutdown))
        .bind(SOCKET_ADDRESS)
        .unwrap()
        .run();

    let (mut send_stop, recv_stop) = {
        let (p, c) = futures::channel::oneshot::channel::<()>();
        (Some(p), c)
    };

    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            ServiceControl::Stop => {
                send_stop.take().unwrap().send(()).unwrap();
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    status_handle.set_service_status(service_status(
        ServiceState::Running,
        ServiceControlAccept::STOP,
    ))?;

    actix_web::rt::spawn(async move {
        recv_stop.await.unwrap();
        status_handle
            .set_service_status(service_status(
                ServiceState::Stopped,
                ServiceControlAccept::empty(),
            ))
            .unwrap();

        actix_web::rt::System::current().stop()
    });

    sys.run().unwrap();

    Ok(())
}

fn service_status(
    current_state: ServiceState,
    controls_accepted: ServiceControlAccept,
) -> ServiceStatus {
    ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state,
        controls_accepted,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    }
}
