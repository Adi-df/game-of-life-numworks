#![no_std]
#![no_main]

pub mod eadk;
use eadk::{display, Color, Rect, SCREEN_HEIGHT, SCREEN_WIDTH};

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
const LINE_SIZE: usize = (SCREEN_WIDTH / CELL_SIZE) as usize;
const COLUMN_SIZE: usize = (SCREEN_HEIGHT / CELL_SIZE) as usize;
const BOARD_SIZE: usize = LINE_SIZE * COLUMN_SIZE;

type Board = [bool; BOARD_SIZE];

enum AppState {
    Editor,
    Running,
    StepByStep,
}

fn draw_board(board: &Board) {
    (0..LINE_SIZE).for_each(|x| {
        (0..COLUMN_SIZE).for_each(|y| {
            display::push_rect_uniform(
                Rect {
                    x: x as u16 * CELL_SIZE,
                    y: y as u16 * CELL_SIZE,
                    width: CELL_SIZE,
                    height: CELL_SIZE,
                },
                if board[(x + y * LINE_SIZE) as usize] {
                    Color::BLACK
                } else {
                    Color::WHITE
                },
            )
        })
    });
}

#[no_mangle]
fn _eadk_main() {
    let mut state: AppState = AppState::Editor;
    let mut board: Board = [false; BOARD_SIZE];

    loop {
        draw_board(&board);
        display::wait_for_vblank();
    }
}
