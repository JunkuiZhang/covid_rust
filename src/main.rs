use settings::WINDOW_WIDTH;

mod game;
mod settings;

fn main() {
    let mut g = game::Game::new(WINDOW_WIDTH, "Hello");
    g.run();
}
