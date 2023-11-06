extern crate sudoku_rs;
use sudoku_rs::prelude::*;
mod def;

fn print_board_tex(game: &mut sudoku_sys::sgs_game, bid: sudoku_sys::URND32, sd: u32, nboard: u32) {
    println!("{}", def::HEAD_TEX);

    for n in 0..nboard {
        game.seed(bid + n);
        game.setbid(bid + n);
        game.createsudoku_rnd(sd);

        println!("{}", def::HEAD_SUDOKU_TEX);

        for y in 0..sudoku_sys::S_SQR {
            for x in 0..sudoku_sys::S_SQR {
                let val = game.getvalue(x, y);
                print!(
                    "|{}",
                    if val != 0 {
                        val.to_string()
                    } else {
                        " ".to_string()
                    }
                );
            }
            println!("|.");
        }

        println!("{}", def::TAIL_SUDOKU_TEX);

        if n + 1 < nboard {
            println!("\\newpage\n");
        }
    }

    println!("{}", def::TAIL_TEX);
}

fn main() {
    let bid = sudoku_rs::seed_from_entropy();
    let mut game = sudoku_sys::sgs_game::new(0, def::NBLANK);
    print_board_tex(&mut game, bid, def::SD, 4);
}
