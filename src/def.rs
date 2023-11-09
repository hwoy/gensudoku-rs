pub const S_SQR: u32 = sudoku_rs::sudoku_sys::S_SQR;
pub const NBLANK: u32 = (S_SQR * S_SQR * 60) / 100;
pub const SD: u32 = (S_SQR * S_SQR * 5) / 100;
pub const NBOARD: u32 = 1;

pub const HEAD_TEX: &'static str = r##"\documentclass[a4paper]{article}
\usepackage{geometry}
\usepackage{sudoku}
\begin{document}

"##;

pub const TAIL_TEX: &'static str = r##"

\end{document}
"##;

pub const HEAD_SUDOKU_TEX: &'static str = r##"\setlength\sudokusize{15cm}
\begin{sudoku}"##;

pub const TAIL_SUDOKU_TEX: &'static str = r##"\end{sudoku}"##;

extern crate sudoku_rs;
use sudoku_rs::prelude::*;

use std::io::Write;

fn print_sudoku_tex(
    writable_object: &mut impl Write,
    sudoku: &sudoku_sys::sgs_game,
    seed: sudoku_sys::URND32,
    bid: sudoku_sys::sgt_bid,
    n: u32,
) -> std::io::Result<()> {
    writable_object.write_fmt(format_args!(
        r##"\noindent \verb|SEED = {}, BID = {}, N = {}, NUMBLANK = {}| \newline"##,
        seed, bid, n, sudoku.numblank
    ))?;

    writable_object.write_fmt(format_args!("\n{}\n", HEAD_SUDOKU_TEX))?;

    for e in sudoku.board_unit().iter() {
        for value in e.iter().map(|unit| unit.value) {
            writable_object.write_fmt(format_args!(
                "|{}",
                if value != 0 {
                    value.to_string()
                } else {
                    " ".to_string()
                }
            ))?;
        }

        writable_object.write_fmt(format_args!("|.\n"))?;
    }

    writable_object.write_fmt(format_args!("{}\n", TAIL_SUDOKU_TEX))
}

use std::iter::Iterator;

type SudokuIteratorItem = (
    sudoku_sys::sgs_game,
    sudoku_sys::URND32,
    sudoku_sys::sgt_bid,
    u32,
);

pub struct SudokuTex<I>(I,u32);

pub fn build_sudoku_iter<I:Iterator<Item = SudokuIteratorItem>>(
    nbseed: sudoku_sys::URND32,
    sbid: sudoku_sys::sgt_bid,
    nblank: u32,
    sd: u32,
    nboard: u32,
) -> Sudoku<I>(I,u32) {
	
    let iter = (0..nboard).map(move |n| {
        (
            sudoku_rs::Builder::new()
                .seed(nbseed + n)
                .setbid(sbid + n)
                .setnblank(nblank)
                .build()
                .to_sudoku_rnd(sd),
            nbseed + n,
            sbid + n,
            n,
        )
    });
	SudokuTex(iter,nboard)
}

pub trait PrintTex {
    fn print_tex(self, writable_object: impl Write, nboard: u32) -> std::io::Result<()>;
}

impl<I> PrintTex for I
where
    I: Iterator<Item = SudokuIteratorItem>,
{
    fn print_tex(self, writable_object: impl Write, nboard: u32) -> std::io::Result<()> {
        let mut writable_object = writable_object;
        writable_object.write_fmt(format_args!("{}\n", HEAD_TEX))?;

        for (sudoku, seed, bid, n) in self {
            print_sudoku_tex(&mut writable_object, &sudoku, seed, bid, n)?;

            if n + 1 < nboard {
                writable_object.write_fmt(format_args!("\\newpage\n\n"))?;
            }
        }

        writable_object.write_fmt(format_args!("{}\n", TAIL_TEX))
    }
}
