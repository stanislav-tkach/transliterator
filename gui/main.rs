extern crate translator;

extern crate piston;
extern crate piston_window;
extern crate conrod;
extern crate find_folder;

use piston_window::{PistonWindow, WindowSettings, Glyphs};
use piston::event_loop::{Events, EventLoop};
use conrod::{Theme, Canvas};

fn main() {
    let window: PistonWindow = WindowSettings::new("Hello Conrod", [800, 600])
                                                  .exit_on_esc(true)
                                                  .vsync(true)
                                                  .build()
                                                  .unwrap();

    let ui = {
        let assets_path = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets_path.join("NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
        conrod::Ui::new(glyph_cache.unwrap(), theme)
    };

    for event in window.ups(60) {
        ui.set_widgets(|ui| {
            Canvas::new()
                .pad(30.)
                .color(conrod::color::rgb(0.2, 0.35, 0.45))
                .scroll_kids()
                .set(CANVAS, ui);
        });
    }
}
