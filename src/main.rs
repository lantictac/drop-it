use std::ffi::OsString;

use windows_service::{define_windows_service, service_dispatcher};

const SERVICE_NAME: &str = "DropIt";

define_windows_service!(ffi_service_main, service_main);

fn service_main(_arguments: Vec<OsString>) {
    drop_it::run(SERVICE_NAME).unwrap()
}

fn main() -> windows_service::Result<()> {
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)
}
