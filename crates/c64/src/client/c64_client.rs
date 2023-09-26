use crate::c64::C64;
use keyboard_types::KeyboardEvent;
use machine::client::*;
use machine::debugger::DebuggerState;
use machine::mos6502::Registers;
use machine::{Addr, Machine, MachineStatus};
use std::sync::{Arc, Mutex};

type Result<T> = std::result::Result<T, ClientError>;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MachineState {
    pub registers: Registers,
    pub last_op: String,
    pub memory_slice: Vec<u8>,
    pub screen: String,
}

pub struct C64Client {
    base_client: DirectClient<C64>,    // awful!!!
    pub debugger_state: DebuggerState, // event_emitter: EventEmitter,
}

impl C64Client {
    pub fn new(c64: C64) -> Self {
        C64Client {
            base_client: DirectClient::new(c64),
            debugger_state: DebuggerState {
                observed_mem: (0..200),
                ..Default::default()
            },
        }
    }

    pub fn start_sync(&mut self) -> Result<()> {
        self.base_client.start_sync()
    }

    pub fn mutex(&self) -> Arc<Mutex<C64>> {
        self.base_client.mutex()
    }

    pub fn step(&self) -> MachineState {
        let c64 = self.base_client.lock();
        let registers = c64.cpu().registers.clone();
        let last_op = c64.disassemble(&c64.last_op, true);
        let memory_slice = c64.memory().fragment(
            self.debugger_state.observed_mem.start,
            self.debugger_state.observed_mem.end,
        );
        let screen = c64.get_screen_memory();
        MachineState {
            registers,
            last_op,
            memory_slice,
            screen,
        }
    }
}

impl InteractiveClient for C64Client {
    type Error = ClientError;

    fn send_key(&mut self, key: KeyboardEvent) {
        log::info!("Sending key {:?}", key);
    }

    fn get_screen_memory(&self) -> Result<Vec<u8>> {
        self.base_client.get_mem_slice(0x400, 0x07e8)
    }
}

// this is stupid but I have no idea how to do it better
impl NonInteractiveClient for C64Client {
    type Error = ClientError;

    fn get_status(&self) -> MachineStatus {
        self.base_client.get_status()
    }

    fn start(&mut self) -> Result<()> {
        self.base_client.start()
    }

    fn reset(&mut self) -> Result<()> {
        self.base_client.reset()
    }

    fn stop(&mut self) -> Result<()> {
        self.base_client.stop()
    }

    fn pause(&mut self) -> Result<()> {
        self.base_client.pause()
    }

    fn resume(&mut self) -> Result<()> {
        self.base_client.resume()
    }

    fn next(&mut self) -> Result<bool> {
        self.base_client.next()
    }

    fn get_mem_slice(&self, from: Addr, to: Addr) -> Result<Vec<u8>> {
        self.base_client.get_mem_slice(from, to)
    }

    fn get_cpu_state(&self) -> Result<Registers> {
        self.base_client.get_cpu_state()
    }
}

impl Client for C64Client {}
