type Addr = u16;

pub struct RAM {
    memory: Box<[u8]>
}

// TODO consider better way of initializing the memory
// see this: https://www.reddit.com/r/rust/comments/jzwwqb/about_creating_a_boxed_slice/
// and this: https://www.reddit.com/r/rust/comments/c4zdue/newbie_question_array_in_a_struct/

impl RAM {
    pub fn new(size: usize) -> Self {
        RAM {
            memory: vec![0;size].into_boxed_slice()
        }
    }

    pub fn get_byte(&self, addr: Addr) -> u8 {
        self.memory[addr as usize]
    }

    pub fn set_byte(&mut self, addr: Addr, val: u8) {
        self.memory[addr as usize] = val;
    }

    pub fn get_word(&self, addr: Addr) -> u16 {
        let idx = addr as usize;
        (self.memory[idx] as u16) | ((self.memory[idx+1] as u16) << 8)

    }

    pub fn set_word(&mut self, addr: Addr, val: u16) {
        let idx = addr as usize;
        let [high, low] = val.to_be_bytes();
        self.memory[idx] = low;
        self.memory[idx+1] = high; // little endian!
    }

    pub fn write(&mut self, addr: Addr, data: &[u8]) {
        let mut idx = addr as usize;
        for byte in data.iter() {
            self.memory[idx] = *byte;
            idx += 1;
        }
    }
}
