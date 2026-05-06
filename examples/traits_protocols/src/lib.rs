use ferryx_macros::ferryx;

pub trait Scorable {
    fn score(&self) -> f32;
}

#[ferryx]
pub struct ModelOutput {
    pub confidence: f32,
}

impl Scorable for ModelOutput {
    fn score(&self) -> f32 {
        self.confidence
    }
}

#[ferryx]
impl ModelOutput {
    pub fn calibrated(&self, factor: f32) -> ModelOutput {
        ModelOutput {
            confidence: self.confidence * factor,
        }
    }
}

