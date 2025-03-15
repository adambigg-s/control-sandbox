use std::ops::{Add, Mul};

use crate::runge_kutta_four::Differentiable;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct State {
    pub position: f32,
    pub velocity: f32,
    pub tick: i32,
    pub delta_time: f32,
}

impl Differentiable for State {
    fn derivative(&self, control: &Control) -> State {
        State {
            position: self.velocity,
            velocity: control.force,
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
            ..Default::default()
        }
    }
}

impl Mul<f32> for State {
    type Output = State;
    
    fn mul(self, rhs: f32) -> Self::Output {
        State {
            position: self.position * rhs,
            velocity: self.velocity * rhs,
            ..Default::default()
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
        let k1 = self.state.derivative(&self.control);
        let state_k2 = self.state + k1 * (self.state.delta_time / 2.);
        let k2 = state_k2.derivative(&self.control);
        let state_k3 = self.state + k2 * (self.state.delta_time / 2.);
        let k3 = state_k3.derivative(&self.control);
        let state_k4 = self.state + k3 * self.state.delta_time;
        let k4 = state_k4.derivative(&self.control);

        let derivative_sum = State {
            position: (k1.position + 2. * k2.position + 2. * k3.position + k4.position) / 6.,
            velocity: (k1.velocity + 2. * k2.velocity + 2. * k3.velocity + k4.velocity) / 6.,
            ..Default::default()
        };

        self.state.position += derivative_sum.position * self.state.delta_time;
        self.state.velocity += derivative_sum.velocity * self.state.delta_time;
        self.state.tick += 1;
    }
}
