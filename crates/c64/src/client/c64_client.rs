use crate::c64::C64;
use crate::key_utils::ui_event_to_c64_key_codes;
use crossbeam_channel::Receiver;
use keyboard_types::{KeyState, KeyboardEvent};
use machine::{
    client::*, debugger::DebuggerState, mos6502::Registers, utils::lock, Addr, Machine,
    MachineError, MachineStatus, Memory,
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type Result<T> = std::result::Result<T, MachineError>;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MachineState {
    pub status: MachineStatus,
    pub registers: Registers,
    pub last_op: String,
    pub memory_slice: Vec<u8>,
    pub screen: Vec<u8>,
    pub character_set: u8,
    pub debugger: DebuggerState,
    pub next_op: String,
}

pub struct C64Client {
    base_client: DirectClient<C64>, // awful!!!
}

impl C64Client {
    pub fn new(c64: C64) -> Self {
        C64Client {
            base_client: DirectClient::new(c64),
        }
    }

    pub fn start_sync(&mut self) -> Result<()> {
        self.base_client.start_sync()
    }

    pub fn mutex(&self) -> Arc<Mutex<C64>> {
        self.base_client.mutex()
    }

    pub fn get_debugger_state(&self) -> DebuggerState {
        self.base_client.lock().debugger_state.clone()
    }

    pub fn set_debugger_state(&mut self, state: DebuggerState) {
        self.base_client.lock().debugger_state = state;
    }

    pub fn step(&mut self) -> MachineState {
        self.handle_events();
        let c64 = self.base_client.lock();
        let registers = c64.cpu().registers.clone();
        let last_op = c64.disassemble(&c64.last_op, true, false);
        let next_op = c64.disassemble(&c64.decode_next(), false, true);
        let memory_slice = c64.memory().fragment(
            c64.debugger_state.observed_mem.start,
            c64.debugger_state.observed_mem.end,
        );
        let screen = c64.get_screen_memory();
        let character_set = c64.get_byte(0xd018); // https://www.c64-wiki.com/wiki/Character_set
        MachineState {
            status: c64.get_status(),
            registers,
            last_op,
            memory_slice,
            screen,
            character_set,
            debugger: c64.debugger_state.clone(),
            next_op,
        }
    }

    fn handle_events(&mut self) {
        use ClientEvent::*;
        if let Some(r) = &self.base_client.receiver {
            for event in r.try_iter().collect::<Vec<ClientEvent>>().iter() {
                match event {
                    EnableBreakpoint(b) => {
                        self.base_client.lock().debugger_state.add_breakpoint(&b)
                    }
                    DisableBreakpoint(b) => {
                        self.base_client.lock().debugger_state.remove_breakpoint(&b)
                    }
                    KeyPress(key_event) => self.send_key(key_event.clone()),
                    SetObservedMemory(range) => {
                        self.base_client.lock().debugger_state.observed_mem = range.clone()
                    }
                };
            }
        }
    }
}

impl InteractiveClient for C64Client {
    type Error = MachineError;

    fn send_key(&mut self, event: KeyboardEvent) {
        log::debug!("Sending key {:?}", event);
        if event.state == KeyState::Up {
            log::error!("Can't send key-up events on this client.");
            return ();
        }

        // key down
        let mut c64 = self.base_client.lock();
        c64.send_keys(&ui_event_to_c64_key_codes(&event), true);

        // key up (simulated with timeout)
        let mut up_event = event.clone();
        up_event.state = KeyState::Up;
        let arc = self.base_client.mutex();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(125));
            lock::<C64>(&arc).send_keys(&ui_event_to_c64_key_codes(&up_event), false);
        });
    }

    fn get_screen_memory(&self) -> Result<Vec<u8>> {
        self.base_client.get_mem_slice(0x400, 0x07e8)
    }
}

// this is stupid but I have no idea how to do it better
impl NonInteractiveClient for C64Client {
    type Error = MachineError;

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

    fn set_receiver(&mut self, r: Receiver<ClientEvent>) {
        self.base_client.set_receiver(r);
    }
}

impl Client for C64Client {}
