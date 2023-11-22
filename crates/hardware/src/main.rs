use rppal::gpio::{Gpio, InputPin, IoPin, Mode, Pin};
use std::{error::Error, time::Duration};
use std::io::Read;
use std::{fs::File, path::PathBuf};
use anyhow::Result;

const STEP: Duration = Duration::from_micros(10000);

const ADDR_PINS: [u8; 16] = [4, 17, 27, 22, 10, 9, 11, 5, 6, 13, 19, 26, 21, 20, 16, 12];
const DATA_PINS: [u8; 8] = [14, 15, 18, 23, 24, 25, 8, 7];
const CLOCK_PIN: u8 = 3;
const RW_PIN: u8 = 2;

fn main() -> Result<()> {
    let addr_pins = ADDR_PINS.map(|id| Gpio::new().unwrap().get(id).unwrap().into_input());

    let mut data_pins =
        DATA_PINS.map(|id| Gpio::new().unwrap().get(id).unwrap().into_io(Mode::Input));

    let rw_pin = Gpio::new()?.get(RW_PIN)?.into_input();

    let mut clock_pin = Gpio::new()?.get(CLOCK_PIN)?.into_output();

//    let program = get_file_as_byte_vec(&PathBuf::from(r"./tests/target/add-sub-14bit.p"))?;
    let rom = get_file_as_byte_vec(&PathBuf::from(r"./rom/kernal-64c.251913-01.bin"))?;
    let kernal = &rom[8192..];
    let basic = &rom[..8192];
    let blank = [0u8; 0x2000];
    let program = [basic, &blank, kernal].concat();


    let mut ram = [0u8; 65536];
    ram[0xfffc] = 0xe2;
    ram[0xfffd] = 0xfc;
    let mut i = 0;
    for cell in program {
        ram[0xa000 + i] = cell;
        i += 1;
    }

    for cycle in 0..4000000 {
        clock_pin.toggle();
        std::thread::sleep(STEP);
        if rw_pin.is_high() {
            set_mode(&mut data_pins, Mode::Output);
        } else {
            set_mode(&mut data_pins, Mode::Input);
        }

        ram[0xd012] = (cycle % 255) as u8;

        let addr = read_word(&addr_pins);
        if data_pins[0].mode() == Mode::Output {
            write_byte(&mut data_pins, ram[addr as usize]);
            println!("[{:07}] Writing from {:04x}: {:02x}", cycle, addr, read_byte(&data_pins));
        } else {
            ram[addr as usize] = read_byte(&data_pins);
            println!("[{:07}] Reading from {:04x}: {:02x}", cycle, addr, read_byte(&data_pins));
        }
    }

    print_screen(&ram);

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

fn get_file_as_byte_vec(filename: &PathBuf) -> Result<Vec<u8>> {
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn print_screen(mem: &[u8]) {
    let char_set: Vec<char> =
        "@abcdefghijklmnopqrstuvwxyz[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?\
         -ABCDEFGHIJKLMNOPQRSTUVWXYZ[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?\
         @ABCDEFGHIJKLMNOPQRSTUVWXYZ[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?\
         -abcdefghijklmnopqrstuvwxyz[£]↑← !\"#$%&'()*+,-./0123456789:;<=>?"
            .chars()
            .collect();
    let mut n = 0;
    println!();
    println!("{}", " ".repeat(44));
    print!("{}", "  ");
    for i in 0x0400..0x07e8 {
        let sc = mem[i];
        print!(
            "{}",
            format!("{}", char_set[sc as usize])
        );
        n += 1;
        if n % 40 == 0 {
            print!("{}", "  ");
            println!();
            print!("{}", "  ");
        }
    }
    println!("{}", " ".repeat(42));
    println!("              ");
}
