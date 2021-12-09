fn main() {
    let (numbers, mut boards) = day_4::load_game(false);
    for n in numbers {
        for (idx, board) in boards.iter_mut().enumerate() {
            let coordinates = board.mark_number(n);
            if let Some((row, col)) = coordinates {
                if board.check_for_win(row, col) {
                    let total = board.total();
                    println!("Last number called: {}", n);
                    println!("Sum of winning board: {}", total);
                    println!("Answer: {}", n * total);
                    println!("\nWinning board: {}\n", idx);
                    board.print_board();
                    return;
                }
            }
        }
    }

    println!("No winners?");
}
