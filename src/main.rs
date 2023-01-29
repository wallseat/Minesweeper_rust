pub mod colors {
    use crossterm::style::Color;

    pub const GREY: Color = Color::Rgb {
        r: 50,
        g: 50,
        b: 50,
    };
    pub const GREEN: Color = Color::Green;
    pub const RUST_ORANGE: Color = Color::Rgb {
        r: 196,
        g: 85,
        b: 8,
    };
    pub const YELLOW: Color = Color::Yellow;
    pub const RED: Color = Color::Red;
    pub const BLUE: Color = Color::Rgb {
        r: 0,
        g: 150,
        b: 255,
    };
    pub const DARK_BLUE: Color = Color::Rgb {
        r: 0,
        g: 71,
        b: 171,
    };
    pub const DARK_RED: Color = Color::DarkRed;
    pub const CYAN: Color = Color::Cyan;
    pub const DARK_GREEN: Color = Color::DarkGreen;
    pub const DARK_GREY: Color = Color::DarkGrey;
}

pub mod messages {
    use crate::colors;

    use crossterm::style::Stylize;

    pub fn show_start() {
        println!("┌───────────────────────────────────────────────────────┐"); // 57
        println!(
            "│    {} {} {} {}    │",
            "Добро пожаловать в игру".with(colors::GREY),
            "Minesweeper💣".bold().with(colors::YELLOW),
            "in".with(colors::GREY),
            "Rust🦀".bold().with(colors::RUST_ORANGE)
        );
        println!(
            "│         {} {} {} {} {} {}         │",
            "Ваша задача".with(colors::GREY),
            "найти".with(colors::GREEN),
            "и".with(colors::GREY),
            "пометить".with(colors::GREEN),
            "все".with(colors::GREY),
            "мины".with(colors::RUST_ORANGE)
        );
        println!(
            "│       {} {}      │",
            "Вы можете открывать поля вводя команду -".with(colors::GREY),
            "S".with(colors::YELLOW)
        );
        println!(
            "│     {} {}     │",
            "Помечать как подозрительную вводя команду -".with(colors::GREY),
            "?".with(colors::YELLOW)
        );
        println!(
            "│      {} {}     │",
            "Или помечать поле как мину вводя команду -".with(colors::GREY),
            "F".with(colors::YELLOW)
        );
        println!(
            "│  {}  │",
            "Для указания координаты нужно указать букву и цифру".with(colors::GREY),
        );
        println!(
            "│               {} {}               │",
            "В порядке:".with(colors::GREY),
            "Строка Столбец".with(colors::GREEN)
        );
        println!(
            "│                     {} {} {}                     │",
            "Пример:".with(colors::GREY),
            "1 a".with(colors::GREEN),
            "s".with(colors::YELLOW)
        );
        println!(
            "│                         {}                        │",
            "Удачи!".with(colors::RUST_ORANGE)
        );
        println!("└───────────────────────────────────────────────────────┘");
    }
}

pub mod game {
    use crate::{
        draw, messages,
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
        messages::show_start();

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
}

pub mod draw {
    use crate::{
        colors,
        game::{Cell, State},
        utils::num_to_str_cord_name,
    };

    use crossterm::style::Stylize;

    impl Cell {
        fn get_icon(self) -> String {
            match self {
                Cell::Empty => String::from("#").with(colors::GREY).to_string(),
                Cell::Number(x) => match x {
                    1 => String::from("1").with(colors::BLUE).to_string(),
                    2 => String::from("2").with(colors::GREEN).to_string(),
                    3 => String::from("3").with(colors::RED).to_string(),
                    4 => String::from("4").with(colors::DARK_BLUE).to_string(),
                    5 => String::from("5").with(colors::DARK_RED).to_string(),
                    6 => String::from("6").with(colors::CYAN).to_string(),
                    7 => String::from("7").with(colors::DARK_GREEN).to_string(),
                    8 => String::from("8").with(colors::DARK_GREY).to_string(),
                    _ => String::from("~"),
                },
                Cell::Mine => String::from("X").bold().with(colors::RED).to_string(),
                Cell::DeactivatedMine => String::from("V").bold().with(colors::GREEN).to_string(),
                Cell::Closed => String::from(" "),
                Cell::Question => String::from("?").bold().with(colors::GREEN).to_string(),
                Cell::Mark => String::from("F").bold().with(colors::YELLOW).to_string(),
                _ => panic!("Invalid cell type: {:?}", self),
            }
        }
    }

