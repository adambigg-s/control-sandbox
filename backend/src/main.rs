mod simulation;
mod runge_kutta_four;

use simulation::{Control, Simulation, State};

unsafe extern "C" {
    fn controller(state: &State) -> Control;
}

fn call_controller(state: &State) -> Control {
    unsafe { controller(state) }
}

fn main() {
    let mut simulation = Simulation {
        state: State {
            position: 1300.,
            delta_time: 0.01,
            ..Default::default()
        },
        ..Default::default()
    };
    loop {
        simulation.control = call_controller(&simulation.state);

        simulation.step();

        print!("{:.2} ", simulation.control.force);
        println!("state: {:.2?}", simulation.state);

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
