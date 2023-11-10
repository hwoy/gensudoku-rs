extern crate sudoku_rs;

pub const S_SQR: u32 = sudoku_rs::sudoku_sys::S_SQR;
pub const NBLANK: u32 = (S_SQR * S_SQR * 60) / 100;
pub const SD: u32 = (S_SQR * S_SQR * 5) / 100;
pub const NBOARD: u32 = 1;
