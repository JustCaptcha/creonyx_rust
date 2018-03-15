extern crate sfml;

use engine::sfml::graphics::*;
use engine::sfml::system::*;
use engine::sfml::window::*;

pub fn run(window: &mut RenderWindow) {
    println!("Menu Section!", );
    let mut font = Font::from_file("resources/orange_kid.ttf").unwrap();
    let mut hello_world = Text::new("Menu here, prees backspace!", &font, 40);
    window.clear(&Color::BLUE);
    window.draw(&hello_world);
    window.display();

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                    | Event::KeyPressed {
                        code: Key::BackSpace, ..
                    } => return,
                    _ => {}
            }
        }
    }
}