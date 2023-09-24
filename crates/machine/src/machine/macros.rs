#[macro_export]
macro_rules! impl_reg_setter {
    ($type: ident) => {
        impl RegSetter<u8> for $type {
            fn set_A(&mut self, val: u8) {
                self.cpu_mut().registers.accumulator = Wrapping(val);
            }
            fn set_X(&mut self, val: u8) {
                self.cpu_mut().registers.x = Wrapping(val);
            }
            fn set_Y(&mut self, val: u8) {
                self.cpu_mut().registers.y = Wrapping(val);
            }
            fn set_SC(&mut self, val: u8) {
                self.cpu_mut().registers.stack = Wrapping(val);
            }
        }

        impl RegSetter<Wrapping<u8>> for $type {
            fn set_A(&mut self, val: Wrapping<u8>) {
                self.cpu_mut().registers.accumulator = val;
            }
            fn set_X(&mut self, val: Wrapping<u8>) {
                self.cpu_mut().registers.x = val;
            }
            fn set_Y(&mut self, val: Wrapping<u8>) {
                self.cpu_mut().registers.y = val;
            }
            fn set_SC(&mut self, val: Wrapping<u8>) {
                self.cpu_mut().registers.stack = val;
            }
        }
    };
}
