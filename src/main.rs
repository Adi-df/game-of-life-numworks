#![no_std]
#![no_main]

pub mod eadk;

#[export_name = "eadk_app_name"]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 10] = *b"GameOLife\0";

#[export_name = "eadk_app_api_level"]
#[link_section = ".rodata.eadk_app_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[export_name = "eadk_app_icon"]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

#[no_mangle]
fn _eadk_main() {}
