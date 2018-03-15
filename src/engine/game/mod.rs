extern crate sfml;

use sfml::graphics::RenderWindow;


mod game_state;
mod logic;
mod render;

mod menu;

pub fn start(window: &mut RenderWindow) {
    println!("First GAME INIT", );

}

pub fn logic_run(window: &mut RenderWindow) {
    logic::run(window);
}

pub fn render_run(window: &mut RenderWindow) {
    render::run(window);
}

pub fn menu_run(window: &mut RenderWindow) {
    menu::run(window);
}