use embedded_graphics::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, primitives::*, style::*, fonts::*, image::{Image, ImageRawLE}};
use embedded_graphics_simulator::*;

use wio_splash::WioSplash;

fn main () -> Result<(), core::convert::Infallible>{
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("draw a line", &output_settings);

    let raw = ImageRawLE::new(include_bytes!("./assets/ferris.raw"), 86, 64);
    let splash = WioSplash::new(Rgb565::GREEN, raw);
    splash.draw(&mut display)?;
    
    // let image = Image::new(&raw, Point::new(32, 32));
    // image.draw(&mut display)?;

    window.show_static(&display);

    Ok(())
}
