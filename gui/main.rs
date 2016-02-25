extern crate translator;

extern crate piston;
extern crate piston_window;
#[macro_use]
extern crate conrod;
extern crate find_folder;

use piston_window::{PistonWindow, WindowSettings, Glyphs, UpdateEvent};
use piston::event_loop::EventLoop;
use conrod::{Theme, Ui, Canvas, TextBox, Widget, Sizeable, Positionable};
use conrod::color::{self, Colorable};

fn main() {
    let window: PistonWindow = WindowSettings::new("Hello Conrod", [800, 600])
                                   .exit_on_esc(true)
                                   .vsync(true)
                                   .build()
                                   .unwrap();

    let mut ui = {
        let assets_path = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets_path.join("NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    for w in window.ups(60) {
        ui.handle_event(&w);

        w.update(|_| {
            ui.set_widgets(|ui| {
                let background_color = color::rgb(0.2, 0.35, 0.45);
                let mut text = "".to_owned();

                Canvas::new()
                    .pad(30.)
                    .color(background_color)
                    .scroll_kids()
                    .set(CANVAS, ui);

                TextBox::new(&mut text)
                    .font_size(20)
                    .w_h(420.0, 40.0)
                    .mid_left_of(CANVAS)
                    .color(background_color.invert())
                    .react(|_string: &mut String| {})
                    .set(INPUT_TEXT, ui);

                TextBox::new(&mut text)
                    .font_size(20)
                    .w_h(320.0, 40.0)
                    .down_from(INPUT_TEXT, 45.0)
                    .color(background_color.invert())
                    .react(|_string: &mut String| {})
                    .set(OUTPUT_TEXT, ui);
            })
        });

        w.draw_2d(|context, graphic| ui.draw_if_changed(context, graphic));
    }
}

widget_ids! {
    CANVAS,
    INPUT_TEXT,
    OUTPUT_TEXT,
}
