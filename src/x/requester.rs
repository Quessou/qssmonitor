use std::alloc;
use std::{ffi::c_uchar, os::raw::c_int};
use x11::xlib::Window;

use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Requester {
    xdo: *mut libxdo_sys::Struct_xdo,
    mtx: Mutex<u8>,
}

unsafe impl Send for Requester {}
unsafe impl Sync for Requester {}

impl Requester {
    pub async fn get_active_window(&self) -> Window {
        let mut window: x11::xlib::Window = 0;
        let guard = self.mtx.lock().await;
        unsafe {
            libxdo_sys::xdo_get_active_window(self.xdo, &mut window);
        }
        window
    }

    pub async fn get_window_name(&self, window: Window) -> String {
        let mut name: *mut c_uchar = unsafe { alloc::alloc(alloc::Layout::new::<c_uchar>()) };
        let mut name_len: c_int = 0;
        let mut name_type: c_int = 0;
        unsafe {
            libxdo_sys::xdo_get_window_name(
                self.xdo,
                window,
                &mut name,
                &mut name_len,
                &mut name_type,
            );
            let window_name: String =
                String::from_raw_parts(name, name_len.try_into().unwrap(), 128);
            window_name
        }
    }

    pub async fn get_window_pid(&self, window: Window) -> i32 {
        unsafe { libxdo_sys::xdo_get_pid_window(self.xdo, window) }
    }
}

impl Default for Requester {
    fn default() -> Self {
        let xdo = unsafe { libxdo_sys::xdo_new(std::ptr::null()) };
        Self {
            xdo,
            mtx: Mutex::new(0),
        }
    }
}

impl Drop for Requester {
    fn drop(&mut self) {
        unsafe {
            self.xdo.drop_in_place();
        }
    }
}
