use clap::Parser;
use rand::Rng;

#[derive(Parser)]
struct Cli {
    /// The width of the board
    #[arg(short, long)]
    width: u8,

    /// The height of the board
    #[arg(short = 't', long)]
    height: u8,
}

struct Board {
    width: u8,
    height: u8,
    state: Vec<Vec<u8>>,
    generation: u64,
    status: String,
}

fn main() {
    // Get the command line arguments from user
    let args = Cli::parse();

    // Create the initial state of the board
    let game_board = Board {
        width: args.width,
        height: args.height,
        state: initalize_board(args.width, args.height),
        generation: 1,
        status: String::from("alive")
    };

    println!("{:?}", game_board.state)
}

fn initalize_board(width: u8, height: u8) -> Vec<Vec<u8>>  {
    // Board is represented as a vector containing a vector that contains u8s, this allows
    // for a Cartesian coordinate system where each cell can be represented by doing the following: board[y][x]
    let mut board: Vec<Vec<u8>> = Vec::new();

    // First loop through each row in the board
    for _i in 0..height {
        let mut row: Vec<u8> = Vec::new();
        // Second loop through each cell in a row
        for _j in 0..width {
            // Set the cell to either 1 or 0 (alive or dead)
            let cell_state: u8 = rand::thread_rng().gen_range(0..=1);
            row.push(cell_state);
        }
        board.push(row);
    }
    return board
}
