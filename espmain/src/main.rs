use esp_idf_svc::sys::system;

use core::convert::TryInto;

use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};

use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use esp_idf_svc::espnow::{EspNow, PeerInfo, BROADCAST};
use esp_idf_svc::sys::esp_nofail;
use log::info;

const SSID: &str = "WIFI_SSID";
const PASSWORD: &str = "WIFI_PASS";
const ESP_NOW_CHANNEL: u8 = 1;


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    init_espnow().unwrap();

    espmain::complex_example_func();
    vcp::complex_example_func();
}


fn init_espnow() -> anyhow::Result<(), anyhow::Error> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See
    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    let esp_now = esp_idf_svc::espnow::EspNow::take().map_err(|e| e.panic()).unwrap();

    esp_now
        .add_peer(PeerInfo {
            peer_addr: BROADCAST,
            channel: ESP_NOW_CHANNEL,
            ifidx: 0,
            encrypt: false,
            ..Default::default()
        })
        .map_err(|e| e.panic()).unwrap();


    Ok(())
}