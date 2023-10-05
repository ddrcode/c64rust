pub type Addr = u16;

pub trait Memory {
    // TODO: Must check whether the three corresponding its at addr 0x00 are 1
    // check https://www.c64-wiki.com/wiki/Bank_Switching for details
    fn mem(&self, addr: Addr) -> &[u8];
    fn init_rom(&mut self, data: &[u8]);
    fn init_rom_at_addr(&mut self, addr: Addr, data: &[u8]);
    fn size(&self) -> usize;

    fn read_byte(&self, addr: Addr) -> u8 {
        self.mem(addr)[addr as usize]
    }

    fn read_word(&self, addr: Addr) -> u16 {
        let idx = addr as usize;
        let mem = self.mem(addr);
        (mem[idx] as u16) | ((mem[addr.wrapping_add(1) as usize] as u16) << 8)
    }

    fn write_byte(&mut self, addr: Addr, val: u8);

    fn write(&mut self, addr: Addr, data: &[u8]) {
        let mut idx = addr;
        for byte in data.iter() {
            self.write_byte(idx, *byte);
            idx = idx.wrapping_add(1);
        }
    }

    fn dump(&self, from: Addr, to: Addr) {
        let range = std::ops::Range {
            start: from,
            end: to,
        };
        for i in range {
            print!("{}", char::from(self.read_byte(i)));
            // print!("{:04x}: {:02x} ", i, self.get_byte(i));
        }
        println!();
    }

    fn fragment(&self, from: Addr, to: Addr) -> Vec<u8> {
        let mut vec = Vec::<u8>::with_capacity((to - from) as usize);
        let range = std::ops::Range {
            start: from,
            end: to,
        };
        for i in range {
            vec.push(self.read_byte(i));
        }
        vec
    }
}
