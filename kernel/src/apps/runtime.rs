use crate::process::Process;

pub struct ApplicationRuntime;

impl ApplicationRuntime {
    pub const fn new() -> Self { Self }

    pub fn validate_entry(entry: u64) -> Result<(), &'static str> {
        if entry == 0 || entry >= 0x0000_8000_0000_0000 {
            return Err("invalid application entry address");
        }
        Ok(())
    }

    pub fn prepare(&self, _process: &Process) -> Result<(), &'static str> {
        Ok(())
    }
}
