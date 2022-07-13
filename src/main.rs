use std::io;
use std::process::exit;
use crate::board_ops::{check_board, print_board};

mod board_ops;

fn str_to_move(pos: &str) -> i32 {
    match pos.trim() {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        _ => 0
    }
}

fn main() {
    let mut board = [[' ' as char; 3]; 3];

    let mut turn = 'x';

    loop {
        let mut x: i32;
        let mut y: i32;

        loop {
            print_board(board);

            println!("Player {} please enter your move in the form x,y", turn);
            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(e) => continue
            };

            let pos : Vec<&str> = input.split(",").collect();
            if pos.len() != 2 {
                continue
            }

            x = str_to_move(pos[0]);
            y = str_to_move(pos[1]);

            if x == 0 || y == 0 {
                println!("Invalid Coords");
                continue
            }

            if board[(3 - y) as usize][(x - 1) as usize] != ' ' {
                println!("This space is taken!");
                continue
            }

            println!("x: {}, y: {}", x, y);
            break
        }

        board[(3 - y) as usize][(x - 1) as usize] = turn;

        if check_board(board, turn) {
            println!("Player {} wins!", turn);
            exit(0);
        }

        turn = if turn == 'x' { 'o' } else { 'x' };
    }
}