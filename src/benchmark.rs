mod kernel;
mod model;
mod simulator;
mod util_3d;
use crate::simulator::Simulator;

fn main() {
    let mut sim = Simulator::setup();

    let mut step = 0.1;

    loop {
        sim.update();
        let t = sim.get_time();
        if t >= step {
            dbg!(t);
            step += 0.1;
        }
        if t >= 2. {
            break;
        }
    }
}
