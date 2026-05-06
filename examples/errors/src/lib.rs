use ferryx_macros::ferryx;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathError {
    #[error("division by zero")]
    DivisionByZero,
}

#[ferryx]
pub struct Calculator;

#[ferryx]
impl Calculator {
    pub fn divide(&self, left: f32, right: f32) -> Result<f32, MathError> {
        if right == 0.0 {
            return Err(MathError::DivisionByZero);
        }
        Ok(left / right)
    }
}

