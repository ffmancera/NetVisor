use cairo::{Context, Format, ImageSurface};
use std::fs::File;

pub fn draw_picture(file: &String) {
    let surface = ImageSurface::create(Format::ARgb32, 600, 600).expect("Couldn’t create surface");
    let context = Context::new(&surface).unwrap();
    context.set_source_rgb(1.0, 0.0, 0.0);
    context.paint();
    let mut file = File::create(file).expect("Couldn’t create file");
    surface
        .write_to_png(&mut file)
        .expect("Couldn’t write to png");
}
