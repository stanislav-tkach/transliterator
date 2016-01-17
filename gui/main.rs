extern crate translator;

extern crate piston;
extern crate piston_window;

use piston::event_loop::{Events, EventLoop};

fn main() {
    let window: piston_window::PistonWindow = piston_window::WindowSettings::new("Hello Conrod", [800, 600])
                                                  .exit_on_esc(true)
                                                  .vsync(true)
                                                  .build()
                                                  .unwrap();

    for event in window.ups(60) {
    }
}
