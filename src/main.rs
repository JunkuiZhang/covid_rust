mod game;
mod settings;


fn main() {
    let mut g = game::Game::new(600, "Hello");
    g.run();
}