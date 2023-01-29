use crate::color;

use crossterm::style::Stylize;

pub fn show_start() {
    println!("┌───────────────────────────────────────────────────────┐"); // 57
    println!(
        "│    {} {} {} {}    │",
        "Добро пожаловать в игру".with(color::GREY),
        "Minesweeper💣".bold().with(color::YELLOW),
        "in".with(color::GREY),
        "Rust🦀".bold().with(color::RUST_ORANGE)
    );
    println!(
        "│         {} {} {} {} {} {}         │",
        "Ваша задача".with(color::GREY),
        "найти".with(color::GREEN),
        "и".with(color::GREY),
        "пометить".with(color::GREEN),
        "все".with(color::GREY),
        "мины".with(color::RUST_ORANGE)
    );
    println!(
        "│       {} {}      │",
        "Вы можете открывать поля вводя команду -".with(color::GREY),
        "S".with(color::YELLOW)
    );
    println!(
        "│     {} {}     │",
        "Помечать как подозрительную вводя команду -".with(color::GREY),
        "?".with(color::YELLOW)
    );
    println!(
        "│      {} {}     │",
        "Или помечать поле как мину вводя команду -".with(color::GREY),
        "F".with(color::YELLOW)
    );
    println!(
        "│  {}  │",
        "Для указания координаты нужно указать букву и цифру".with(color::GREY),
    );
    println!(
        "│               {} {}               │",
        "В порядке:".with(color::GREY),
        "Строка Столбец".with(color::GREEN)
    );
    println!(
        "│                     {} {} {}                     │",
        "Пример:".with(color::GREY),
        "1 a".with(color::GREEN),
        "s".with(color::YELLOW)
    );
    println!(
        "│                         {}                        │",
        "Удачи!".with(color::RUST_ORANGE)
    );
    println!("└───────────────────────────────────────────────────────┘");
}
