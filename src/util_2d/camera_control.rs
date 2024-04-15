use macroquad::{
    camera::Camera2D,
    window::{screen_height, screen_width},
};

// adjust camera zoom according to input
// display_unit: +/-

pub fn get_camera(display_unit: f32) -> Camera2D {
    let pixel_per_unit: f32 = screen_height().min(screen_width()) / display_unit;
    let camera = Camera2D {
        zoom: macroquad::math::vec2(
            pixel_per_unit / screen_width(),
            -pixel_per_unit / screen_height(),
        ),
        ..Default::default()
    };
    camera
}
