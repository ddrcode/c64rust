pub type Addr = u16;

pub trait Memory {
    // TODO: Must check whether the three corresponding its at addr 0x00 are 1
    // check https://www.c64-wiki.com/wiki/Bank_Switching for details
    fn mem(&self, addr: Addr) -> &[u8];
    fn init_rom(&mut self, data: &[u8]);

    fn get_byte(&self, addr: Addr) -> u8 {
        self.mem(addr)[addr as usize]
    }

    fn get_word(&self, addr: Addr) -> u16 {
        let idx = addr as usize;
        let mem = self.mem(addr);
        (mem[idx] as u16) | ((mem[idx + 1] as u16) << 8)
    }

    fn set_byte(&mut self, addr: Addr, val: u8);
    fn set_word(&mut self, addr: Addr, val: u16);

    fn write(&mut self, addr: Addr, data: &[u8]) {
        let mut idx = addr;
        for byte in data.iter() {
            self.set_byte(idx, *byte);
            idx += 1;
        }
    }

    fn dump(&self, from: Addr, to: Addr) {
        let range = std::ops::Range {
            start: from,
            end: to,
        };
        for i in range {
            print!("{}", char::from(self.get_byte(i)));
            // print!("{:04x}: {:02x} ", i, self.get_byte(i));
        }
        println!();
    }
}
