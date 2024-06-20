// IMPORTS


// CONSTANTS
const ROW_COUNT: usize = 3;
const COLUMN_COUNT: usize = 3;

// MAIN FUNCTION
fn main() {
    let mut board: Vec<Vec<char>> = create_board();
    board[0][0] = 'X';
    board[2][2] = 'O';
    print_board(&board);
    clear_board(&mut board);
    print_board(&board);

}

// FUNCTIONS
fn create_board() -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = Vec::new();
    for _ in 0..ROW_COUNT {
        let row: Vec<char> = vec![' '; COLUMN_COUNT];
        board.push(row);
    }
    return board;
}

fn print_board(board: &Vec<Vec<char>>) {
    let mut count = 1;
    for row_num in 0..ROW_COUNT {
        for column_num in 0..COLUMN_COUNT {
            let current_cell = board[row_num][column_num];
            
            if current_cell == ' ' {
                print!(" {} ", count);
            }
            else {
                print!(" {} ", current_cell);
            }
            count += 1;
        }
        println!();
    }
    println!()
}

fn clear_board(board: &mut Vec<Vec<char>>) {
    for row_num in 0..ROW_COUNT {
        for column_num in 0..COLUMN_COUNT {
            board[row_num][column_num] = ' ';
        }
    }
}
