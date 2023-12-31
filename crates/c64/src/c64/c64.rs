#![allow(non_snake_case)]

use super::{C64Memory, CIA1, CIA2, VIC_II};
use crate::key_utils::C64KeyCode;
use machine::{
    cli::{FromProfile, Profile},
    debugger::{DebugMachine, Debugger, DebuggerState},
    impl_reg_setter,
    mos6502::{execute_operation, Operation, MOS6502},
    Addr, Cycles, FromConfig, Machine, MachineConfig, MachineStatus, Memory, RegSetter, emulator::{abstractions::{Device, Accessor}, components::CIA_6526},
};
use std::num::Wrapping;

pub struct C64 {
    config: MachineConfig,
    mos6510: MOS6502,
    mem: C64Memory,
    gpu: VIC_II,
    cia1: Device<CIA1>,
    cia2: Device<CIA2>,
    status: MachineStatus,
    cycles: u64,
    pub debugger_state: DebuggerState,
    pub last_op: Operation,
}

impl C64 {
    pub fn new(config: MachineConfig) -> Self {
        let cia1 = Device::from(CIA1::new());
        let cia2 = Device::from(CIA2::new());
        C64 {
            config,
            mos6510: MOS6502::new(),
            mem: C64Memory::new(&cia1, &cia2),
            gpu: VIC_II::new(),
            cia1,
            cia2,
            status: MachineStatus::Stopped,
            cycles: 0,
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

    pub fn get_screen_memory(&self) -> Vec<u8> {
        self.memory().fragment(0x0400, 0x07e8)
    }

    pub fn key_down(&mut self, ck: C64KeyCode) {
        self.cia1.lock().keyboard.key_down(ck as u8);

        // let sc = VIC_II::ascii_to_petscii(ch);
        // self.memory_mut().write_byte(0x0277, sc);
        // self.memory_mut().write_byte(0x00c6, 1); // number of keys in the keyboard buffer
        // self.memory_mut().write_byte(0xffe4, 22);

        // self.machine.memory_mut().write_byte(0xc5, 2);
        // self.machine.memory_mut().write_byte(0xcb, 3);
    }

    pub fn key_up(&mut self, ck: C64KeyCode) {
        self.cia1.lock().keyboard.key_up(ck as u8);
    }

    pub fn send_keys(&mut self, vec: &Vec<C64KeyCode>, is_down: bool) {
        vec.iter().for_each(|kc: &C64KeyCode| {
            if is_down {
                self.key_down(*kc)
            } else {
                self.key_up(*kc)
            };
        });
    }

    pub fn is_io(&self, addr: Addr) -> bool {
        let flag = self.memory().read_byte(1) & 0b00000111;
        flag & 0b100 > 0 && flag & 11 > 0 && addr >= 0xdc00 && addr <= 0xdc0f
    }
}

impl_reg_setter!(C64);

impl Machine for C64 {
    type MemoryImpl = C64Memory;

    fn memory(&self) -> &C64Memory {
        &self.mem
    }

    fn memory_mut(&mut self) -> &mut C64Memory {
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

    fn get_cycles(&self) -> Cycles {
        self.cycles
    }

    fn advance_cycles(&mut self, cycles: u8) {
        self.cycles = self.cycles.wrapping_add(cycles.into());
    }

    fn execute_operation(&mut self, op: &Operation) -> u8 {
        let res = execute_operation(&op, self);
        self.last_op = op.clone();
        res
    }

    fn post_next(&mut self, op: &Operation) {
        // FIXME this is an ugly workaround to fix screen scannig
        // value at 0d012 represents currently scanned line
        // if not updated  - screen won't be refreshed
        // it should be implemented at VIC level
        self.write_byte(0xd012, (self.cycles % 255) as u8);

        self.cia1.lock().tick_times(op.def.len());
        self.cia2.lock().tick_times(op.def.len());

        if self.get_status() == MachineStatus::Running && self.should_pause(op) {
            self.start_debugging();
        }
        self.update_debugger_state();
    }
}

impl Debugger for C64 {
    type MachineImpl = C64;

    fn debugger_state(&self) -> &DebuggerState {
        &self.debugger_state
    }

    fn debugger_state_mut(&mut self) -> &mut DebuggerState {
        &mut self.debugger_state
    }
    fn machine(&self) -> &C64 {
        self
    }
}

impl DebugMachine for C64 {}

impl FromConfig for C64 {
    fn from_config(config: MachineConfig) -> Self {
        C64::new(config)
    }
}

impl FromProfile for C64 {
    fn from_profile(profile: &Profile) -> Self {
        let mut c64 = C64::new((&profile.config).into());
        if let Some(dc) = &profile.debug {
            c64.debugger_state = DebuggerState::from(dc);
        }
        c64
    }
}
