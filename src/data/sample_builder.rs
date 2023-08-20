use super::Sample;
use crate::process;
use crate::x;

#[derive(Default)]
pub struct SampleBuilder {
    xdo_requester: x::Requester,
    process_requester: process::Requester,
}

impl SampleBuilder {
    pub fn build_sample(&self) -> Sample {
        let window = self.xdo_requester.get_active_window();
        let window_name = self.xdo_requester.get_window_name(window);
        let pid = self.xdo_requester.get_window_pid(window);
        let process_name = self.process_requester.get_process_name(pid);
        Sample::new(process_name.into(), window_name.into(), pid)
    }
}
