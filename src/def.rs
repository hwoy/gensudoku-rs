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

pub struct SudokuIter<I>
where
    I: Iterator<Item = SudokuIteratorItem>,
{
    sudoku_iter: I,
    nboard: u32,
}

impl<I> SudokuIter<I>
where
    I: Iterator<Item = SudokuIteratorItem>,
{
    pub fn from(sudoku_iter: I, nboard: u32) -> Self {
        Self {
            sudoku_iter: sudoku_iter,
            nboard: nboard,
        }
    }
}

pub fn build_sukoku_iter(
    nbseed: sudoku_sys::URND32,
    sbid: sudoku_sys::sgt_bid,
    nblank: u32,
    sd: u32,
    nboard: u32,
) -> SudokuIter<impl Iterator<Item = SudokuIteratorItem>> {
    let sudoku_iter = (0..nboard).map(move |n| {
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
    SudokuIter::from(sudoku_iter, nboard)
}

pub trait PrintTex {
    fn write_tex(self, writable_object: impl Write) -> std::io::Result<()>;
}

impl<I> PrintTex for SudokuIter<I>
where
    I: Iterator<Item = SudokuIteratorItem>,
{
    fn write_tex(self, writable_object: impl Write) -> std::io::Result<()> {
        let mut writable_object = writable_object;
        writable_object.write_fmt(format_args!("{}\n", HEAD_TEX))?;

        let Self {
            sudoku_iter,
            nboard,
        } = self;

        for (sudoku, seed, bid, n) in sudoku_iter {
            print_sudoku_tex(&mut writable_object, &sudoku, seed, bid, n)?;

            if n + 1 < nboard {
                writable_object.write_fmt(format_args!("\\newpage\n\n"))?;
            }
        }

        writable_object.write_fmt(format_args!("{}\n", TAIL_TEX))
    }
}
