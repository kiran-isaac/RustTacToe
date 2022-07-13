pub type Board = [[char; 3]; 3];

pub fn check_board(b: Board, current: char) -> bool {
    for i in 0..3 {
        if b[0][i] == current && b[1][i] == current && b[2][i] == current ||
            b[i][0] == current && b[i][1] == current && b[i][2] == current {
            return true
        }
    }

    if b[0][0] == current && b[1][1] == current && b[2][2] == current ||
        b[2][0] == current && b[1][1] == current && b[0][2] == current {
        return true;
    }

    false
}

pub fn print_board(b: Board) {
    println!("     |     |");
    println!("  {}  |  {}  |  {}", b[0][0], b[0][1], b[0][2]);
    println!("_____|_____|_____");

    println!("     |     |");
    println!("  {}  |  {}  |  {}", b[1][0], b[1][1], b[1][2]);
    println!("_____|_____|_____");

    println!("     |     |");
    println!("  {}  |  {}  |  {}", b[2][0], b[2][1], b[2][2]);
    println!("     |     |");
}