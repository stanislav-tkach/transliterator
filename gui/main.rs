extern crate translator;

extern crate piston_window;
#[macro_use]
extern crate conrod;
extern crate find_folder;

use translator::Translator;

use piston_window::{PistonWindow, WindowSettings, EventLoop, Glyphs, UpdateEvent, G2d};
use conrod::{Theme, Canvas, TextBox, Widget, Sizeable, Positionable, Graphics};
use conrod::color::{self, Colorable};

type Backend = (<G2d<'static> as Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Conrod", [800, 600])
                                       .exit_on_esc(true)
                                       .vsync(true)
                                       .build()
                                       .unwrap();

    let mut ui = {
        let assets_path = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets_path.join("NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    let mut original_text = "".to_owned();
    let mut translated_text = "".to_owned();

    let translator = Translator::new();

    window.set_ups(60);
    while let Some(event) = window.next() {
        ui.handle_event(&event);

        event.update(|_| {
            ui.set_widgets(|mut ui| {
                let background_color = color::rgb(0.2, 0.35, 0.45);

                // TODO: Use frame width for text boxes.
                Canvas::new()
                    .pad(30.)
                    .color(background_color)
                    .scroll_kids()
                    .set(CANVAS, &mut ui);

                TextBox::new(&mut original_text)
                    .font_size(20)
                    .w_h(420.0, 40.0)
                    .mid_left_of(CANVAS)
                    .color(background_color.invert())
                    .react(|_string: &mut String| {})
                    .set(INPUT_TEXT, &mut ui);

                TextBox::new(&mut translated_text)
                    .font_size(20)
                    .w_h(420.0, 40.0)
                    .down_from(INPUT_TEXT, 45.0)
                    .color(background_color.invert())
                    .react(|_string: &mut String| {})
                    .set(OUTPUT_TEXT, &mut ui);
            })
        });

        window.draw_2d(&event,
                       |context, graphic| ui.draw_if_changed(context, graphic));

        translated_text = translator.translate(&original_text);
    }
}

widget_ids! {
    CANVAS,
    INPUT_TEXT,
    OUTPUT_TEXT,
}
