use crate::c64::C64;
use keyboard_types::KeyboardEvent;
use machine::client::{Client, ClientError, DirectClient, InteractiveClient, NonInteractiveClient};
use machine::mos6502::Registers;
use machine::{Addr, MachineStatus};
use std::sync::{Arc, Mutex};

type Result<T> = std::result::Result<T, ClientError>;

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
}

impl InteractiveClient for C64Client {
    type Error = ClientError;

    fn send_key(&mut self, key: KeyboardEvent) {}

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
