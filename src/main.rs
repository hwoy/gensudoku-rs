extern crate sudoku_rs;
use sudoku_rs::prelude::*;
mod def;

extern crate clap;
use clap::{arg, value_parser, ArgAction};

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

fn parse_command_line(
    def_nbseed: sudoku_sys::URND32,
    def_sbid: sudoku_sys::URND32,
    def_nblank: u32,
    def_sd: u32,
    def_nboard: u32,
) -> (sudoku_sys::URND32, sudoku_sys::URND32, u32, u32, u32) {
    let matches = clap::Command::new("gensudoku-rs")
        .arg(
            arg!(--nbseed <NBSEED> "A seed for RNG")
                .required(false)
                .value_parser(value_parser!(sudoku_sys::URND32))
                .action(ArgAction::Set),
        )
        .arg(
            arg!(--sbid <SBID> "A board ID")
                .value_parser(value_parser!(sudoku_sys::URND32))
                .action(ArgAction::Set),
        )
        .arg(
            arg!(--nblank <NBLANK> "Mean of number of blank of sudoku board")
                .value_parser(value_parser!(u32))
                .action(ArgAction::Set),
        )
        .arg(
            arg!(--sd <SD> "Standard Deviation of nblank")
                .value_parser(value_parser!(u32))
                .action(ArgAction::Set),
        )
        .arg(
            arg!(--nboard <NBOARD> "Number of sudoku board")
                .value_parser(value_parser!(u32))
                .action(ArgAction::Set),
        )
        .get_matches();

    let nbseed = if let Some(val) = matches.get_one::<sudoku_sys::URND32>("nbseed") {
        *val
    } else if matches.contains_id("nbseed") {
        panic!("Parsing nbseed!!");
    } else {
        def_nbseed
    };

    let sbid = if let Some(val) = matches.get_one::<sudoku_sys::URND32>("sbid") {
        *val
    } else if matches.contains_id("sbid") {
        panic!("Parsing sbid!!");
    } else {
        def_sbid
    };

    let nblank = if let Some(val) = matches.get_one::<sudoku_sys::URND32>("nblank") {
        *val
    } else if matches.contains_id("nblank") {
        panic!("Parsing nblank!!");
    } else {
        def_nblank
    };

    let sd = if let Some(val) = matches.get_one::<sudoku_sys::URND32>("sd") {
        *val
    } else if matches.contains_id("sd") {
        panic!("Parsing sd!!");
    } else {
        def_sd
    };

    let nboard = if let Some(val) = matches.get_one::<sudoku_sys::URND32>("nboard") {
        *val
    } else if matches.contains_id("nboard") {
        panic!("Parsing nboard!!");
    } else {
        def_nboard
    };

    (nbseed, sbid, nblank, sd, nboard)
}

fn main() -> std::io::Result<()> {
    let entropy_number = sudoku_rs::seed_from_entropy();

    let (nbseed, sbid, nblank, sd, nboard) = parse_command_line(
        entropy_number,
        entropy_number,
        def::NBLANK,
        def::SD,
        def::NBOARD,
    );

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
