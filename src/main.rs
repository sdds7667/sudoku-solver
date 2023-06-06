mod environment_control;

use enigo::*;
use std::{thread, time};
use environment_control::*;


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
fn move_mouse_to_cell(enigo: &mut Enigo, row: usize, col: usize) {


}

fn click(enigo: &mut Enigo, row: usize, col: usize) {
    let board_start_x: i32 = 735;
    let board_start_y: i32 = 176;
    let first_cell_border_right: i32 = 791;
    let first_cell_border_bottom: i32 = 232;

    println!("Clicking on {row}, {col}");
    let square_width :i32= first_cell_border_right- board_start_x;
    let square_height = first_cell_border_bottom - board_start_y;
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
    let cell0 = ButtonCoords::new(735, 176, 791, 232);
    let number0 = ButtonCoords::new(733, 778, 791, 846);

    let mut env = Environment::new(cell0, number0, Delays::default());
    env.visually_inspect_number_buttons();

    let mut matrix: Vec<Vec<usize>> = vec![
        vec![0, 3, 0, 0, 0, 7, 4, 0, 0],
        vec![0, 6, 9, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 4, 0, 0, 9, 6],
        vec![8, 0, 0, 0, 0, 0, 0, 0, 5],
        vec![0, 0, 0, 0, 0, 2, 0, 3, 0],
        vec![0, 0, 0, 9, 5, 0, 0, 0, 0],
        vec![7, 0, 0, 0, 0, 0, 8, 0, 0],
        vec![0, 0, 0, 6, 0, 0, 0, 0, 0],
    ];


    let mut clone = matrix.clone();

    let mut row_contains = row_contains_from_matrix(&matrix);
    let mut column_contains = column_contains_from_matrix(&matrix);
    let mut macro_cell_contains = macro_cell_contains_from_matrix(&matrix);

    println!("{:?}", backtrack(&mut matrix, &mut row_contains, &mut column_contains, &mut macro_cell_contains, 0, 0));

    for i in 0usize..=8 {
        for j in 0usize..=8 {
            if clone[i][j] == 0 {
                env.click_cell(i, j);
                env.click_number(matrix[i][j]);
            }
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
}
