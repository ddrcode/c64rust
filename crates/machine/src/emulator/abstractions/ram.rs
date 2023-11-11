use super::{Addr, Addressable};

pub trait RAM: Addressable {
    fn load(&mut self, addr: Addr, data: &[u8]) {
        for i in addr..addr + data.len() as Addr {
            self.write_byte(i, data[(i - addr) as usize]);
        }
    }
}
