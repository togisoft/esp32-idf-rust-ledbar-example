use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;


fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    let pins = Peripherals::take().unwrap().pins;

    let mut led_list = [
        PinDriver::output(pins.gpio15.downgrade())?,
        PinDriver::output(pins.gpio2.downgrade())?,
        PinDriver::output(pins.gpio0.downgrade())?,
        PinDriver::output(pins.gpio4.downgrade())?,
        PinDriver::output(pins.gpio5.downgrade())?,
        PinDriver::output(pins.gpio18.downgrade())?,
        PinDriver::output(pins.gpio19.downgrade())?,
        PinDriver::output(pins.gpio21.downgrade())?,
        PinDriver::output(pins.gpio22.downgrade())?,
        PinDriver::output(pins.gpio23.downgrade())?,
    ];

    loop {
        let mut i: i8 = 0;
        let mut j: i8 = 9;

        while i < 6 && j > 4 {
            led_list[i as usize].set_high().unwrap();
            led_list[j as usize].set_high().unwrap();
            FreeRtos::delay_ms(200);
            i += 1;
            j -= 1;
        }

        while i > -1 && j < 11 {
            led_list[i as usize].set_low().unwrap();
            led_list[j as usize].set_low().unwrap();
            FreeRtos::delay_ms(200);
            i -= 1;
            j += 1;
        }
    }
}
