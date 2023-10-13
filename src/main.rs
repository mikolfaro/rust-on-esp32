mod color;
mod wifi;

use embedded_svc::http::Method;
use esp_idf_hal::ledc::{LedcTimerDriver, LedcDriver, config::TimerConfig, config::Resolution};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, timer::EspTaskTimerService,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use crate::color::Color;
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

    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default()
            .frequency(50_isize.Hz().into())
            .resolution(Resolution::Bits14)
    ).unwrap();

    let mut red_led_driver = Arc::new(Mutex::new(LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        peripherals.pins.gpio39,
    ).unwrap()));
    let mut green_led_driver = Arc::new(Mutex::new(LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        peripherals.pins.gpio40,
    ).unwrap()));
    let mut blue_led_driver = Arc::new(Mutex::new(LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        peripherals.pins.gpio41,
    ).unwrap()));

    server
        .fn_handler("/color", Method::Post, move |mut req| {
            let mut buffer = [0_u8, 6];
            req.read(&mut buffer)?;
            let color: Color = from_utf8(&buffer).try_into()?;

            info!("Setting color to {:?}", color);

            // let mut resp = req.into_ok_response()?;
            // resp.write("Hell, World!".as_bytes())?;

            // red_led_driver.lock().unwrap().set_duty(100);
            // green_led_driver.lock().unwrap().set_duty(100);
            // blue_led_driver.lock().unwrap().set_duty(100);

            info!("Toggle Pin");

            Ok(())
        })
        .unwrap();

    loop {
        sleep(Duration::from_secs(1));
    }
}
