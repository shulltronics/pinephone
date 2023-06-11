use rand::random;

use embedded_graphics::{
    prelude::*,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::{BinaryColor, Bgr888},
    text::{Alignment, Text, LineHeight, TextStyleBuilder},
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use profont::PROFONT_24_POINT;

use framebuffer_display::{FramebufferDisplay, Orientation};

fn main() {
    println!("Hello from Rust on PinePhone!\n");

    let mut display = FramebufferDisplay::new();

    display.set_orientation(Orientation::LANDSCAPE);
    println!("Display orientation: {:?}\n", display.get_orientation());
    println!("Display size: {:?}\n", display.get_size());

    display.clear();

    // Draw a border:
    let (w, h): (u32, u32) = display.get_size();
    let border = Rectangle::new(Point::new(0, 0), Size::new(w-1, h-1))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(1)
                .stroke_color(Bgr888::GREEN)
                .build());
    border.draw(&mut display).unwrap();

    // Draw a square:
    let square = Rectangle::new(Point::new(10, 10), Size::new(50, 50))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Bgr888::BLUE)
                .build());
    square.draw(&mut display).unwrap();

    let mut char_style = MonoTextStyle::new(&PROFONT_24_POINT, Bgr888::new(255, 255, 255));
    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .line_height(LineHeight::Percent(200))
        .build();

    let mut i: u8 = 0;
    while true {
        char_style.text_color = Some(wheel(i));
        let mut text = Text::with_text_style("Hello, Pinephone!\nFrom Rust", Point::new(75, 75), char_style, text_style);
        text.draw(&mut display).unwrap();
        i = i.wrapping_add(1);
    }

}

fn wheel(mut w: u8) -> Bgr888 {
    if w < 85 {
        return Bgr888::new(255u8 - w*3, w*3, 0);
    } else if w < 170 {
        w -= 85;
        return Bgr888::new(0, 255u8 - w*3, w*3);
    } else {
        w -= 170;
        return Bgr888::new(w*3, 0, 255u8 - w*3);
    }
}
