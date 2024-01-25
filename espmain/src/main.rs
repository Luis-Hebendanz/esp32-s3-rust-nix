// ignore unused imports
#![allow(unused_imports)]
// ignore unused variables
#![allow(unused_variables)]
// ignore unused functions
#![allow(dead_code)]

use esp_idf_svc::sys::system;

use core::convert::TryInto;

use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};

use esp_idf_svc::espnow::{EspNow, PeerInfo, BROADCAST};
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys::esp_nofail;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};

use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::{sys::EspError, task::thread::ThreadSpawnConfiguration};



use log::info;

const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");
const ESP_NOW_CHANNEL: u8 = 1;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Configure a thread to run on the main core
    // ThreadSpawnConfiguration {
    //     name: Some("Main Thread\0".as_bytes()),
    //     stack_size: 4096,
    //     priority: 5,
    //     pin_to_core: Some(Core::Core0),
    //     ..Default::default()
    // }
    // .set().map_err(|e| e.panic()).unwrap();

    init_espnow().unwrap();

    //vcp::complex_example_func();
    log::info!("Going into busy loop!=====");
    loop {
        esp_idf_svc::hal::delay::Delay::new_default().delay_ms(1000);
    }
}

fn init_espnow() -> anyhow::Result<(), anyhow::Error> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See
    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))
            .map_err(|e| e.panic())
            .unwrap(),
        sys_loop,
    )?;

    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.into(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.into(),
        channel: None,
    });

    wifi.set_configuration(&wifi_configuration)
        .map_err(|e| e.panic())
        .unwrap();

    wifi.start().map_err(|e| e.panic()).unwrap();
    log::info!("Wifi started");

    wifi.connect().map_err(|e| e.panic()).unwrap();
    log::info!("Wifi connected");

    wifi.wait_netif_up().map_err(|e| e.panic()).unwrap();
    log::info!("Wifi netif up");

    let esp_now = esp_idf_svc::espnow::EspNow::take()
        .map_err(|e| e.panic())
        .unwrap();

    esp_now
        .add_peer(PeerInfo {
            peer_addr: BROADCAST,
            channel: ESP_NOW_CHANNEL,
            ifidx: 0,
            encrypt: false,
            ..Default::default()
        })
        .map_err(|e| e.panic())
        .unwrap();

    Ok(())
}
