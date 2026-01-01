use Odysseus::engine::{start, Game, Screen};

fn main() {
    let game = Game { screen: Screen::Title };
    start(game);
}
