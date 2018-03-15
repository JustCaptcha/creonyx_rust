extern crate sfml;

use engine::sfml::graphics::*;
use engine::sfml::system::*;
use engine::sfml::window::*;
use engine::sfml::system::{sleep, Time};

pub fn run(window: &mut RenderWindow) {
    let mut v: View;
    // v.reset();
    let tiles = Texture::from_file("resources/block_1.png").unwrap();
    let mut sprite = Sprite::new();
    sprite.set_texture(&tiles, true);
    sprite.set_position((20.0, 28.0));
    sprite.set_scale((0.1, 0.1));

    window.clear(&Color::CYAN);
    window.draw(&sprite);
    window.display();
    sleep(Time::milliseconds(100));
    
}