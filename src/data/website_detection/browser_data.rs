/// Data related to a browser
/// ### Attributes :
///    - `browser_name` : The name of the browser (i.e. Firefox)
///    - `window_name_suffix` : The suffix of the window name for the browser (i.e. for Firefox, it
///    is something like " - Mozilla Firefox"). We want to remove it to
#[derive(Debug, Clone)]
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
