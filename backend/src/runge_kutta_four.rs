use crate::simulation::Control;

pub trait Differentiable {
    fn derivative(&self, other: &Control) -> Self;
}
