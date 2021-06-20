use std::io::{self, BufRead, Lines, StdinLock, Write};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut board = engine::Board::new();

    loop {
        println!();
        print_board(&board);

        // Compress
        board.compress(get_direction(&mut lines));

        // Next turn
        match board.next_turn() {
            engine::BoardState::Ok => {}
            engine::BoardState::GameOverLose => {
                println!("YOU LOSE!");
                return;
            }
            engine::BoardState::GameOverWin => {
                println!("YOU WIN!");
                return;
            }
        }
    }
}

fn get_direction(lines: &mut Lines<StdinLock>) -> engine::Direction {
    loop {
        print!("w(up) / a (left) / s (down) / d (right) ? ");
        io::stdout().flush().unwrap();

        match lines.next().unwrap().unwrap().as_str() {
            "w" => return engine::Direction::Up,
            "a" => return engine::Direction::Left,
            "s" => return engine::Direction::Down,
            "d" => return engine::Direction::Right,
            _ => {}
        }
    }
}

fn print_board(board: &engine::Board) {
    let blocks = board.get_blocks();
    println!("---------------------");
    for row_index in 0..4 {
        println!(
            "|{}|{}|{}|{}|",
            WrappedBlock(blocks[row_index * 4]),
            WrappedBlock(blocks[row_index * 4 + 1]),
            WrappedBlock(blocks[row_index * 4 + 2]),
            WrappedBlock(blocks[row_index * 4 + 3])
        );
        println!("---------------------");
    }
}

struct WrappedBlock(Option<u16>);

impl std::fmt::Display for WrappedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(value) => write!(f, "{:^4}", value),
            None => write!(f, "    "),
        }
    }
}
