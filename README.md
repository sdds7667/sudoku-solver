# Sudoku Solver in Rust - README

This project is a Sudoku solver implemented in Rust that automates the process of solving a Sudoku puzzle by clicking on
the right squares. The solver utilizes a backtracking algorithm to find the solution for any given Sudoku puzzle.

## Features

- **Automated Puzzle Solving**: The Sudoku solver is designed to automatically click on the correct squares in order to
  complete the puzzle. It intelligently determines the possible numbers for each empty square and iteratively fills them
  until a valid solution is found.

- **Backtracking Algorithm**: The solver employs a backtracking algorithm, which is a brute-force technique that
  systematically tries different combinations of numbers until a valid solution is found. If a chosen number leads to an
  invalid solution, it backtracks and tries a different number.

- **User-Friendly Interface**: The Sudoku solver provides a user-friendly interface that allows you to input the puzzle
  either manually or through a file. It then displays the step-by-step process of solving the puzzle, highlighting the
  clicked squares.

## Usage

1. Clone the repository:

```bash
git clone git@github.com:sdds7667/sudoku-solver.git
```
2. Navigate to the project directory:
3. Run the following command to build the project:

```bash
cargo build --release
```
4. Configure the coordinates of the first cell from the sudoku grid,
   by replacing in the file `src/main.rs`, in the function `main()`:

```rust
let cell0 = ButtonCoords::new(735, 176, 791, 232);
let number0 = ButtonCoords::new(733, 778, 791, 846);
```

5. Type out the sudoku numbers in place of

```rust
let mut matrix: Vec<Vec<usize> > = vec![
```

Leave 0's for the empty cells.

6. Run the program!

For the extreme level sudoku, the program takes about 10-20s to solve the puzzle.

## Demo [Takes about 20s of backtracking]
![Sudoku Solver](/demo.gif "Sudoku Solver")
