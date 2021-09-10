use std::time::Duration;

use futures::channel::oneshot;
use windows_service::service::{
    ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus, ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};

pub struct ServiceEventHandler<'a> {
    service_name: &'a str,
}

impl<'a> ServiceEventHandler<'a> {
    pub fn new(service_name: &'a str) -> Self {
        ServiceEventHandler { service_name }
    }

    const RUNNING: ServiceStatus = ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::ZERO,
        process_id: None,
    };

    const STOPPED: ServiceStatus = ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::ZERO,
        process_id: None,
    };

    pub fn run(&self) -> windows_service::Result<()> {
        let (mut send_stop, recv_stop) = {
            let (p, c) = oneshot::channel::<()>();
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

        let status_handle = service_control_handler::register(self.service_name, event_handler)?;

        status_handle.set_service_status(Self::RUNNING)?;

        actix_web::rt::spawn(async move {
            recv_stop.await.unwrap();
            status_handle.set_service_status(Self::STOPPED).unwrap();

            actix_web::rt::System::current().stop()
        });

        Ok(())
    }
}
