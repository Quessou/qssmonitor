use config::Config;
use config::File;
use libxdo_sys;
use tracing;

// Unsafe stuff
use std::alloc;
use std::os::raw::c_int;
use std::os::raw::c_uchar;

mod default_config;
mod filesystem;
mod logging;

use default_config::QssMontiorConfig;
use logging::initialization::initialize_subscriber;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = File::with_name(filesystem::paths::get_config_file_path().to_str().unwrap());
    let config = QssMontiorConfig::default();

    let _logging_guard = initialize_subscriber();

    if let Err(e) = filesystem::config_initialization::initialize_configuration(&config) {
        // Isn't there something better to do here ?
        if e != configgen_rs::Error::ConfigDirectoryAlreadyExists(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "",
        )) {
            tracing::error!("Could not initialize configuration directory");
            return Err(Box::new(e));
        }
    };
    let loaded_config = Config::builder()
        .add_source(config::File::from(config_file))
        .build()
        .unwrap();
    let read_config = loaded_config.try_deserialize::<QssMontiorConfig>().unwrap();

    println!("{:?}", read_config);

    unsafe {
        let xdo = libxdo_sys::xdo_new(std::ptr::null());
        let mut window: x11::xlib::Window = 0;
        let toto = libxdo_sys::xdo_get_active_window(xdo, &mut window);
        let mut name: *mut c_uchar = alloc::alloc(alloc::Layout::new::<c_uchar>());
        let mut name_len: c_int = 0;
        let mut name_type: c_int = 0;
        let _ =
            libxdo_sys::xdo_get_window_name(xdo, window, &mut name, &mut name_len, &mut name_type);
        let s: String = String::from_raw_parts(name, name_len.try_into().unwrap(), 100);
        println!("{}", s);
    }
    Ok(())
}
