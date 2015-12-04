extern crate translator;

extern crate piston;
extern crate piston_window;
extern crate glutin_window;

use piston::event_loop::{Events, EventLoop};

fn main() {
    let window: glutin_window::GlutinWindow = piston_window::WindowSettings::new("Hello Conrod", [800, 600])
//        .opengl(opengl)
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    let event_iter = window.events().ups(60).max_fps(60);
    for _event in event_iter {
    }
}
