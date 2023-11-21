use rppal::gpio::{Gpio, InputPin, IoPin, Mode, Pin};
use std::{error::Error, time::Duration};

const ADDR_PINS: [u8; 16] = [4, 17, 27, 22, 10, 9, 11, 5, 6, 13, 19, 26, 21, 20, 16, 12];
const DATA_PINS: [u8; 8] = [14, 15, 18, 23, 24, 25, 8, 7];
const CLOCK_PIN: u8 = 3;
const RW_PIN: u8 = 2;

fn main() -> Result<(), Box<dyn Error>> {
    let addr_pins = ADDR_PINS.map(|id| Gpio::new().unwrap().get(id).unwrap().into_input());

    let mut data_pins =
        DATA_PINS.map(|id| Gpio::new().unwrap().get(id).unwrap().into_io(Mode::Input));

    let rw_pin = Gpio::new()?.get(RW_PIN)?.into_input();

    let mut clock_pin = Gpio::new()?.get(CLOCK_PIN)?.into_output();

    for _ in 0..10 {
        clock_pin.toggle();
        std::thread::sleep(Duration::from_millis(100));
        if rw_pin.is_high() {
            set_mode(&mut data_pins, Mode::Output);
        } else {
            set_mode(&mut data_pins, Mode::Input);
        }

        let addr = read_word(&addr_pins);
        if data_pins[0].mode() == Mode::Output {
            match addr {
                0x0200..=0xfff0 => write_byte(&mut data_pins, 0xea),
                0xfffc => write_byte(&mut data_pins, 0),
                0xfffd => write_byte(&mut data_pins, 2),
                _ => (),
            }
            println!("Writing to {:04x}", addr);
        } else {
            println!("Reading from {:04x}: {:02x}", addr, read_byte(&data_pins));
        }
    }

    Ok(())
}

fn read_word(pins: &[InputPin]) -> u16 {
    let mut res = 0u16;
    for i in 0..16 {
        res |= (pins[i].read() as u16) << i;
    }
    res
}

fn read_byte(pins: &[IoPin]) -> u8 {
    let mut res = 0u8;
    for i in 0..8 {
        res |= (pins[i].read() as u8) << i;
    }
    res
}

fn write_byte(pins: &mut [IoPin], val: u8) {
    for i in 0..8 {
        if (val & (1 << i)) > 0 {
            pins[i].set_high();
        } else {
            pins[i].set_low();
        }
    }
}

fn set_mode(pins: &mut [IoPin], mode: Mode) {
    pins.iter_mut().for_each(|pin| {
        pin.set_mode(mode);
    });
}
