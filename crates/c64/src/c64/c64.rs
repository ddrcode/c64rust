#![allow(non_snake_case)]

use super::{C64KeyCode, C64Memory, CIA1, CIA6526, VIC_II};
use machine::{
    debugger::{DebugMachine, Debugger, DebuggerState},
    impl_reg_setter,
    mos6502::{execute_operation, Operation, MOS6502},
    Addr, Machine, MachineConfig, MachineStatus, Memory, RegSetter,
};
use std::num::Wrapping;

pub struct C64 {
    config: MachineConfig,
    mos6510: MOS6502,
    mem: Box<dyn Memory + Send>,
    gpu: VIC_II,
    pub cia1: CIA1,
    status: MachineStatus,
    cycle: u128, // FIXME remove!
    pub debugger_state: DebuggerState,
    pub last_op: Operation,
}

impl C64 {
    pub fn new(config: MachineConfig) -> Self {
        let size = config.ram_size.clone();
        C64 {
            config,
            mos6510: MOS6502::new(),
            mem: Box::new(C64Memory::new(size)),
            gpu: VIC_II {},
            cia1: CIA1::new(0xdc00),
            status: MachineStatus::Stopped,
            cycle: 0,
            debugger_state: DebuggerState::default(),
            last_op: Operation::default(),
        }
    }

    pub fn load(&mut self, progmem: &[u8], addr: u16) {
        self.memory_mut().write(addr, progmem);
    }

    pub fn print_screen(&self) {
        self.gpu.print_screen(&self.memory());
    }

    pub fn get_screen_memory(&self) -> String {
        let mut chars = String::new();
        for i in 0x0400..0x07e8 {
            let sc = self.get_byte(i);
            let ch = VIC_II::to_ascii(sc);
            chars.push(ch);
        }
        chars
    }

    pub fn send_key(&mut self, ck: C64KeyCode) {
        self.cia1.keyboard.key_down(ck as u8);

        // let sc = VIC_II::ascii_to_petscii(ch);
        // self.memory_mut().set_byte(0x0277, sc);
        // self.memory_mut().set_byte(0x00c6, 1); // number of keys in the keyboard buffer
        // self.memory_mut().set_byte(0xffe4, 22);

        // self.machine.memory_mut().set_byte(0xc5, 2);
        // self.machine.memory_mut().set_byte(0xcb, 3);
    }

    pub fn send_key_with_modifier(&mut self, ck: C64KeyCode, modifier: C64KeyCode) {
        self.cia1.keyboard.key_down(modifier as u8);
        self.cia1.keyboard.key_down(ck as u8);
    }

    pub fn is_io(&self, addr: Addr) -> bool {
        let flag = self.memory().get_byte(1) & 0b00000111;
        flag & 0b100 > 0 && flag & 11 > 0 && addr >= 0xdc00 && addr <= 0xdc0f
    }
}

impl_reg_setter!(C64);

impl Machine for C64 {
    fn memory(&self) -> &Box<dyn Memory + Send + 'static> {
        &self.mem
    }

    fn memory_mut(&mut self) -> &mut Box<dyn Memory + Send + 'static> {
        &mut self.mem
    }

    fn cpu(&self) -> &MOS6502 {
        &self.mos6510
    }

    fn cpu_mut(&mut self) -> &mut MOS6502 {
        &mut self.mos6510
    }

    fn get_config(&self) -> &MachineConfig {
        &self.config
    }

    fn get_status(&self) -> MachineStatus {
        self.status
    }

    fn set_status(&mut self, status: MachineStatus) {
        self.status = status;
    }

    fn execute_operation(&mut self, op: &Operation) -> u8 {
        self.last_op = op.clone();
        execute_operation(&op, self)
    }

    fn get_byte(&self, addr: Addr) -> u8 {
        if self.is_io(addr) {
            self.cia1.get_byte(addr)
        } else {
            self.memory().get_byte(addr)
        }
    }

    fn set_byte(&mut self, addr: Addr, val: u8) {
        if self.is_io(addr) {
            self.cia1.set_byte(addr, val)
        } else {
            self.memory_mut().set_byte(addr, val)
        }
    }

    fn post_next(&mut self, op: &Operation) {
        // FIXME this is an ugly workaround to fix screen scannig
        // value at 0d012 represents currently scanned line
        // if not updated  - screen won't be refreshed
        // it should be implemented at VIC level
        self.cycle = self.cycle.wrapping_add(1);
        self.set_byte(0xd012, (self.cycle % 255) as u8);

        if self.get_status() == MachineStatus::Running && self.should_pause(op) {
            self.start_debugging();
        }
    }
}

impl Debugger for C64 {
    fn debugger_state(&self) -> &DebuggerState {
        &self.debugger_state
    }

    fn machine(&self) -> &dyn Machine {
        self
    }
}

impl DebugMachine for C64 {}
