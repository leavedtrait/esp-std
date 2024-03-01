
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
//use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::EspWifi;

use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;

use crate::wifi::wifi::get;

const PASS: &str = "#macharia@00";
const SSID: &str = "Sming_ext";

mod wifi;
fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
 

    // Configure Wifi
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = EspWifi::new(peripherals.modem, sysloop, Some(nvs))?;

    let mut ssid_str = heapless::String::<32>::new();
    ssid_str.push_str(SSID).unwrap_or_default();
    let mut pass_str = heapless::String::<64>::new();
    pass_str.push_str(PASS).unwrap_or_default();

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid_str,
        password: pass_str,
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    }))?;

    // Start Wifi
    wifi.start()?;

    // Connect Wifi
    wifi.connect()?;

    // Confirm Wifi Connection
    while !wifi.is_connected()?{
        // Get and print connection configuration
        let config = wifi.get_configuration().unwrap_or_default();
        log::info!("Waiting for station {:?}", config);
    }

    log::info!("Connected");

    let mut led = PinDriver::output(peripherals.pins.gpio2)?;
    log::info!("Hello, world!");
    loop {
        //get("http://neverssl.com/")?;
        led.set_high()?;
        log::info!("On!");
        // we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(1000);

        led.set_low()?;
        log::info!("Off!");
        FreeRtos::delay_ms(1000);
    }
}
