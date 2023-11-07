extern crate sudoku_rs;
use sudoku_rs::prelude::*;
mod def;

use std::io::Write;

fn print_board_tex(
    mut writer: impl Write,
    game: &mut sudoku_sys::sgs_game,
    nbseed: sudoku_sys::URND32,
    sbid: sudoku_sys::URND32,
    sd: u32,
    nboard: u32,
) -> std::io::Result<()> {
    writer.write_fmt(format_args!("{}\n", def::HEAD_TEX))?;

    for n in 0..nboard {
        game.seed(nbseed + n);
        game.setbid(sbid + n);
        game.createsudoku_rnd(sd);

        writer.write_fmt(format_args!(
            r##"\noindent \verb|N_BLANKSEED = {}, SBID = {}| \verb|N = {}, SN_BLANK = {}, SD = {}| \newline "##,
            nbseed + n,
            sbid + n,
            n,
            game.numblank,
			sd
        ))?;

        writer.write_fmt(format_args!("{}\n", def::HEAD_SUDOKU_TEX))?;

        for y in 0..sudoku_sys::S_SQR {
            for x in 0..sudoku_sys::S_SQR {
                let val = game.getvalue(x, y);

                writer.write_fmt(format_args!(
                    "|{}",
                    if val != 0 {
                        val.to_string()
                    } else {
                        " ".to_string()
                    }
                ))?;
            }

            writer.write_fmt(format_args!("|.\n"))?;
        }

        writer.write_fmt(format_args!("{}\n", def::TAIL_SUDOKU_TEX))?;

        if n + 1 < nboard {
            writer.write_fmt(format_args!("\\newpage\n\n"))?;
        }
    }

    writer.write_fmt(format_args!("{}\n", def::TAIL_TEX))?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let nbseed = sudoku_rs::seed_from_entropy();
    let sbid = nbseed;
    let nblank = def::NBLANK;
    let sd = def::SD;
    let nboard = def::NBOARD;

    let mut game = sudoku_sys::sgs_game::new(0, nblank);
    print_board_tex(
        std::io::stdout().lock(),
        &mut game,
        nbseed,
        sbid,
        sd,
        nboard,
    )?;

    Ok(())
}
