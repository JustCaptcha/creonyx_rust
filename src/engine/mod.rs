extern crate sfml;

use engine::sfml::graphics::*;
use engine::sfml::system::*;
use engine::sfml::window::*;

use sfml::audio::{Sound, SoundBuffer, SoundStatus};
use sfml::system::{Time, sleep};

mod game;


pub fn run() {
    let mut window = RenderWindow::new((800, 600),
                                        "Creonyx",
                                        Style::CLOSE,
                                        &Default::default());
    let mut sound_buffer = SoundBuffer::from_file("resources/plasterbrain-game-start.ogg").unwrap();
    let mut sound = Sound::with_buffer(&sound_buffer);
    sound.play();
    // sleep(Time::milliseconds(1000));

    game::menu_run(&mut window);

    // First game init.
    game::start(&mut window);
    
    // Game loop
    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                    | Event::KeyPressed {
                        code: Key::Escape, ..
                    } => return,
                    _ => {}
            }
    }

    game::logic_run(&mut window);
    game::render_run(&mut window);

    }
}