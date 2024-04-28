mod kernel;
mod model;
mod render;
mod simulator;
mod util_3d;

use render::Render;
use simulator::Simulator;
use util_3d::*;

#[macroquad::main("simulation")]
async fn main() {
    let mut sim = Simulator::setup();

    let mut render = Render::new();
    let frame_period = ((1. / 2.) * 1000.) as u128;
    let mut next_render = std::time::Instant::now();

    loop {
        sim.update();
        if next_render.elapsed().as_millis() >= frame_period {
            dbg!(sim.get_time());
            let space = sim.get_space();
            let display_distance = sim.get_display_distance();
            next_render = render
                .render_distance_from_zero(space, display_distance)
                .await;
        }
    }
}
