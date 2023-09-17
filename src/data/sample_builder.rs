use super::website_detection::WebsiteNameDetector;
use super::Sample;
use crate::process;
use crate::x;

#[derive(Debug, Default)]
pub struct SampleBuilder {
    // TODO: Make the interface x::Requester a bit more abstract
    xdo_requester: x::Requester,
    process_requester: process::Requester,
    website_name_detector: WebsiteNameDetector,
}

impl SampleBuilder {
    pub fn new(
        xdo_requester: x::Requester,
        process_requester: process::Requester,
        website_name_detector: WebsiteNameDetector,
    ) -> Self {
        SampleBuilder {
            xdo_requester,
            process_requester,
            website_name_detector,
        }
    }
    pub async fn build_sample(&self) -> Sample {
        let window = self.xdo_requester.get_active_window().await;
        let window_name = self.xdo_requester.get_window_name(window).await;
        let pid = self.xdo_requester.get_window_pid(window).await;
        let process_name = self.process_requester.get_process_name(pid);
        let website_name = self
            .website_name_detector
            .get_website_name(&process_name, &window_name);

        Sample::new(process_name.into(), window_name.into(), website_name, pid)
    }
}
