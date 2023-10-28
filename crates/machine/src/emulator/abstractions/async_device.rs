use crate::utils::lock;
use std::sync::{Arc, Mutex, MutexGuard};

pub trait DeviceTrait {}

#[derive(Clone)]
pub struct Device<T: DeviceTrait>(Arc<Mutex<T>>);

pub trait Accessor<T: DeviceTrait> {
    fn mutex(&self) -> Arc<Mutex<T>>;
    fn lock(&self) -> MutexGuard<T>;
}

impl<T: DeviceTrait> Accessor<T> for Device<T> {
    fn mutex(&self) -> Arc<Mutex<T>> {
        self.0.clone()
    }

    fn lock(&self) -> MutexGuard<T> {
        lock(&self.0)
    }
}

impl<T: DeviceTrait> From<T> for Device<T> {
    fn from(device: T) -> Self {
        Device(Arc::new(Mutex::new(device)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accessor() {
        struct A {}
        impl DeviceTrait for A {}
        let _my_device = Device::from(A {});

        let d = A {};
        let _his_device: Device<A> = d.into();
    }

    #[test]
    fn nested_devices() {
        #[derive(Clone, Debug)]
        struct Ram(u16);
        struct Rom {}
        impl DeviceTrait for Ram {}
        impl DeviceTrait for Rom {}

        struct Gpu {
            ram: Device<Ram>,
        }
        impl DeviceTrait for Gpu {}

        struct Machine {
            ram: Device<Ram>,
            gpu: Device<Gpu>,
        }

        impl Machine {
            fn new() -> Self {
                let ram = Device::from(Ram(0));
                let gpu = Device::from(Gpu { ram: ram.clone() });
                Machine { ram, gpu }
            }
        }

        let m = Machine::new();
        m.gpu.lock().ram.lock().0 = 0xff;
        assert_eq!(0xff, m.ram.lock().0);
    }
}
