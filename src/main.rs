use log::info;

use ray_tracing_in_one_weekend::color;

fn main() {
    env_logger::init();
    // Image
    let image_height = 256;
    let image_width = 256;

    // Render
    println!("P3\n{} {}\n255", image_height, image_width);
    for j in 0..image_height {
        info!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = color::Color::new(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0.0,
            );
            color::write_color(pixel_color);
        }
    }
    info!(" \rDone.                 \n");
}
