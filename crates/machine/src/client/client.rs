use crate::machine::{Addr, MachineStatus};
use crate::mos6502::Registers;
use crossbeam_channel::Receiver;
use keyboard_types::KeyboardEvent;

use super::ClientEvent;

pub trait NonInteractiveClient {
    type Error: std::error::Error + Send + Sync;

    fn start(&mut self) -> Result<(), Self::Error>;
    fn stop(&mut self) -> Result<(), Self::Error>;
    fn reset(&mut self) -> Result<(), Self::Error>;
    fn pause(&mut self) -> Result<(), Self::Error>;
    fn resume(&mut self) -> Result<(), Self::Error>;

    fn toggle_debug(&mut self) -> Result<(), Self::Error> {
        match self.get_status() {
            MachineStatus::Running => self.pause(),
            MachineStatus::Debug => self.resume(),
            _ => self.resume(),
        }
    }

    fn next(&mut self) -> Result<bool, Self::Error>;

    fn get_page(&self, page: u8) -> Result<Vec<u8>, Self::Error> {
        let addr = (page as Addr) * 256;
        self.get_mem_slice(addr, addr + 255)
    }

    fn get_mem_slice(&self, from: Addr, to: Addr) -> Result<Vec<u8>, Self::Error>;
    fn get_cpu_state(&self) -> Result<Registers, Self::Error>;
    fn get_status(&self) -> MachineStatus;

    fn is_running(&self) -> bool {
        self.get_status() != MachineStatus::Stopped
    }

    fn set_receiver(&mut self, r: Receiver<ClientEvent>);
}

pub trait InteractiveClient {
    type Error: std::error::Error + Send + Sync;

    fn send_key(&mut self, key: KeyboardEvent);
    fn get_screen_memory(&self) -> Result<Vec<u8>, Self::Error>;
}

pub trait Client: NonInteractiveClient + InteractiveClient {}
