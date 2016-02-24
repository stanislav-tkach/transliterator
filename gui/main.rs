extern crate translator;

extern crate piston;
extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};
use piston::event_loop::{Events, EventLoop};

fn main() {
    let window: PistonWindow = WindowSettings::new("Hello Conrod", [800, 600])
                                                  .exit_on_esc(true)
                                                  .vsync(true)
                                                  .build()
                                                  .unwrap();

    for event in window.ups(60) {
        self.ui.set_widgets(|ui| {
            Canvas::new()
                .pad(30.)
                .color(conrod::color::rgb(0.2, 0.35, 0.45))
                .scroll_kids()
                .set(CANVAS, ui);
    }
}
