use std::ops::{Add, Mul};

pub trait Differentiable {
    fn derivative(&self, other: &Control) -> Self;
}

pub struct RungeKutta4 {
    integrand: State,
    delta_time: f32,
}

impl RungeKutta4 {
    fn build(state: State, delta_time: f32) -> RungeKutta4 {
        RungeKutta4 {
            integrand: state,
            delta_time,
        }
    }

    fn step(&mut self, impulse: &Control) {
        let k1 = self.integrand.derivative(impulse);
        let k2 = (self.integrand + k1 * (self.delta_time / 2.)).derivative(impulse);
        let k3 = (self.integrand + k2 * (self.delta_time / 2.)).derivative(impulse);
        let k4 = (self.integrand + k3 * self.delta_time).derivative(impulse);

        let increment = k1 * (self.delta_time / 6.)
            + (k2 + k3) * (self.delta_time / 3.)
            + k4 * (self.delta_time / 6.);

        self.integrand = self.integrand + increment;
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct State {
    pub position: f32,
    pub velocity: f32,
    pub acceleration: f32,
    pub tick: i32,
    pub delta_time: f32,
}

impl Differentiable for State {
    fn derivative(&self, control: &Control) -> State {
        State {
            position: self.velocity,
            velocity: self.acceleration,
            acceleration: control.force,
            tick: self.tick,
            delta_time: self.delta_time,
        }
    }
}

impl Add for State {
    type Output = State;

    fn add(self, rhs: State) -> Self::Output {
        State {
            position: self.position + rhs.position,
            velocity: self.velocity + rhs.velocity,
            acceleration: self.acceleration + rhs.acceleration,
            tick: self.tick,
            delta_time: self.delta_time,
        }
    }
}

impl Mul<f32> for State {
    type Output = State;

    fn mul(self, rhs: f32) -> Self::Output {
        State {
            position: self.position * rhs,
            velocity: self.velocity * rhs,
            acceleration: self.acceleration * rhs,
            tick: self.tick,
            delta_time: self.delta_time,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct Control {
    pub force: f32,
}

#[derive(Default, Debug)]
pub struct Simulation {
    pub state: State,
    pub control: Control,
}

impl Simulation {
    pub fn step(&mut self) {
        let mut rk4 = RungeKutta4::build(self.state, self.state.delta_time);
        rk4.step(&self.control);
        self.state = rk4.integrand;
        self.state.tick += 1;
    }
}
