use clap::Parser;
use colored::Colorize;
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

pub struct Board {
    pub width: u32,
    pub height: u32,
    pub state: Vec<Vec<u32>>,
    pub generation: u64,
    pub alive_cells: u32,
}

#[derive(Parser)]
#[clap(version, about = "Conway's Game of Life")]
pub struct Cli {
    /// The width of the board
    #[arg(short, long, default_value_t = 20)]
    pub width: u32,

    /// The height of the board
    #[arg(short = 't', long, default_value_t = 20)]
    pub height: u32,

    /// File that board state should be read from, leave blank for random board.
    #[arg(short, long)]
    pub file: Option<String>,
}

pub fn initalize_board(width: u32, height: u32) -> Vec<Vec<u32>> {
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

pub fn read_board_from_file(filename: String) -> Board {
    // File result refers to the success state of opening the file
    let file_result = File::open(filename);

    // Only set file if file opened properly
    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Error opening board state file: {:?}", error),
    };
    let reader = BufReader::new(file);

    let mut board: Vec<Vec<u32>> = Vec::new();

    // Loop individually through every line (row) of the state file
    for (_index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!("Error reading line from board state file: {:?}", error),
        };

        // Split the row into a vector of each individual character
        let char_row: Vec<char> = line.chars().collect();
        let mut row: Vec<u32> = Vec::new();
        for char in char_row {
            // Convert char representation of the cell state to u32, make sure its actually an integer
            row.push(
                char.to_digit(10)
                    .expect("File must contain state in form 0 or 1"),
            );
        }

        board.push(row)
    }
    // Width of the board is the same as the length of the first row
    let width: u32 = board[0].len().try_into().unwrap();
    // Height of the board is the same as the length of all the rows
    let height: u32 = board.len().try_into().unwrap();

    let mut game_board = Board {
        width,
        height,
        state: board,
        generation: 1,
        alive_cells: 0,
    };

    game_board.alive_cells = count_alive_cells(&game_board.state);
    return game_board;
}

pub fn clear_terminal_screen() {
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

pub fn render(board: &Vec<Vec<u32>>, width: u32) {
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

pub fn count_alive_cells(board: &[Vec<u32>]) -> u32 {
    let mut alive_cells = 0;

    for row in board {
        for cell in row {
            match cell {
                0 => continue,
                1 => alive_cells += 1,
                _other => panic!("Unexpected cell value"),
            }
        }
    }
    alive_cells
}

pub fn print_cell(iteration: &mut u32, width: u32, color: [u8; 3]) {
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
