use crate::{
    draw, message,
    utils::{flip, str_cord_name_to_num},
};
use rand::{thread_rng, Rng};
use std::{io, ops::Not};

#[derive(Debug)]
pub struct State {
    pub field_width: usize,
    pub field_height: usize,

    pub under_field: Vec<Vec<Cell>>,
    pub mask_field: Vec<Vec<Cell>>,

    pub is_play: bool,

    is_mines_placed: bool,
    mines_count: i32,
    marked_mines_count: i32,
    safe_zone_radius: i32,
}

#[derive(Clone, Debug)]
pub enum Cell {
    Uninitialized,

    // under-field cells
    Empty,
    Booked,
    Number(u8),
    Mine,
    DeactivatedMine,

    // mask-field cells
    Closed,
    Open,
    Question,
    Mark,
}

impl State {
    pub fn new(
        width: usize,
        height: usize,
        mines_count: i32,
        safe_zone_radius: Option<i32>,
    ) -> State {
        let safe_zone_radius = safe_zone_radius.unwrap_or(3);

        if ((width * height) as i32) - (safe_zone_radius * 2 + 1) * (safe_zone_radius * 2 + 1)
            < mines_count
        {
            panic!("To much mines_count for cur field size!")
        }

        let field_init = |state: Cell| -> Vec<Vec<Cell>> {
            (0..height)
                .into_iter()
                .map(|_| {
                    (0..width)
                        .into_iter()
                        .map(|_| state.clone())
                        .collect::<Vec<Cell>>()
                })
                .collect::<Vec<Vec<Cell>>>()
        };

        let under_field = field_init(Cell::Uninitialized);
        let mask_field = field_init(Cell::Closed);

        State {
            field_width: width,
            field_height: height,
            is_mines_placed: false,
            mines_count: mines_count,
            marked_mines_count: 0,
            under_field: under_field,
            mask_field: mask_field,
            safe_zone_radius: safe_zone_radius,
            is_play: true,
        }
    }

    pub fn open(&mut self, x: usize, y: usize) -> Cell {
        self.mask_field[y][x] = Cell::Open;

        if !self.is_mines_placed {
            self.prepare_save_zone(x, y);
            self.place_mines();
            self.place_numbers();
        }

        if matches!(self.under_field[y][x], Cell::Empty | Cell::Number(_)) {
            for i in y..=y + 2 {
                if (i as i32) - 1 < 0 || i > self.field_height {
                    continue;
                }

                for j in x..=x + 2 {
                    if (j as i32) - 1 < 0 || j > self.field_width {
                        continue;
                    }

                    if matches!(self.under_field[y][x], Cell::Number(_))
                        & !matches!(self.under_field[i - 1][j - 1], Cell::Empty)
                    {
                        continue;
                    }

                    if matches!(self.mask_field[i - 1][j - 1], Cell::Closed) {
                        self.open(j - 1, i - 1);
                    }
                }
            }
        }

        return self.under_field[y][x].clone();
    }

    pub fn open_all(&mut self) {
        for i in 0..self.field_height {
            for j in 0..self.field_width {
                self.mask_field[i][j] = Cell::Open;
            }
        }
    }

    pub fn mark(&mut self, x: usize, y: usize) -> Cell {
        if matches!(self.mask_field[y][x], Cell::Closed) {
            self.mask_field[y][x] = Cell::Mark;
            if matches!(self.under_field[y][x], Cell::Mine) {
                self.under_field[y][x] = Cell::DeactivatedMine;
                self.marked_mines_count += 1;
            }
        } else if matches!(self.mask_field[y][x], Cell::Mark) {
            self.mask_field[y][x] = Cell::Closed;
            if matches!(self.under_field[y][x], Cell::DeactivatedMine) {
                self.under_field[y][x] = Cell::Mine;
                self.marked_mines_count -= 1;
            }
        }

        return self.under_field[y][x].clone();
    }

    pub fn question(&mut self, x: usize, y: usize) -> Cell {
        if matches!(self.mask_field[y][x], Cell::Closed) {
            self.mask_field[y][x] = Cell::Question;
        } else if matches!(self.mask_field[y][x], Cell::Question) {
            self.mask_field[y][x] = Cell::Closed;
        }

        return self.under_field[y][x].clone();
    }

