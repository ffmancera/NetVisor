use cairo::{Context, Format, ImageSurface};
use std::fs::File;

use crate::diag_ctxt::DiagramCtxt;

pub fn draw_picture(diag_ctx: DiagramCtxt, file: &String) {
    // TODO: the right size is hard to guess, make this dynamic somehow
    let surface = ImageSurface::create(Format::ARgb32, 1280, 720).expect("Couldn’t create surface");
    let context = Context::new(&surface).unwrap();

    // Set a white background
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint().unwrap();

    rectangle_with_border(
        &context,
        (190., 110.),
        (900., 500.),
        (0.9765625, 0.98046875, 0.58203125),
    );
    write_text_top_right_corner_rect(
        &context,
        (190., 110.),
        (900., 500.),
        25.,
        &diag_ctx.hostname,
    );

    let mut file = File::create(file).expect("Couldn’t create file");
    surface
        .write_to_png(&mut file)
        .expect("Couldn’t write to png");
}

fn rectangle_with_border(
    context: &Context,
    pos: (f64, f64),
    size: (f64, f64),
    color: (f64, f64, f64),
) {
    context.set_source_rgb(0., 0., 0.);
    context.rectangle(pos.0, pos.1, size.0, size.1);
    context.fill().unwrap();

    context.set_source_rgb(color.0, color.1, color.2);
    context.rectangle(pos.0 + 5., pos.1 + 5., size.0 - 10., size.1 - 10.);
    context.fill().unwrap();
}

fn write_text_top_right_corner_rect(
    context: &Context,
    rect_pos: (f64, f64),
    rect_size: (f64, f64),
    size: f64,
    text: &String,
) {
    context.set_font_size(size);
    context.set_source_rgb(0.0, 0.0, 0.0);
    let extents = context.text_extents(text).unwrap();

    context.move_to(
        rect_pos.0 + rect_size.0 - extents.x_advance() - 10.,
        rect_pos.1 - (extents.y_bearing()) + 10.,
    );
    context.show_text(text).unwrap();
}
