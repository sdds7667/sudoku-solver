use enigo::*;


pub struct Delays {
    before_click: u64,
    after_click: u64,
    before_move: u64,
    after_move: u64,
}

impl Delays {
    fn new(before_click: u64, after_click: u64, before_move: u64, after_move: u64) -> Delays {
        Delays {
            before_click,
            after_click,
            before_move,
            after_move,
        }
    }

    pub(crate) fn default() -> Delays {
        Delays {
            before_click: 10,
            after_click: 50,
            before_move: 10,
            after_move: 10,
        }
    }
}

pub struct ButtonCoords {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl ButtonCoords {
    pub(crate) fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        ButtonCoords {
            left,
            top,
            right,
            bottom,
        }
    }
}


pub struct Environment {
    cell0: ButtonCoords,
    number0: ButtonCoords,
    enigo: Enigo,
    delays: Delays,

}

impl Environment {
    pub(crate) fn new(cell0: ButtonCoords, number0: ButtonCoords,
                      delays: Delays) -> Environment {
        Environment {
            cell0,
            number0,
            enigo: Enigo::new(),
            delays,
        }
    }

    pub(crate) fn move_to_cell(&mut self, row: usize, col: usize) {
        let cell_width = self.cell0.right - self.cell0.left;
        let cell_height = self.cell0.bottom - self.cell0.top;

        let cell_center_x = self.cell0.left + cell_width * col as i32 + cell_width / 2;
        let cell_center_y = self.cell0.top + cell_height * row as i32 + cell_height / 2;

        self.enigo.mouse_move_to(cell_center_x, cell_center_y);
    }

    pub(crate) fn click_cell(&mut self, row: usize, col: usize) {
        self.move_to_cell(row, col);
        std::thread::sleep(std::time::Duration::from_millis(self.delays.before_click));
        self.enigo.mouse_click(MouseButton::Left);
        std::thread::sleep(std::time::Duration::from_millis(self.delays.after_click));
    }

    pub(crate) fn move_to_number(&mut self, number: usize) {
        let number_width = self.number0.right - self.number0.left;
        let number_height = self.number0.bottom - self.number0.top;

        let number_center_x = self.number0.left + number_width * (number - 1) as i32 + number_width / 2;
        let number_center_y = self.number0.top + number_height / 2;

        self.enigo.mouse_move_to(number_center_x, number_center_y);
    }

    pub(crate) fn click_number(&mut self, number: usize) {
        self.move_to_number(number);
        std::thread::sleep(std::time::Duration::from_millis(self.delays.before_click));
        self.enigo.mouse_click(MouseButton::Left);
        std::thread::sleep(std::time::Duration::from_millis(self.delays.after_click));
    }

    pub(crate) fn visually_inspect_sudoku_board(&mut self) {
        for i in 0..=8 {
            for j in 0..=8 {
                self.move_to_cell(i, j);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    pub(crate) fn visually_inspect_number_buttons(&mut self) {
        for i in 1..=9 {
            self.move_to_number(i);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}