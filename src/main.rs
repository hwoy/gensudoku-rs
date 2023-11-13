extern crate sudoku_rs;
use sudoku_rs::prelude::*;

mod def;

mod print_tex;
use print_tex::PrintTex;

extern crate clap;
use clap::{arg, value_parser, ArgAction};

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[cfg(feature = "open64")]
extern "C" {
    fn open(_: *const (), _: i32, _: i32) -> i32;
}

#[cfg(feature = "open64")]
#[no_mangle]
pub extern "C" fn open64(a: *const (), b: i32, c: i32) -> i32 {
    unsafe { open(a, b, c) }
}

fn parse_command_line(
    def_nbseed: sudoku_sys::URND32,
    def_sbid: sudoku_sys::sgt_bid,
    def_nblank: u32,
    def_sd: u32,
    def_nboard: u32,
) -> (
    sudoku_sys::URND32,
    sudoku_sys::sgt_bid,
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
                .value_parser(value_parser!(sudoku_sys::sgt_bid))
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

    let sbid = if let Some(val) = matches.get_one::<sudoku_sys::sgt_bid>("sbid") {
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

    let writer: Box<dyn Write> = if let Some(pathbuf) = filename {
        let writer = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(pathbuf)
            .unwrap();
        Box::new(writer)
    } else {
        let writer = std::io::stdout().lock();
        Box::new(writer)
    };

    print_tex::build_sukoku_iter(nbseed, sbid, nblank, sd, nboard).write_tex(writer)
}
