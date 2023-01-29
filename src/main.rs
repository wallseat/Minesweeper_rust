mod color;
mod draw;
mod game;
mod message;
mod utils;

fn main() {
    ctrlc::set_handler(move || {
        utils::clear();
        std::process::exit(-1);
    })
    .expect("Error setting Ctrl-C handler");

    game::run();
}
