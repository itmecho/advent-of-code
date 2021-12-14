fn main() {
    let (numbers, mut boards) = day_4::load_game();
    let board_count = boards.len();
    let mut winning_boards = vec![];
    let mut last_call = 0;

    'game: for n in &numbers {
        last_call = *n;
        for (idx, board) in boards.iter_mut().enumerate() {
            if winning_boards.contains(&idx) {
                continue;
            }
            let coordinates = board.mark_number(*n);
            if let Some((row, col)) = coordinates {
                if board.check_for_win(row, col) {
                    winning_boards.push(idx);
                    if winning_boards.len() == board_count {
                        break 'game;
                    }
                }
            }
        }
    }

    let last_winner = *winning_boards.last().unwrap();
    let board = boards.get(last_winner).unwrap();
    let total = board.total();

    println!("Last number called: {}", last_call);
    println!("Sum of winning board: {}", total);
    println!("Answer: {}", last_call * total);
    println!("\nWinning board: {}\n", last_winner);
    board.print_board();
}
