# Conway's Game of Life - CS50 Final Project
#### Video Demo: 
#### Description:
This project is an implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in the [Rust](https://rust-lang.org) programming language.

##### The game
Life is a simple zero-player game that simulates the "movement" of cells on a board. The game is governed by a set of simple rules, which are:

1. Any live cell with fewer than two live neighbours dies.
2. Any live cell with two or three live neighbours' lives.
3. Any live cell with more than three live neighbours dies.
4. Any dead cell with exactly three live neighbours becomes a live cell.

##### Why Rust?

I was inspired to do this project when reading [Robert Heaton's blog post](https://robertheaton.com/2018/07/20/project-2-game-of-life/) regarding programming projects. But I felt that a Python implementation would be both too easy and too boring, which led to looking at other possible languages to write in. To pick, I looked at [stackoverflow's developer survey results](https://stackoverflow.blog/2023/06/13/developer-survey-results-are-in/), these show a large interest in Rust, which I have already heard praise for across the internet, so I decided to use that as the language for the project. 

Rust is a difficult language, and a lot of time in this project was spent outside of this repo learning rust via resources such as the [rust book](https://doc.rust-lang.org/book/)

To help, I also wrote an implementation of the game in Python so I could get an idea of how I'd implement the basic structure of the program.

##### Rust V Python

Below are the results of generating the initial state of a 1000x1000 board on both the Python and Rust implementations; although the Python implementation is not featured in this repo, I can say that the concept behind the two is similar.

|        | 1      | 2      | 3      | 4      | 5      | AVG    |
|--------|--------|--------|--------|--------|--------|--------|
| RUST   | 2.55s  | 2.33s  | 2.39s  | 2.40s  | 2.45s  | 2.42s  |
| PYTHON | 34.98s | 32.74s | 33.60s | 33.96s | 32.91s | 33.64s |

##### The program

The program's functionality is split into two files, `main.rs` and `helpers.rs`. 

##### main.rs
`main.rs` contains functions that run the logic behind the game. 

In `main()`, the program checks if it should generate a random board or read a state from a file and then runs the game accordingly. 

In `run_game()`, each step of the game takes place; first, the state of the board is printed to the screen via the `render()` function. Next, the amount of alive cells within the current board state is calculated, this is so that the program can quit if all cells are dead. After that, the program sleeps for an amount of time defined by the constant SLEEP_TIME at the top of the file. The program then clears the screen and generates the next iteration via the `next_iteration()` function.

In `next_iteration()`, the program calculates the next state of the board based on a given board state. A temporary board is created that allows us to gradually write changes to the main board without affecting the alive neighbours of other cells. By looping through each row and column, the new cell state is calculated via the `compute_cell_state()` function. Finally, the new state is written to the board, and the generation is incremented.

Finally, in `compute_cell_state()`, the program calculates the next state of a given cell based on the rules at the top of the program. An array is created that contains the "mutators" to the coordinates of the neighbouring cells. Before retrieving the neighbouring cell status, a check is performed to ensure that the co-ordinate is not outside of the board, which would cause a panic in the program. Another check makes that the current "neighbour" cell is not the cell that is having its state calculated. After retrieving the cell value, the cell's next state is calculated based on the rules of the game and returned from the function.

##### helpers.rs
`helpers.rs` contains functions and structs that run the programs' "backend" but do not hold any logic for the game.

At the top of the file, two structs are defined: `Cli`, which uses the [clap](https://docs.rs/clap/latest/clap/) rust crate to retrieve command line arguments, and `Board`, which holds all the required data regarding the games board.

In `initalize_board()`, a new board is created that randomly selects the values 0 (dead) and 1 (alive) to go in each cell. This is done by looping through each row and creating a vector containing all the cells of that row, which is then pushed to the board vector.

In `read_board_from_file()`, the program takes in a filename and reads a board status from it; each cell is represented by a 0 and 1. Most of this function is error checking to ensure the file exists and can be read, as well as the lines are valid. The logic behind creating the board is similar to the `initalize_board()` function but also checks that the file has lines of matching width. The width, height and number of alive cells are also calculated and put into an instance of the Board struct, which is then returned.

In `clear_terminal_screen()`, a function is defined that allows us to clear both Linux and Windows terminals with their respective clear commands; any errors are also handled.

In `render()` and `print_cell()`, the board state is displayed to the terminal in a recognisable and human-understandable way, rather than 0s and 1s. The value of the cell is checked, and then the program calls `print_cell()` with the colour values of black or white. `print_cell()` also handles when to print with and without a newline; this ensures that the board is the correct width.

In `count_alive_cells()`, the program loops through each cell in the board and count if it is alive or not; the program keeps a running total of the amount alive and returns the value.