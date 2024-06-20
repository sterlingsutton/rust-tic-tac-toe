// IMPORTS


// CONSTANTS
const ROW_COUNT: usize = 3;
const COLUMN_COUNT: usize = 3;

// MAIN FUNCTION
fn main() {
    let mut board: Vec<Vec<char>> = create_board(ROW_COUNT, COLUMN_COUNT);
    print_board(board);
}

// FUNCTIONS
fn create_board(row_count: usize, column_count: usize) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = Vec::new();
    for _ in 0..row_count {
        let row: Vec<char> = vec![' '; column_count];
        board.push(row);
    }
    return board;
}

fn print_board(board: Vec<Vec<char>>) {
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
}