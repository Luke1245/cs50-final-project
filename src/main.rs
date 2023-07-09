use clap::Parser;
use std::{thread, time};

// helpers.rs contains code for all functions and structs that make the game work, but do not contribute to the logic
pub mod helpers;

fn main() {
    // Get the command line arguments from user
    let args = helpers::Cli::parse();

    // Check if file flag is set
    if args.file.is_some() {
        // Load the game board from the file
        let game_board: helpers::Board = helpers::read_board_from_file(args.file.unwrap());

        run_game(game_board)
    } else {
        // Create a random board
        let mut game_board = helpers::Board {
            width: args.width,
            height: args.height,
            state: helpers::initalize_board(args.width, args.height),
            generation: 1,
            alive_cells: 0,
        };
        game_board.alive_cells = helpers::count_alive_cells(&game_board.state);
        run_game(game_board)
    }
}

fn run_game(mut game_board: helpers::Board) {
    while game_board.alive_cells != 0 {
        helpers::render(&game_board.state, game_board.width);
        game_board.alive_cells = helpers::count_alive_cells(&game_board.state);

        thread::sleep(time::Duration::from_secs(1));

        helpers::clear_terminal_screen();

        next_iteration(
            &mut game_board.state,
            game_board.width,
            game_board.height,
            &mut game_board.generation,
        );
    }
    println!(
        "All cells are dead, game has ended on generation: {}",
        game_board.generation
    );
}

fn next_iteration(board: &mut [Vec<u32>], width: u32, height: u32, generation: &mut u64) {
    // Create a copy of the board to compare changes to, so that changes to the board
    // for the new iterationdon't affect the old iteration
    let temporary_board = board.to_owned();

    for row in 0..height {
        for column in 0..width {
            let new_cell_state = compute_cell_state(
                &temporary_board,
                row.try_into().unwrap(),
                column.try_into().unwrap(),
                width,
                height,
            );
            // Edit current ACTUAL board coordinate with new state
            board[row as usize][column as usize] = new_cell_state;
        }
    }

    *generation += 1
}

fn compute_cell_state(
    temporary_board: &[Vec<u32>],
    row: usize,
    column: usize,
    width: u32,
    height: u32,
) -> u32 {
    let mut alive_neighbours = 0;
    // Top left corner is row - 1 column - 1, Top middle corner is row - 1 column + 0 etc. This array allows us to iterate through each corner,
    // instead of using a bunch of if statements and seperate variables for each corner
    let neighbour_postitions: [i32; 3] = [-1, 0, 1];
    let current_cell = temporary_board[row][column];

    for row_mutator in neighbour_postitions {
        for column_mutator in neighbour_postitions {
            // Gets the neighbours position in the board
            let neighbour_row = row as i32 + row_mutator;
            let neighbour_column = column as i32 + column_mutator;

            // Check if neighbours position is outside of the board (neighbour of corner or side)
            if neighbour_row < 0 || neighbour_column < 0 {
                continue;
            }

            // Check if neighbours position is outside of the board (neighbour of corner or side)
            if neighbour_row >= height as i32 || neighbour_column >= width as i32 {
                continue;
            }

            // If this is true than the current neighbour is the actual cell we're computing the state for, so ignore
            if row_mutator == 0 && column_mutator == 0 {
                continue;
            }

            if temporary_board[neighbour_row as usize][neighbour_column as usize] == 1 {
                alive_neighbours += 1;
            } else {
                continue;
            }
        }
    }

    if current_cell == 1 {
        if alive_neighbours <= 1 {
            0
        } else if (2..=3).contains(&alive_neighbours) {
            1
        } else {
            0
        }
    } else if current_cell == 0 {
        if alive_neighbours == 3 {
            1
        } else {
            0
        }
    } else {
        panic!("Unexpected cell value")
    }
}
