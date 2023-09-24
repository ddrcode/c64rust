use super::Machine;

pub struct MachineEvents {
    pub on_next: Option<fn(&mut dyn Machine, &u128)>,
}
