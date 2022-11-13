// Port manages Cargo projects.
use std::process::Command;

use super::errors::ExecError;

pub enum PortOperation {
    Create{ name: String }
}

pub struct PortInput {
    pub op: PortOperation,
    pub target: String
}

pub fn port(input: PortInput) -> Result<(), ExecError> {
    match input.op {
        PortOperation::Create{ name } => {
            Command::new("cargo")
                .current_dir(input.target)
                .arg("new")
                .arg("--bin")
                .arg(name.as_str())
                .spawn()
                .unwrap();

            Ok(())
        }
    }    
}
