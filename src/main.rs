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
const LINE_SIZE: u16 = SCREEN_WIDTH / CELL_SIZE;
const COLUMN_SIZE: u16 = SCREEN_HEIGHT / CELL_SIZE;
const BOARD_SIZE: usize = LINE_SIZE as usize * COLUMN_SIZE as usize;

type Board = [[bool; COLUMN_SIZE as usize]; LINE_SIZE as usize];
type OnBoard<T> = Vec<(T, T), BOARD_SIZE>;

enum AppState {
    Editor,
    Running,
    StepByStep,
}

fn draw_board(on_board: &OnBoard<u16>) {
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

fn run_cell(board: &Board, cell: (usize, usize)) -> Option<bool> {
    todo!();
}

fn run_once(board: &mut Board) {
    let mut dead: OnBoard<usize> = Vec::new();
    let mut born: OnBoard<usize> = Vec::new();

    board.iter().enumerate().for_each(|(x, col)| {
        col.iter()
            .enumerate()
            .for_each(|(y, _)| match run_cell(&board, (x, y)) {
                Some(true) => born.push((x, y)).unwrap(),
                Some(false) => dead.push((x, y)).unwrap(),
                _ => {}
            });
    });

    dead.into_iter().for_each(|(x, y)| board[x][y] = false);
    born.into_iter().for_each(|(x, y)| board[x][y] = true);
}

#[no_mangle]
fn _eadk_main() {
    let mut state: AppState = AppState::Editor;
    let mut pointer: (u16, u16) = (LINE_SIZE / 2, COLUMN_SIZE / 2);

    let mut board: Board = [[false; COLUMN_SIZE as usize]; LINE_SIZE as usize];

    loop {
        let keyboard_state = keyboard::scan();

        draw_board(
            &board
                .iter()
                .enumerate()
                .flat_map(|(x, col)| {
                    col.into_iter().enumerate().filter_map(move |(y, b)| {
                        if *b {
                            Some((x as u16, y as u16))
                        } else {
                            None
                        }
                    })
                })
                .collect(),
        );

        if keyboard_state.key_down(key::XNT) {
            state = AppState::Editor;
        } else if keyboard_state.key_down(key::VAR) {
            state = AppState::Running;
        } else if keyboard_state.key_down(key::TOOLBOX) {
            state = AppState::StepByStep;
        }

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
                    let current = &mut board[pointer.0 as usize][pointer.1 as usize];
                    *current = !*current;
                }

                if keyboard_state.key_down(key::UP) && pointer.1 > 0 {
                    pointer.1 -= 1;
                } else if keyboard_state.key_down(key::DOWN) && pointer.1 < COLUMN_SIZE - 1 {
                    pointer.1 += 1;
                }
                if keyboard_state.key_down(key::LEFT) && pointer.0 > 0 {
                    pointer.0 -= 1;
                } else if keyboard_state.key_down(key::RIGHT) && pointer.0 < LINE_SIZE - 1 {
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
