use cairo::{Context, Format, ImageSurface};
use std::fs::File;

use crate::diag_ctxt::{DiagramCtxt, IfaceCtxt};

pub fn draw_picture(diag_ctx: DiagramCtxt, file: &String) {
    // TODO: the right size is hard to guess, make this dynamic somehow
    let initial_size = calculate_size(&diag_ctx);
    let surface = ImageSurface::create(
        Format::ARgb32,
        (initial_size.0 as i32) + 400,
        (initial_size.1 as i32) + 200,
    )
    .expect("Couldn’t create surface");
    let context = Context::new(&surface).unwrap();

    // Set a white background
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint().unwrap();

    // Draw initial box representing the host
    let initial_pos = (200., 100.);
    rectangle_with_border(
        &context,
        initial_pos,
        initial_size,
        (0.9765625, 0.98046875, 0.58203125),
    );
    write_text_top_right_corner_rect(&context, initial_pos, initial_size, 25., &diag_ctx.hostname);

    // TODO: draw all the interfaces, this is just some initial work
    let mut count: u32 = 0;
    for iface in diag_ctx.clone().ifaces {
        draw_iface_rect(
            &context,
            &iface,
            initial_pos,
            initial_size,
            diag_ctx.clone().count_ifaces_for_depth(1) as u32,
            count,
        );
        count = count + 1;
    }

    let mut file = File::create(file).expect("Couldn’t create file");
    surface
        .write_to_png(&mut file)
        .expect("Couldn’t write to png");
}

fn draw_iface_rect(
    context: &Context,
    iface_ctx: &IfaceCtxt,
    rect_pos: (f64, f64),
    rect_size: (f64, f64),
    total: u32,
    count: u32,
) {
    let iface_width = 300.;
    let iface_height = 100.;
    let pos = (
        rect_pos.0
            + ((rect_size.0 / total as f64) / 2. - iface_width / 2.)
            + ((count as f64) * rect_size.0 / (total as f64)),
        rect_pos.1 + (rect_size.1 / ((iface_ctx.depth as f64) + 1.) - (iface_height / 2.)),
    );
    rectangle_with_border(
        &context,
        pos,
        (iface_width, iface_height),
        (0.5703125, 0.7734375, 1.),
    );
    write_text_center_rect(
        &context,
        pos,
        (iface_width, iface_height),
        25.,
        &iface_ctx.iface.name,
    );
}

fn calculate_size(diag_ctx: &DiagramCtxt) -> (f64, f64) {
    let mut size = (300., 300.);
    for _ in diag_ctx.ifaces.clone() {
        size.0 = size.0 + 300.;
    }
    size
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

fn write_text_center_rect(
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
        rect_pos.0 + (rect_size.0 - extents.x_advance()) / 2.,
        rect_pos.1 + (rect_size.1 - extents.y_bearing()) / 2.,
    );
    context.show_text(text).unwrap();
}
