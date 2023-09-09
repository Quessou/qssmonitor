pub struct BrowserData {
    pub browser_name: String,
    pub window_name_suffix: String,
}

impl BrowserData {
    pub fn new(browser_name: String, window_name_suffix: String) -> Self {
        BrowserData {
            browser_name,
            window_name_suffix,
        }
    }
}