    pub fn draw(state: &State) {
        if !state.is_play {
            return;
        }

        let v_col_max_length = state.field_height.to_string().chars().count();
        let h_col_max_length = {
            let mut c: usize = 0;
            let mut d: usize = state.field_width - 1;
            while d != 0 {
                d /= 26;
                c += 1;
            }
            c
        };

        // Row headers padding
        print!("{:width$} ", " ", width = v_col_max_length);
        // Draw columns header
        println!(
            " {}",
            (0..state.field_width)
                .into_iter()
                .map(|x| {
                    let mut v = num_to_str_cord_name(x);
                    for _ in 0..h_col_max_length - v.len() {
                        v.push(' ')
                    }
                    v
                })
                .collect::<Vec<String>>()
                .join(" ")
        );
        // Draw top border
        print!("{:width$}┏ ", " ", width = v_col_max_length);
        println!(
            "{}┓",
            std::iter::repeat(format!("━{:<width$}", " ", width = h_col_max_length))
                .take(state.field_width)
                .collect::<String>()
        );

        // Draw row num + left border + cells + right border
        for i in 0..state.field_height {
            print!("{:width$}┃ ", i, width = v_col_max_length);
            for j in 0..state.field_width {
                let mask_field = state.mask_field[i][j].clone();
                if matches!(mask_field, Cell::Closed | Cell::Question | Cell::Mark) {
                    print!("{}{}", mask_field.get_icon(), " ".repeat(h_col_max_length));
                    continue;
                }
                let under_field = state.under_field[i][j].clone();
                print!("{}{}", under_field.get_icon(), " ".repeat(h_col_max_length));
            }
            println!("┃")
        }

        // Draw bottom border
        print!("{:width$}┗ ", " ", width = v_col_max_length);
        println!(
            "{}┛",
            std::iter::repeat(format!("━{:<width$}", " ", width = h_col_max_length))
                .take(state.field_width)
                .collect::<String>()
        );
    }
}

pub mod utils {
    use crate::draw::draw;
    use crate::game::State;

    pub fn num_to_str_cord_name(num: usize) -> String {
        let mut v = Vec::<String>::new();
        let mut d: usize = num;
        loop {
            v.push(
                char::from_u32((('a' as usize) + (d % 26)) as u32)
                    .unwrap()
                    .to_string(),
            );
            d /= 26;
            if d == 0 {
                break;
            }
        }
        v.reverse();
        v.join("")
    }

    pub fn str_cord_name_to_num(str: String) -> Result<usize, ()> {
        let count = str.chars().count();
        let mut num = 0;

        for (idx, letter) in str.chars().enumerate().into_iter() {
            if !letter.is_ascii_alphabetic() {
                return Err(());
            }

            let mut p = (letter as u32 - 'a' as u32) as usize;
            let rank = count - idx - 1;

            if rank != 0 {
                p += 1
            }

            num += p * usize::pow(26, rank as u32)
        }

        return Ok(num);
    }

    pub fn clear() {
        print!("{esc}c", esc = 27 as char)
    }

    pub fn flip(state: &State) {
        clear();
        draw(&state);
    }
}

fn main() {
    ctrlc::set_handler(move || {
        utils::clear();
        std::process::exit(-1);
    })
    .expect("Error setting Ctrl-C handler");

    game::run();
}
