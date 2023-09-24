use super::*;
use crate::utils::lock;
use crate::machine::{ Addr, Machine, MachineStatus };
use crate::mos6502::{ Registers };
use std::sync::{Arc, Mutex, MutexGuard};
use std;

type Result<T> = std::result::Result<T, ClientError>;

pub struct DirectClient<T: Machine> {
    machine_mtx: Arc<Mutex<T>>
}

impl<T: Machine> DirectClient<T> {
    pub fn new(machine: T) -> Self {
        DirectClient {
            machine_mtx: Arc::new(Mutex::new(machine))
        }
    }

    fn lock(&self) -> MutexGuard<T> {
        lock::<T>(&self.machine_mtx)
    }
}

impl<T: Machine> NonInteractiveClient for DirectClient<T> {
    type Error = ClientError;
    
    fn get_status(&self) -> MachineStatus {
        *self.lock().get_status()
    }

    fn start(&mut self) -> Result<()> {
        self.lock().start();
        Ok(())
    }

    fn reset(&mut self) -> Result<()> {
        self.lock().reset();
        Ok(())
    }

    fn pause(&mut self) -> Result<()> {
        self.lock().debug();
        Ok(())
    }

    fn resume(&mut self) -> Result<()> {
        self.lock().resume();
        Ok(())
    }

    fn next(&mut self) -> Result<bool> {
        Ok(self.lock().next())
    }

    fn get_mem_slice(&self, from: Addr, to: Addr) -> Result<Vec<u8>> {
        Ok(self.lock().memory().fragment(from, to))
    }
    
    fn get_cpu_state(&self) -> Result<Registers> {
        Ok(self.lock().cpu().registers.clone())
    }
}

