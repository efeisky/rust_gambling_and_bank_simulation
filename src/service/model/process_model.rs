#[derive(PartialEq)]
pub enum ProcessType {
    Login, Register, Invalid
}

pub struct ProcessResult {
    pub process: ProcessType,
    pub status: bool,
    pub username: String
}

impl ProcessResult {
    pub fn default(process_type: ProcessType) -> ProcessResult {
        ProcessResult {
            process: process_type,
            status: false,
            username: String::default(),
        }
    }
}