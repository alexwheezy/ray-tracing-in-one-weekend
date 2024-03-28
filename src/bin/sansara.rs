use ray_tracing_in_one_weekend::{camera, generate_scene, render, vec3::Point3};

fn main() {
    env_logger::init();

    // World
    let world = generate_scene::random_sphere();

    // Image
    // Ratio of image width over height
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    // Rendered image width in pixel count
    const IMAGE_WIDTH: i32 = 1280;
    // Count of random samples for each pixel
    const SAMPLE_PER_PIXEL: u32 = 500;
    // Maximum number of ray bounces into scene
    const MAX_DEPTH: u32 = 50;

    const VFOV: f32 = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_distance = 10.0;

    // Camera
    let image = camera::Image::with_width(IMAGE_WIDTH);
    let camera_settings =
        camera::CameraSettings::new(ASPECT_RATIO, VFOV, defocus_angle, focus_distance);
    let transform = camera::Xform::new(look_from, look_at, vup);
    let camera = camera::Camera::new(image, transform, camera_settings);

    // Render
    let render_settings = render::RenderSettings::new(SAMPLE_PER_PIXEL, MAX_DEPTH);
    let mut tracing = render::Render::new(render_settings, camera);
    tracing.render(&world);
}
