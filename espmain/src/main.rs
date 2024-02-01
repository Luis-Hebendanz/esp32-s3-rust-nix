// ignore unused imports
#![allow(unused_imports)]
// ignore unused variables
#![allow(unused_variables)]
// ignore unused functions
#![allow(dead_code)]

use ::vcp::vcp::Vcp;
use esp_idf_svc::sys::system;
use serde_json;

use core::convert::TryInto;
use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use esp_idf_svc::wifi::{AccessPointConfiguration, AuthMethod, ClientConfiguration, Configuration};

use esp_idf_svc::espnow::{EspNow, PeerInfo, BROADCAST};
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys::esp_nofail;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};

use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::{sys::EspError, task::thread::ThreadSpawnConfiguration};

use log::info;
use vcp::vcp;

// const SSID: &str = env!("WIFI_SSID");
// const PASSWORD: &str = env!("WIFI_PASS");
const ESP_NOW_CHANNEL: u8 = 1;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    init_espnow().unwrap();

    //vcp::complex_example_func();
    log::info!("Going into busy loop!=====");
    loop {
        esp_idf_svc::hal::delay::Delay::new_default().delay_ms(1000);
    }
}

pub fn mac_to_string(mac: &[u8]) -> String {
    let mut mac_str = String::new();
    for i in 0..mac.len() {
        mac_str.push_str(&format!("{:02x}", mac[i]));
        if i < mac.len() - 1 {
            mac_str.push(':');
        }
    }
    mac_str
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
    // let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
    //     ssid: SSID.into(),
    //     bssid: None,
    //     auth_method: AuthMethod::WPA2Personal,
    //     password: PASSWORD.into(),
    //     channel: None,
    // });
    let ap_conf = Configuration::AccessPoint(AccessPointConfiguration::default());

    wifi.set_configuration(&ap_conf)
        .map_err(|e| e.panic())
        .unwrap();

    wifi.start().map_err(|e| e.panic()).unwrap();
    log::info!("Wifi started");

    //  wifi.connect().map_err(|e| e.panic()).unwrap();
    // log::info!("Wifi connected");

    wifi.wait_netif_up().map_err(|e| e.panic()).unwrap();
    log::info!("Wifi netif up");

    let esp_now = esp_idf_svc::espnow::EspNow::take()
        .map_err(|e| e.panic())
        .unwrap();

    esp_now
        .add_peer(PeerInfo {
            peer_addr: BROADCAST,
            channel: ESP_NOW_CHANNEL,
            ifidx: 1,
            encrypt: false,
            ..Default::default()
        })
        .map_err(|e| e.panic())
        .unwrap();

    // Set current threads name
    ThreadSpawnConfiguration {
        name: Some("Receive Thread\0".as_bytes()),
        stack_size: 8096,
        priority: 15,
        pin_to_core: None,
        ..Default::default()
    }
    .set()
    .map_err(|e| e.panic())
    .unwrap();

    // TODO: I dont have a clue if this works!!!! - Jonathan
    let the_vcp = Arc::new(RwLock::new(Vcp::new(true))); // TODO: change the is_first
    let the_vcp2 = Arc::clone(&the_vcp);

    let esp_now_recv_cb = move |src: &[u8], data: &[u8]| {
        log::info!("Data recv from {}, len {}", mac_to_string(src), data.len());
        // TODO: catch error
        the_vcp
            .write()
            .unwrap()
            .receive(&serde_json::from_str(std::str::from_utf8(data).unwrap()).unwrap());

        //log::info!("Data: {}", std::str::from_utf8(data).unwrap());
    };
    esp_now.register_recv_cb(esp_now_recv_cb).unwrap();

    let send_thread = std::thread::Builder::new()
        .stack_size(8196)
        .spawn(move || {
            let mut count = 0;
            loop {
                //let data = format!("Hello, World! Count: {}", count);
                for m in the_vcp2.read().unwrap().outgoing_msgs.iter() {
                    let data = serde_json::to_string(&m).unwrap();
                    esp_now
                        .send(BROADCAST, data.as_bytes())
                        .map_err(|e| e.panic())
                        .unwrap();
                }
                the_vcp2.write().unwrap().outgoing_msgs.clear();
                count += 1;
                log::info!("Data sent, count {}", count);
                esp_idf_svc::hal::delay::Delay::new_default().delay_ms(1000);
            }
        })
        .unwrap();
    send_thread.join().unwrap();

    Ok(())
}
