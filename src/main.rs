mod wifi;

use embedded_svc::http::Method;
use esp_idf_hal::{gpio::PinDriver, prelude::Peripherals};
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, timer::EspTaskTimerService,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use crate::wifi::wifi;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // 41 40 39

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let timer_service = EspTaskTimerService::new().unwrap();
    let _wifi = wifi(
        peripherals.modem,
        sysloop,
        Some(EspDefaultNvsPartition::take().unwrap()),
        timer_service,
    )
    .unwrap();

    let mut server = EspHttpServer::new(&Configuration::default()).unwrap();

    let red_led_pin = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio39).unwrap()));
    let green_led_pin = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio40).unwrap()));
    let blue_led_pin = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio41).unwrap()));

    server
        .fn_handler("/", Method::Get, move |req| {
            let mut resp = req.into_ok_response().unwrap();
            resp.write("Hell, World!".as_bytes()).unwrap();

            red_led_pin.lock().unwrap().toggle().unwrap();
            green_led_pin.lock().unwrap().toggle().unwrap();
            blue_led_pin.lock().unwrap().toggle().unwrap();

            info!("Toggle Pin");

            Ok(())
        })
        .unwrap();

    loop {
        sleep(Duration::from_secs(1));
    }
}
