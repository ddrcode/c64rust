use thiserror::Error;

#[derive(Error, Debug)]
pub enum MachineError {
    #[error("Machine client failed: {0}")]
    Client(String),
}

// impl std::fmt::Display for MachineError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }



