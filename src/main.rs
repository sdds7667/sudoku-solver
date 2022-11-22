use enigo::*;
use std::{thread, time};

fn coords_to_macro_cell(row: usize, col: usize) -> usize {
    (row / 3) * 3 + col / 3
}

fn row_contains_from_matrix(matrix: &Vec<Vec<usize>>) -> Vec<bool> {
    let mut res = vec![false; 100];
    for i in 0..=8 {
        for j in 0..=8 {
            if matrix[i][j] != 0 {
                res[i * 9 + matrix[i][j] as usize] = true;
            }
        }
    }
    return res;
}

fn column_contains_from_matrix(matrix: &Vec<Vec<usize>>) -> Vec<bool> {
    let mut res = vec![false; 100];
    for i in 0..=8 {
        for j in 0..=8 {
            if matrix[i][j] != 0 {
                res[matrix[i][j] + j * 9] = true;
            }
        }
    }
    return res;
}

fn macro_cell_contains_from_matrix(matrix: &Vec<Vec<usize>>) -> Vec<bool> {
    let mut res = vec![false; 100];
    for i in 0..=8 {
        for j in 0..=8 {
            if matrix[i][j] != 0 {
                res[9 * coords_to_macro_cell(i, j) + matrix[i][j]] = true;
            }
        }
    }
    return res;
}


fn backtrack(matrix: &mut Vec<Vec<usize>>, rc: &mut Vec<bool>, cc: &mut Vec<bool>, mcc: &mut Vec<bool>, row: usize, col: usize) -> bool {
    if matrix[row][col] != 0 {
        return if col == 8 {
            if row == 8 {
                true
            } else {
                backtrack(matrix, rc, cc, mcc, row + 1, 0)
            }
        } else {
            backtrack(matrix, rc, cc, mcc, row, col + 1)
        };
    }

    for i in 1usize..=9 {
        if !(rc[row * 9 + i] || cc[col * 9 + i] || mcc[coords_to_macro_cell(row, col) * 9 + i]) {
            rc[row * 9 + i] = true;
            cc[col * 9 + i] = true;
            mcc[coords_to_macro_cell(row, col) * 9 + i] = true;
            matrix[row][col] = i;
            if col == 8 {
                if row == 8 {
                    return true;
                } else {
                    if backtrack(matrix, rc, cc, mcc, row + 1, 0) {
                        return true;
                    }
                }
            } else {
                if backtrack(matrix, rc, cc, mcc, row, col + 1) {
                    return true;
                }
            }
            rc[row * 9 + i] = false;
            cc[col * 9 + i] = false;
            mcc[coords_to_macro_cell(row, col) * 9 + i] = false;
            matrix[row][col] = 0;
        }
    }
    false
}

fn click(enigo: &mut Enigo, row: usize, col: usize) {
    println!("Clicking on {row}, {col}");
    let square_width :i32= 55;
    let square_height = 55;
    let duration= time::Duration::from_millis(50);
    enigo.mouse_move_to((1920 / 2 - square_width * 4 + ((col as i32) * square_width)), 1080 / 2 - square_height / 2 - square_height * 5 + square_height * (row as i32));
    thread::sleep(time::Duration::from_millis(10));
    enigo.mouse_click(MouseButton::Left);
    thread::sleep(duration)
}

fn number_click(enigo: &mut Enigo, number: usize) {
    println!("Clicking on number {number}!");
    let square_width = 55;
    let ten_millis = time::Duration::from_millis(50);
    enigo.mouse_move_to(1920 / 2 - square_width * 4+ ((number - 1) as i32) * square_width, 1080 / 2 + 325);
    thread::sleep(time::Duration::from_millis(10));
    enigo.mouse_click(MouseButton::Left);
    thread::sleep(ten_millis);
}

fn main() {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(1920/2, 1080/2);
    enigo.mouse_click(MouseButton::Left);
    let mut matrix: Vec<Vec<usize>> = vec![
        vec![0, 0, 0, 0, 3, 0, 0, 0, 0],
        vec![8, 0, 0, 0, 0, 9, 0, 5, 7],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![0, 0, 0, 9, 0, 0, 0, 0, 0],
        vec![0, 6, 3, 1, 0, 0, 4, 2, 0],
        vec![0, 2, 0, 0, 7, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 8, 0, 0, 0],
        vec![0, 5, 0, 0, 0, 0, 8, 0, 0],
        vec![2, 0, 4, 0, 0, 0, 0, 9, 6],
    ];


    let mut clone = matrix.clone();

    let mut row_contains = row_contains_from_matrix(&matrix);
    let mut column_contains = column_contains_from_matrix(&matrix);
    let mut macro_cell_contains = macro_cell_contains_from_matrix(&matrix);

    println!("{:?}", backtrack(&mut matrix, &mut row_contains, &mut column_contains, &mut macro_cell_contains, 0, 0));

    for i in 0usize..=8 {
        for j in 0usize..=8 {
            if clone[i][j] == 0 {
                click(&mut enigo, i, j);
                number_click(&mut enigo, matrix[i][j]);
            }
            print!("{} ", matrix[i][j]);
        }
        println!();
    }




    println!("Hello, world!");
}
