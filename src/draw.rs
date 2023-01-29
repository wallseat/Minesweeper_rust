use crate::{
    color,
    game::{Cell, State},
    utils::num_to_str_cord_name,
};

use crossterm::style::Stylize;

impl Cell {
    fn get_icon(self) -> String {
        match self {
            Cell::Empty => String::from("#").with(color::GREY).to_string(),
            Cell::Number(x) => match x {
                1 => String::from("1").with(color::BLUE).to_string(),
                2 => String::from("2").with(color::GREEN).to_string(),
                3 => String::from("3").with(color::RED).to_string(),
                4 => String::from("4").with(color::DARK_BLUE).to_string(),
                5 => String::from("5").with(color::DARK_RED).to_string(),
                6 => String::from("6").with(color::CYAN).to_string(),
                7 => String::from("7").with(color::DARK_GREEN).to_string(),
                8 => String::from("8").with(color::DARK_GREY).to_string(),
                _ => String::from("~"),
            },
            Cell::Mine => String::from("X").bold().with(color::RED).to_string(),
            Cell::DeactivatedMine => String::from("V").bold().with(color::GREEN).to_string(),
            Cell::Closed => String::from(" "),
            Cell::Question => String::from("?").bold().with(color::GREEN).to_string(),
            Cell::Mark => String::from("F").bold().with(color::YELLOW).to_string(),
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
