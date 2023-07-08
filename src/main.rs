use clap::Parser;
use colored::Colorize;
use rand::Rng;
use std::process::Command;
use std::{thread, time};

#[derive(Parser)]
struct Cli {
    /// The width of the board
    #[arg(short, long)]
    width: u32,

    /// The height of the board
    #[arg(short = 't', long)]
    height: u32,
}

struct Board {
    width: u32,
    height: u32,
    state: Vec<Vec<u32>>,
    generation: u64,
}

fn main() {
    // Get the command line arguments from user
    let args = Cli::parse();

    // Create the initial state of the board
    let mut game_board = Board {
        width: args.width,
        height: args.height,
        state: initalize_board(args.width, args.height),
        generation: 1,
    };

    
    loop {
        render(&game_board.state, game_board.width);

        thread::sleep(time::Duration::from_secs(1));

        clear_terminal_screen();
        
        next_iteration(
            &mut game_board.state,
            game_board.width,
            game_board.height,
            &mut game_board.generation,
        );
    }
}

fn clear_terminal_screen() {
    // Clear on windows
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("Failed to clear screen")
            .wait()
            .expect("Wait failed");
    } else {
        // Linux clear command
        Command::new("clear")
            .spawn()
            .expect("Failed to clear screen")
            .wait()
            .expect("Wait failed");
    };
}

fn initalize_board(width: u32, height: u32) -> Vec<Vec<u32>> {
    // Board is represented as a vector containing a vector that contains u32s, this allows
    // for a Cartesian coordinate system where each cell can be represented by doing the following: board[y][x]
    let mut board: Vec<Vec<u32>> = Vec::new();

    // First loop through each row in the board
    for _i in 0..height {
        let mut row: Vec<u32> = Vec::new();
        // Second loop through each cell in a row
        for _j in 0..width {
            // Set the cell to either 1 or 0 (alive or dead)
            let cell_state: u32 = rand::thread_rng().gen_range(0..=1);
            row.push(cell_state);
        }
        board.push(row);
    }
    board
}

fn render(board: &Vec<Vec<u32>>, width: u32) {
    for row in board {
        let mut iteration: u32 = 1;
        for cell in row {
            match cell {
                // 0 refers to dead cell, print in black
                0 => print_cell(&mut iteration, width, [0, 0, 0]),
                // 1 refers to alive cell, print in white
                1 => print_cell(&mut iteration, width, [255, 255, 255]),
                _other => panic!("Unexpected cell value"),
            }
        }
    }
}

fn print_cell(iteration: &mut u32, width: u32, color: [u8; 3]) {
    // if this is true, the rest of the row still needs to be printed to screen, so we use print!
    // instead of println! to print without a newline
    if *iteration < width {
        // u2588 is a blank block character, printing two of them makes a nice square :]
        print!(
            "{}",
            "\u{2588}\u{2588}".truecolor(color[0], color[1], color[2])
        );
        *iteration += 1;
    }
    // else it is the end of the line, so we can print with a newline
    else {
        println!(
            "{}",
            "\u{2588}\u{2588}".truecolor(color[0], color[1], color[2])
        );
        *iteration += 1;
    }
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
