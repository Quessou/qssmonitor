#[derive(Debug, Default)]
pub struct Requester {}

impl Requester {
    pub fn get_process_name(&self, pid: i32) -> String {
        let process = procfs::process::Process::new(pid);
        process.unwrap().stat().unwrap().comm
    }
}
