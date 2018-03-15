extern crate sfml;
use engine::sfml::graphics::RenderWindow;
use engine::sfml::window::Key;
use engine::sfml::window::mouse;
use sfml::audio::{SoundBuffer, Sound, SoundStatus};

use sfml::system::{sleep, Time};
use std;
use std::io::Write;

fn player_input() {
    if Key::Right.is_pressed() {
        // sprite.move_((0.1, 0.0));
    }
    if mouse::Button::Left.is_pressed() {
        println!("Left button!", );

    }
}

pub fn run(window: &mut RenderWindow) {

    player_input();

}