    pub fn check_win(&self) -> bool {
        return self.marked_mines_count == self.mines_count;
    }

    fn prepare_save_zone(&mut self, x: usize, y: usize) {
        for i in y as i32 - self.safe_zone_radius..=y as i32 + self.safe_zone_radius {
            if i < 0 || i >= self.field_height as i32 {
                continue;
            }
            for j in x as i32 - self.safe_zone_radius..=y as i32 + self.safe_zone_radius {
                if j < 0 || j >= self.field_height as i32 {
                    continue;
                }
                self.under_field[i as usize][j as usize] = Cell::Booked;
            }
        }
    }

    fn place_mines(&mut self) {
        if self.is_mines_placed {
            return;
        }

        let mut rng = thread_rng();

        let mut placed_mines = 0;

        while placed_mines != self.mines_count {
            let pos: (usize, usize) = (
                rng.gen_range(0..self.field_width),
                rng.gen_range(0..self.field_height),
            );
            if matches!(self.under_field[pos.1][pos.0], Cell::Uninitialized) {
                self.under_field[pos.1][pos.0] = Cell::Mine;
                placed_mines += 1;
            }
        }

        self.is_mines_placed = true;
    }

    fn place_numbers(&mut self) {
        for i in 0..self.field_height {
            for j in 0..self.field_width {
                if matches!(self.under_field[i][j], Cell::Uninitialized | Cell::Booked).not() {
                    continue;
                }

                let mut mines_around = 0;

                for ii in i..=i + 2 {
                    if (ii as i32) - 1 < 0 || ii > self.field_height {
                        continue;
                    }

                    for jj in j..=j + 2 {
                        if (jj as i32) - 1 < 0 || jj > self.field_width {
                            continue;
                        }

                        if matches!(self.under_field[ii - 1][jj - 1], Cell::Mine) {
                            mines_around += 1;
                        }
                    }
                }

                self.under_field[i][j] = if mines_around > 0 {
                    Cell::Number(mines_around)
                } else {
                    Cell::Empty
                };
            }
        }
    }
}

pub fn run() {
    message::show_start();

    let mut state = State::new(26, 15, 60, Some(2));
    draw::draw(&state);

    let mut buffer = String::new();

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut e: Cell = Cell::Empty;

    'main_loop: while state.is_play {
        buffer.clear();

        match io::stdin().read_line(&mut buffer) {
            Ok(_) => (),
            Err(_) => continue,
        };

        let splitted = buffer.split_whitespace();
        if splitted.clone().count() != 3 {
            println!("Invalid arguments count!");
            continue;
        }

        for (idx, token) in splitted.enumerate() {
            match idx {
                // Row
                0 => {
                    match token.parse::<usize>() {
                        Ok(i) => y = i,
                        Err(_) => {
                            println!("Invalid first argument!");
                            continue 'main_loop;
                        }
                    };
                }

                // Column
                1 => {
                    match str_cord_name_to_num(token.to_lowercase()) {
                        Ok(i) => x = i,
                        Err(_) => {
                            println!("Invalid second argument!");
                            continue 'main_loop;
                        }
                    };
                }

                // Action
                2 => {
                    match token.to_lowercase().as_str() {
                        "s" => e = Cell::Open,
                        "?" => e = Cell::Question,
                        "f" => e = Cell::Mark,
                        _ => {
                            println!("Invalid third argument!");
                            continue 'main_loop;
                        }
                    };
                }
                _ => {
                    println!("Invalid input format!");
                    continue 'main_loop;
                }
            }
        }

        if x >= state.field_width || y >= state.field_height {
            println!("Invalid coordinates!");
            continue;
        }

        match e {
            Cell::Open => match state.open(x, y) {
                Cell::Mine => {
                    state.open_all();

                    flip(&state);

                    println!("You lose!");

                    state.is_play = false;
                }
                _ => flip(&state),
            },
            Cell::Mark => {
                state.mark(x, y);
                if state.check_win() {
                    state.open_all();

                    flip(&state);

                    println!("You win!");

                    state.is_play = false;
                } else {
                    flip(&state);
                }
            }
            Cell::Question => {
                state.question(x, y);
                flip(&state);
                ()
            }
            _ => (),
        }
    }
}
