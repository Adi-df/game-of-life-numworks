#![no_std]
#![no_main]

pub mod eadk;
use eadk::{SCREEN_HEIGHT, SCREEN_WIDTH};
use heapless::Vec;

#[export_name = "eadk_app_name"]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 10] = *b"GameOLife\0";

#[export_name = "eadk_app_api_level"]
#[link_section = ".rodata.eadk_app_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[export_name = "eadk_app_icon"]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

const CELL_SIZE: u16 = 2;
const LINE_SIZE: u16 = SCREEN_WIDTH / CELL_SIZE;
const COLUMN_SIZE: u16 = SCREEN_HEIGHT / CELL_SIZE;
const BOARD_SIZE: usize = LINE_SIZE as usize * COLUMN_SIZE as usize;

enum AppState {
    Editor,
    Running,
    StepByStep,
}

#[no_mangle]
fn _eadk_main() {
    let mut state: AppState = AppState::Editor;
    let mut board: [bool; BOARD_SIZE] = [false; BOARD_SIZE];
}
