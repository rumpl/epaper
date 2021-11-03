use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::BinaryColor, prelude::*, primitives::Rectangle,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use embedded_text::{
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder, VerticalOverdraw},
    TextBox,
};
use ibm437::IBM437_8X8_NORMAL;

fn draw_text<D>(text: &str, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let text_style_normal_8_8 = MonoTextStyle::new(&IBM437_8X8_NORMAL, BinaryColor::Off);

    let textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::Exact(VerticalOverdraw::Visible))
        .alignment(HorizontalAlignment::Left)
        .paragraph_spacing(6)
        .build();
    let bounds = Rectangle::new(Point::new(5, 5), Size::new(390, 290));
    let text_box = TextBox::with_textbox_style(text, bounds, text_style_normal_8_8, textbox_style);

    display.clear(BinaryColor::On)?;
    text_box.draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(400, 300));
    let text = "This is some long line of text that embedded_text will wrap for me because embedded_* is a really nice library";

    draw_text(text, &mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Default)
        .scale(2)
        .build();
    let mut window = Window::new("Hello World", &output_settings);
    window.show_static(&display);

    Ok(())
}
