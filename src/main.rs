extern crate sudoku_rs;
use sudoku_rs::prelude::*;
mod def;

extern crate clap;
use clap::{arg, value_parser, ArgAction};

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn print_board_tex(
    mut writable_object: impl Write,
    nbseed: sudoku_sys::URND32,
    sbid: sudoku_sys::URND32,
    nblank: u32,
    sd: u32,
    nboard: u32,
) -> std::io::Result<()> {
    writable_object.write_fmt(format_args!("{}\n", def::HEAD_TEX))?;

    for n in 0..nboard {
        let sudoku = sudoku_rs::Builder::new()
            .seed(nbseed + n)
            .setbid(sbid + n)
            .setnblank(nblank)
            .build()
            .to_sudoku_rnd(sd);

        writable_object.write_fmt(format_args!(
            r##"\noindent \verb|N_BLANKSEED = {}, SBID = {}, N = {}, SN_BLANK = {}, SD = {}| \newline "##,
            nbseed + n,
            sbid + n,
            n,
            sudoku.numblank,
			sd
        ))?;

        writable_object.write_fmt(format_args!("{}\n", def::HEAD_SUDOKU_TEX))?;

        for y in 0..sudoku_sys::S_SQR {
            for x in 0..sudoku_sys::S_SQR {
                let val = sudoku.getvalue(x, y);

                writable_object.write_fmt(format_args!(
                    "|{}",
                    if val != 0 {
                        val.to_string()
                    } else {
                        " ".to_string()
                    }
                ))?;
            }

            writable_object.write_fmt(format_args!("|.\n"))?;
        }

        writable_object.write_fmt(format_args!("{}\n", def::TAIL_SUDOKU_TEX))?;

        if n + 1 < nboard {
            writable_object.write_fmt(format_args!("\\newpage\n\n"))?;
        }
    }

    writable_object.write_fmt(format_args!("{}\n", def::TAIL_TEX))?;

    Ok(())
}

fn parse_command_line(
    def_nbseed: sudoku_sys::URND32,
    def_sbid: sudoku_sys::URND32,
    def_nblank: u32,
    def_sd: u32,
    def_nboard: u32,
) -> (
    sudoku_sys::URND32,
    sudoku_sys::URND32,
    u32,
    u32,
    u32,
    Option<PathBuf>,
) {
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
        .arg(
            arg!(--file <FILE> "Out put file")
                .value_parser(value_parser!(PathBuf))
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

    let nblank = if let Some(val) = matches.get_one::<u32>("nblank") {
        *val
    } else if matches.contains_id("nblank") {
        panic!("Parsing nblank!!");
    } else {
        def_nblank
    };

    let sd = if let Some(val) = matches.get_one::<u32>("sd") {
        *val
    } else if matches.contains_id("sd") {
        panic!("Parsing sd!!");
    } else {
        def_sd
    };

    let nboard = if let Some(val) = matches.get_one::<u32>("nboard") {
        *val
    } else if matches.contains_id("nboard") {
        panic!("Parsing nboard!!");
    } else {
        def_nboard
    };

    let file: Option<PathBuf> = if let Some(val) = matches.get_one::<PathBuf>("file") {
        Some(val.clone())
    } else if matches.contains_id("file") {
        panic!("Parsing file!!");
    } else {
        None
    };

    (nbseed, sbid, nblank, sd, nboard, file)
}

fn main() -> std::io::Result<()> {
    let entropy_number = sudoku_rs::seed_from_entropy();

    let (nbseed, sbid, nblank, sd, nboard, filename) = parse_command_line(
        entropy_number,
        entropy_number,
        def::NBLANK,
        def::SD,
        def::NBOARD,
    );

    if let Some(pathbuf) = filename {
        let writable_object = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(pathbuf)
            .unwrap();
        print_board_tex(writable_object, nbseed, sbid, nblank, sd, nboard)?;
    } else {
        let writable_object = std::io::stdout().lock();
        print_board_tex(writable_object, nbseed, sbid, nblank, sd, nboard)?;
    }

    Ok(())
}
