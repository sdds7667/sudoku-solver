# Sudoku Solver in Rust - README

This project is a Sudoku solver implemented in Rust that automates the process of solving a Sudoku puzzle by clicking on the right squares. The solver utilizes a backtracking algorithm to find the solution for any given Sudoku puzzle.

## Features

- **Automated Puzzle Solving**: The Sudoku solver is designed to automatically click on the correct squares in order to complete the puzzle. It intelligently determines the possible numbers for each empty square and iteratively fills them until a valid solution is found.

- **Backtracking Algorithm**: The solver employs a backtracking algorithm, which is a brute-force technique that systematically tries different combinations of numbers until a valid solution is found. If a chosen number leads to an invalid solution, it backtracks and tries a different number.

- **User-Friendly Interface**: The Sudoku solver provides a user-friendly interface that allows you to input the puzzle either manually or through a file. It then displays the step-by-step process of solving the puzzle, highlighting the clicked squares.
