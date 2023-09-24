use super::*;
use crate::machine::{Addr, Machine, MachineStatus};
use crate::mos6502::Registers;
use crate::utils::lock;
use runtime::*;
use std;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

type Result<T> = std::result::Result<T, ClientError>;

/// DirectClient is an implementation of Client that runs
/// the machine directly (in a thread) rather than
/// connecting to some remote machine
pub struct DirectClient<T: Machine + Send + 'static> {
    machine_mtx: Arc<Mutex<T>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl<T: Machine + Send + 'static> DirectClient<T> {
    pub fn new(machine: T) -> Self {
        DirectClient {
            machine_mtx: Arc::new(Mutex::new(machine)),
            handle: None,
        }
    }

    fn lock(&self) -> MutexGuard<T> {
        lock::<T>(&self.machine_mtx)
    }

    fn start_machine_in_thread(&mut self) {
        let arc = self.machine_mtx.clone();
        let handle = thread::spawn(move || machine_loop::<T>(arc));
        self.handle = Some(handle);
    }

    fn join(&mut self) -> Result<()> {
        if let Some(handle) = self.handle.take() {
            if !handle.is_finished() {
                return handle
                    .join()
                    .or(Err(ClientError::new("Failed to join machine's thread")));
            }
        }
        Ok(())
    }

    pub fn start_sync(&mut self) -> Result<()> {
        self.start()?;
        self.join()
    }
}

impl<T: Machine + Send + 'static> NonInteractiveClient for DirectClient<T> {
    type Error = ClientError;

    fn get_status(&self) -> MachineStatus {
        self.lock().get_status()
    }

    fn start(&mut self) -> Result<()> {
        if self.is_running() {
            return Err(ClientError::new("Machine is already running"));
        }
        self.start_machine_in_thread();
        Ok(())
    }

    fn reset(&mut self) -> Result<()> {
        self.lock().reset();
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        if self.is_running() {
            self.lock().stop();
        }
        self.join()
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
