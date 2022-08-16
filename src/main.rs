#![no_std]
#![no_main]

pub mod eadk;
use eadk::{display, key, keyboard, timing, Color, Rect, SCREEN_HEIGHT, SCREEN_WIDTH};

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

const CELL_SIZE: u16 = 4;
const LINE_SIZE: usize = (SCREEN_WIDTH / CELL_SIZE) as usize;
const COLUMN_SIZE: usize = (SCREEN_HEIGHT / CELL_SIZE) as usize;
const BOARD_SIZE: usize = LINE_SIZE * COLUMN_SIZE;

type Board = [bool; BOARD_SIZE];
type OnBoard = Vec<(u16, u16), BOARD_SIZE>;

enum AppState {
    Editor,
    Running,
    StepByStep,
}

fn draw_board(on_board: &OnBoard) {
    display::push_rect_uniform(Rect::SCREEN, Color::WHITE);

    on_board.iter().for_each(|(x, y)| {
        display::push_rect_uniform(
            Rect {
                x: *x * CELL_SIZE,
                y: *y * CELL_SIZE,
                width: CELL_SIZE,
                height: CELL_SIZE,
            },
            Color::BLACK,
        )
    });
}

#[no_mangle]
fn _eadk_main() {
    let mut state: AppState = AppState::Editor;
    let mut pointer: (u16, u16) = (LINE_SIZE as u16 / 2, COLUMN_SIZE as u16 / 2);

    let mut board: Board = [false; BOARD_SIZE];
    let mut on_board: OnBoard = Vec::new();

    loop {
        let keyboard_state = keyboard::scan();

        draw_board(&on_board);

        match state {
            AppState::Editor => {
                // Draw the pointer
                display::push_rect_uniform(
                    Rect {
                        x: pointer.0 * CELL_SIZE,
                        y: pointer.1 * CELL_SIZE,
                        width: CELL_SIZE,
                        height: CELL_SIZE,
                    },
                    Color::RED,
                );

                if keyboard_state.key_down(key::EXE) {
                    let current = &mut board[pointer.0 as usize + pointer.1 as usize * LINE_SIZE];
                    match current {
                        false => {
                            on_board.push(pointer).unwrap();
                        }
                        true => {
                            on_board.retain(|p| *p != pointer);
                        }
                    }
                    *current = !*current;
                }

                if keyboard_state.key_down(key::UP) {
                    pointer.1 -= 1;
                } else if keyboard_state.key_down(key::DOWN) {
                    pointer.1 += 1;
                }
                if keyboard_state.key_down(key::LEFT) {
                    pointer.0 -= 1;
                } else if keyboard_state.key_down(key::RIGHT) {
                    pointer.0 += 1;
                }
            }
            AppState::Running => {}
            AppState::StepByStep => {}
        }

        display::wait_for_vblank();
        timing::msleep(100);
    }
}
