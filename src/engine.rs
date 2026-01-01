use crate::maps::{MAP, WORLD};

pub struct Game {
    pub screen: Screen,
}

pub enum Screen {
    Title,
    Map,
    World
}

pub fn start(game: Game) {
    let mut game = Game { screen: Screen::Map };
    draw(&game);
}

pub fn draw(game: &Game) {
    match game.screen {
        Screen::Title => {},
        Screen::Map => print!("{}", MAP),
        Screen::World => {},
    }
}
